use crate::common::steps::{bool_at_step, Step};
use crate::parsers::steps::parse_bool_steps;
use crate::pyinterface::resources::Resources;
use imagesize::size;
use itertools::Itertools;
use pyo3::exceptions::{PyException, PyValueError};
use pyo3::types::{PyAnyMethods, PyList};
use pyo3::{
    pyclass, pyfunction, pymethods, Bound, FromPyObject, IntoPyObject, PyAny, PyResult, Python,
};
use renderer::{InMemoryBinImage, InMemorySvgImage};
use resvg::usvg::{roxmltree, Error};
use std::collections::{BTreeMap, BTreeSet, HashMap};
use std::sync::Arc;
use xmltree::XMLNode;

#[pyclass(frozen)]
pub(crate) struct LoadedImage {
    pub(crate) width: f32,
    pub(crate) height: f32,
    pub(crate) image_data: LoadedImageData,
    pub(crate) named_steps: Vec<Step>,
}

#[pymethods]
impl LoadedImage {
    fn named_steps<'py>(&self, py: Python<'py>) -> PyResult<Vec<Bound<'py, PyAny>>> {
        self.named_steps
            .iter()
            .map(|s| s.into_pyobject(py))
            .collect::<Result<Vec<_>, _>>()
    }
    fn at_step<'py>(&self, step: Step) -> PyResult<PyImage> {
        match &self.image_data {
            LoadedImageData::BinImage(data) => Ok(PyImage {
                width: self.width,
                height: self.height,
                image_data: PyImageData::BinImage(data.clone()),
            }),
            LoadedImageData::SvgImage(data) => Ok(PyImage {
                width: self.width,
                height: self.height,
                image_data: PyImageData::SvgImage(data.clone()),
            }),
            LoadedImageData::FragmentedSvgImage(layers) => Ok(PyImage {
                width: self.width,
                height: self.height,
                image_data: PyImageData::FragmentedSvgImage(
                    layers
                        .iter()
                        .filter_map(|layer| {
                            layer
                                .steps
                                .as_ref()
                                .is_none_or(|steps| bool_at_step(&steps, &step))
                                .then(|| layer.data.clone())
                        })
                        .collect(),
                ),
            }),
        }
    }
}

#[pyclass(frozen)]
pub(crate) struct PyImage {
    pub(crate) width: f32,
    pub(crate) height: f32,
    pub(crate) image_data: PyImageData,
}

struct SvgLayer {
    steps: Option<Vec<(Step, bool)>>,
    data: InMemorySvgImage,
}

pub enum LoadedImageData {
    BinImage(InMemoryBinImage),
    SvgImage(InMemorySvgImage),
    FragmentedSvgImage(Vec<SvgLayer>),
}

pub enum PyImageData {
    BinImage(InMemoryBinImage),
    SvgImage(InMemorySvgImage),
    FragmentedSvgImage(Vec<InMemorySvgImage>),
}

fn create_png(data: Vec<u8>) -> PyResult<LoadedImage> {
    let format = imagesize::image_type(data.as_slice())
        .map_err(|_| PyValueError::new_err("Invalid image format"))?;
    if imagesize::ImageType::Png != format {
        return Err(PyValueError::new_err("Data is not an PNG image"));
    }
    let size = imagesize::blob_size(data.as_slice())
        .map_err(|_| PyValueError::new_err("Invalid image format"))?;
    Ok(LoadedImage {
        width: size.width as f32,
        height: size.height as f32,
        image_data: LoadedImageData::BinImage(InMemoryBinImage::new_png(Arc::new(data))),
        named_steps: Vec::new(),
    })
}

fn create_jpeg(data: Vec<u8>) -> PyResult<LoadedImage> {
    let format = imagesize::image_type(data.as_slice())
        .map_err(|_| PyValueError::new_err("Invalid image format"))?;
    if imagesize::ImageType::Jpeg != format {
        return Err(PyValueError::new_err("Data is not an JPEG image"));
    }
    let size = imagesize::blob_size(data.as_slice())
        .map_err(|_| PyValueError::new_err("Invalid image format"))?;
    Ok(LoadedImage {
        width: size.width as f32,
        height: size.height as f32,
        image_data: LoadedImageData::BinImage(InMemoryBinImage::new_jpeg(Arc::new(data))),
        named_steps: Vec::new(),
    })
}

fn check_is_non_empty(element: &xmltree::Element) -> bool {
    match element.name.as_str() {
        "g" | "svg" => {}
        "namedview" | "defs" => return false,
        _ => return true,
    }
    element.children.iter().any(|node| match node {
        XMLNode::Element(e) => check_is_non_empty(e),
        XMLNode::Comment(_) => false,
        XMLNode::CData(_) | XMLNode::Text(_) | XMLNode::ProcessingInstruction(_, _) => true,
    })
}

fn create_svg(s: String, enable_steps: bool) -> PyResult<LoadedImage> {
    let xml_opt = roxmltree::ParsingOptions {
        allow_dtd: true,
        ..Default::default()
    };

    let doc = roxmltree::Document::parse_with_options(&s, xml_opt)
        .map_err(|_| PyException::new_err("Could not parse SVG as XML file"))?;
    let options = resvg::usvg::Options::default();
    let usvg_tree = resvg::usvg::Tree::from_xmltree(&doc, &options)
        .map_err(|_| PyException::new_err("Could not parse SVG file"))?;
    let size = usvg_tree.size();
    let mut tree = xmltree::Element::parse(s.as_bytes()).unwrap();

    let defs = tree
        .children
        .extract_if(.., |child| match child {
            XMLNode::Element(e) if e.name == "defs" => true,
            _ => false,
        })
        .next();
    // let mut steps = BTreeSet::new();
    // for node in doc.descendants() {
    //     if let Some(label) =
    //         node.attribute(("http://www.inkscape.org/namespaces/inkscape", "label"))
    //     {
    //         if let Some((_, named_steps)) = parse_steps_from_label(label) {
    //             steps.extend(named_steps);
    //         }
    //     }
    // }
    // let named_steps = steps.into_iter().collect_vec();

    let add = |mut tree: xmltree::Element,
               result: &mut Vec<_>,
               defs: &Option<XMLNode>,
               steps: Option<Vec<(Step, bool)>>| {
        if check_is_non_empty(&tree) {
            if let Some(defs) = defs {
                tree.children.insert(0, defs.clone());
            }
            result.push(SvgLayer {
                steps,
                data: InMemorySvgImage::new(Arc::new(tree)),
            })
        }
    };

    if enable_steps {
        let mut result = Vec::new();
        let mut named_steps = BTreeSet::new();
        loop {
            if let Some(cut) = split_tree(&mut tree)? {
                add(cut.before, &mut result, &defs, None);
                add(cut.stepped_tree, &mut result, &defs, Some(cut.steps));
                named_steps.extend(cut.named_steps);
            } else {
                add(tree, &mut result, &defs, None);
                break;
            }
        }
        Ok(LoadedImage {
            width: size.width(),
            height: size.height(),
            image_data: LoadedImageData::FragmentedSvgImage(result),
            named_steps: named_steps.into_iter().collect_vec(),
        })
    } else {
        Ok(LoadedImage {
            width: size.width(),
            height: size.height(),
            image_data: LoadedImageData::SvgImage(InMemorySvgImage::new(Arc::new(tree))),
            named_steps: Vec::new(),
        })
    }
}

#[pyfunction]
pub(crate) fn create_mem_image<'py>(
    data: &Bound<'py, PyAny>,
    image_format: PyImageFormat,
    enable_steps: bool,
) -> PyResult<LoadedImage> {
    match image_format {
        PyImageFormat::Png => {
            let data: Vec<u8> = data.extract()?;
            create_png(data)
        }
        PyImageFormat::Jpeg => {
            let data: Vec<u8> = data.extract()?;
            create_jpeg(data)
        }
        PyImageFormat::Ora => {
            todo!()
        }
        PyImageFormat::Svg => {
            let s: String = data.extract()?;
            create_svg(s, enable_steps)
        }
    }
}

#[pyfunction]
pub(crate) fn load_image<'py>(path: &str, enable_steps: bool) -> PyResult<LoadedImage> {
    let image_format = if let Some(ext) = path.rsplit_once('.').map(|(_, s)| s.to_ascii_lowercase())
    {
        match ext.as_str() {
            "png" => PyImageFormat::Png,
            "jpg" | "jpeg" => PyImageFormat::Jpeg,
            "ora" => PyImageFormat::Ora,
            "svg" => PyImageFormat::Svg,
            _ => return Err(PyException::new_err(format!("Unknown file format: {path}"))),
        }
    } else {
        return Err(PyException::new_err(format!("Unknown file format: {path}")));
    };
    let data = std::fs::read(path)
        .map_err(|_| PyException::new_err(format!("Could not read image file: {path}")))?;
    match image_format {
        PyImageFormat::Png => create_png(data),
        PyImageFormat::Jpeg => create_jpeg(data),
        PyImageFormat::Ora => {
            todo!()
        }
        PyImageFormat::Svg => {
            let s = String::from_utf8(data).map_err(|_| {
                PyException::new_err(format!("File cannot be parsed as UTF-8: {path}"))
            })?;
            create_svg(s, enable_steps)
        }
    }
}

// #[derive(FromPyObject)]
// pub(crate) struct PyPathImage {
//     pub path: String,
// }
//
// #[derive(FromPyObject)]
// pub(crate) struct PyMemImage {
//     pub data_id: usize,
//     pub format: PyImageFormat,
// }

pub(crate) enum PyImageFormat {
    Png,
    Jpeg,
    Ora,
    Svg,
}

impl<'py> FromPyObject<'py> for PyImageFormat {
    fn extract_bound(ob: &Bound<'py, PyAny>) -> PyResult<Self> {
        let s: &str = ob.extract()?;
        Ok(match s {
            "png" => Self::Png,
            "jpeg" => Self::Jpeg,
            "ora" => Self::Ora,
            "svg" => Self::Svg,
            _ => return Err(PyValueError::new_err("Invalid file format")),
        })
    }
}

struct Cut {
    before: xmltree::Element,
    stepped_tree: xmltree::Element,
    steps: Vec<(Step, bool)>,
    named_steps: Vec<Step>,
}

fn split_tree(element: &mut xmltree::Element) -> crate::Result<Option<Cut>> {
    for (i, node) in element.children.iter_mut().enumerate() {
        match node {
            xmltree::XMLNode::Element(e) => {
                if let Some(step_def) = e
                    .attributes
                    .get("label")
                    .and_then(|v| v.rsplit_once("**").map(|x| x.1))
                {
                    let (steps, named_steps) = parse_bool_steps(step_def)?;
                    element.attributes.remove("label");
                    let mut children = std::mem::take(&mut element.children);
                    let mut e1 = element.clone();
                    let mut e2 = element.clone();
                    element.children = children.split_off(i + 1);
                    e2.children = vec![children.pop().unwrap()];
                    e1.children = children;
                    return Ok(Some(Cut {
                        before: e1,
                        stepped_tree: e2,
                        steps,
                        named_steps,
                    }));
                }
                if let Some(cut) = split_tree(e)? {
                    let mut children = std::mem::take(&mut element.children);
                    let mut e1 = element.clone();
                    let mut e2 = element.clone();
                    element.children = children.split_off(i + 1);
                    e2.children = vec![xmltree::XMLNode::Element(cut.stepped_tree)];
                    e1.children = children;
                    e1.children.push(xmltree::XMLNode::Element(cut.before));
                    return Ok(Some(Cut {
                        before: e1,
                        stepped_tree: e2,
                        steps: cut.steps,
                        named_steps: cut.named_steps,
                    }));
                }
            }
            _ => {}
        }
    }
    Ok(None)
}

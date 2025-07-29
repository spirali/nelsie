use crate::pyinterface::resources::Resources;
use imagesize::size;
use pyo3::exceptions::{PyException, PyValueError};
use pyo3::types::{PyAnyMethods, PyList};
use pyo3::{pyclass, pyfunction, Bound, FromPyObject, PyAny, PyResult, Python};
use renderer::{InMemoryBinImage, InMemorySvgImage};
use resvg::usvg::{roxmltree, Error};
use std::collections::HashMap;
use std::sync::Arc;

#[pyclass(frozen)]
pub(crate) struct PyImage {
    pub(crate) width: f32,
    pub(crate) height: f32,
    pub(crate) image_data: PyImageData,
}

pub enum PyImageData {
    BinImage(InMemoryBinImage),
    SvgImage(InMemorySvgImage),
}

#[pyfunction]
pub(crate) fn create_mem_image<'py>(
    data: &Bound<'py, PyAny>,
    image_format: PyImageFormat,
) -> PyResult<PyImage> {
    match image_format {
        PyImageFormat::Png => {
            let data: Vec<u8> = data.extract()?;
            let format = imagesize::image_type(data.as_slice())
                .map_err(|_| PyValueError::new_err("Invalid image format"))?;
            if imagesize::ImageType::Png != format {
                return Err(PyValueError::new_err("Data is not an PNG image"));
            }
            let size = imagesize::blob_size(data.as_slice())
                .map_err(|_| PyValueError::new_err("Invalid image format"))?;
            Ok(PyImage {
                width: size.width as f32,
                height: size.height as f32,
                image_data: PyImageData::BinImage(InMemoryBinImage::new_png(Arc::new(data))),
            })
        }
        PyImageFormat::Jpeg => {
            let data: Vec<u8> = data.extract()?;
            let format = imagesize::image_type(data.as_slice())
                .map_err(|_| PyValueError::new_err("Invalid image format"))?;
            if imagesize::ImageType::Jpeg != format {
                return Err(PyValueError::new_err("Data is not an Jpeg image"));
            }
            let size = imagesize::blob_size(data.as_slice())
                .map_err(|_| PyValueError::new_err("Invalid image format"))?;
            Ok(PyImage {
                width: size.width as f32,
                height: size.height as f32,
                image_data: PyImageData::BinImage(InMemoryBinImage::new_jpeg(Arc::new(data))),
            })
        }
        PyImageFormat::Ora => {
            todo!()
        }
        PyImageFormat::Svg => {
            let s: String = data.extract()?;

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
            let tree = xmltree::Element::parse(s.as_bytes()).unwrap();
            Ok(PyImage {
                width: size.width(),
                height: size.height(),
                image_data: PyImageData::SvgImage(InMemorySvgImage::new(Arc::new(tree))),
            })
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

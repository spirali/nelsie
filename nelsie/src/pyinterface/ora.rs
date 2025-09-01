use crate::common::steps::Step;
use crate::parsers::steps::parse_bool_steps;
use crate::pyinterface::image::{LoadedImage, LoadedImageData, OraLayer};
use imagesize::blob_size;
use renderer::{InMemoryBinImage, Rectangle};
use resvg::usvg::roxmltree;
use std::io::Read;
use std::sync::Arc;

fn read_archive_file_as_string<R: std::io::Seek + Read>(
    archive: &mut zip::ZipArchive<R>,
    filename: &str,
) -> zip::result::ZipResult<String> {
    Ok(std::io::read_to_string(archive.by_name(filename)?)?)
}

pub(crate) fn create_ora(data: Vec<u8>) -> crate::Result<LoadedImage> {
    let mut archive = zip::ZipArchive::new(std::io::Cursor::new(data))?;
    if read_archive_file_as_string(&mut archive, "mimetype")? != "image/openraster" {
        return Err(crate::Error::parsing_err("Not an ORA format"));
    }
    let stack_data = read_archive_file_as_string(&mut archive, "stack.xml")?;
    let stack_doc = roxmltree::Document::parse(&stack_data)
        .map_err(|e| crate::Error::parsing_err(format!("Invalid ORA stack.xml: {}", e)))?;
    let image = option_unpack(stack_doc.root().first_child())?;
    let width: f32 = image
        .attribute("w")
        .and_then(|v| str::parse(v).ok())
        .unwrap_or(0.0);
    let height: f32 = image
        .attribute("h")
        .and_then(|v| str::parse(v).ok())
        .unwrap_or(0.0);

    let mut layers = Vec::new();
    let mut named_steps = Vec::new();
    load_ora_stack(&image, &mut archive, &mut layers, &mut named_steps)?;
    layers.reverse();
    named_steps.sort_unstable();
    named_steps.dedup();
    Ok(LoadedImage {
        width,
        height,
        image_data: LoadedImageData::Ora(layers),
        named_steps,
    })
}

fn option_unpack<T>(value: Option<T>) -> crate::Result<T> {
    value.ok_or_else(|| crate::Error::Generic("Invalid format".to_string()))
}

fn load_ora_stack<R: std::io::Seek + Read>(
    node: &roxmltree::Node,
    archive: &mut zip::ZipArchive<R>,
    layers: &mut Vec<OraLayer>,
    named_steps: &mut Vec<Step>,
) -> crate::Result<()> {
    for child in node.children() {
        let tag = child.tag_name().name();
        if tag == "layer" {
            if child
                .attribute("visibility")
                .map(|v| v == "hidden")
                .unwrap_or(false)
            {
                continue;
            }
            //#let visibility =

            let steps = if let Some(step_def) = child
                .attribute("name")
                .and_then(|v| v.rsplit_once("**").map(|x| x.1))
            {
                let (steps, mut n_steps) = parse_bool_steps(step_def)?;
                named_steps.append(&mut n_steps);
                Some(steps)
            } else {
                None
            };

            let src = option_unpack(child.attribute("src"))?;
            let mut file = archive.by_name(src)?;
            let mut image_data = Vec::new();
            file.read_to_end(&mut image_data)?;
            let (width, height) = blob_size(&image_data)
                .map(|sz| (sz.width as f32, sz.height as f32))
                .unwrap_or((0.0, 0.0));
            layers.push(OraLayer {
                steps,
                rectangle: Rectangle::new(
                    child
                        .attribute("x")
                        .and_then(|v| str::parse(v).ok())
                        .unwrap_or(0.0),
                    child
                        .attribute("y")
                        .and_then(|v| str::parse(v).ok())
                        .unwrap_or(0.0),
                    width,
                    height,
                ),
                data: InMemoryBinImage::new_png(Arc::new(image_data)),
            });
        } else if tag == "stack" {
            load_ora_stack(&child, archive, layers, named_steps)?;
        }
    }
    Ok(())
}

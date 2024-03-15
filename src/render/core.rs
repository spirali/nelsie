use crate::model::{FontData, Resources, Slide, Step};
use crate::render::counters::CountersMap;
use crate::render::rendering::render_to_svg_tree;
use crate::render::OutputFormat;
use crate::NelsieError;
use crate::Result;
use resvg::tiny_skia;
use std::path::Path;
use std::sync::Arc;
use svg2pdf::usvg;
use svg2pdf::usvg::{PostProcessingSteps, TreePostProc};
use usvg::{TreeWriting, XmlOptions};

pub(crate) struct RenderConfig<'a> {
    pub resources: &'a Resources,
    pub slide: &'a Slide,
    pub slide_idx: usize,
    pub step: Step,
    pub default_font: &'a Arc<FontData>,
    pub output_format: OutputFormat,
    pub output_path: Option<&'a Path>,
    pub counter_values: &'a CountersMap<'a>,
}

pub(crate) enum RenderingResult {
    None,
    Tree(usvg::Tree),
    BytesData(Vec<u8>),
}

pub(crate) fn render_slide_step(render_cfg: &RenderConfig) -> Result<RenderingResult> {
    log::debug!("Rendering step {}", render_cfg.step);
    let mut tree = render_to_svg_tree(render_cfg);
    let postprocessing = PostProcessingSteps {
        convert_text_into_paths: true,
    };
    tree.postprocess(postprocessing, &render_cfg.resources.font_db);

    let result = match render_cfg.output_format {
        OutputFormat::Pdf => return Ok(RenderingResult::Tree(tree)),
        OutputFormat::Svg => {
            let svg = tree.to_string(&XmlOptions::default());
            RenderingResult::BytesData(svg.into_bytes())
        }
        OutputFormat::Png => {
            let zoom = 1.0;
            let pixmap_size = tree.size.to_int_size().scale_by(zoom).unwrap();
            let mut pixmap =
                tiny_skia::Pixmap::new(pixmap_size.width(), pixmap_size.height()).unwrap();
            let render_ts = tiny_skia::Transform::from_scale(zoom, zoom);
            resvg::render(&tree, render_ts, &mut pixmap.as_mut());
            RenderingResult::BytesData(
                pixmap
                    .encode_png()
                    .map_err(|e| NelsieError::Generic(e.to_string()))?,
            )
        }
    };

    if let Some(path) = render_cfg.output_path {
        if let RenderingResult::BytesData(data) = result {
            let final_path = path.join(format!(
                "{}-{}.{}",
                render_cfg.slide_idx,
                render_cfg.step,
                render_cfg.output_format.extension()
            ));
            std::fs::write(&final_path, data).map_err(|e| {
                NelsieError::Generic(format!(
                    "Cannot write output file: {}: {}",
                    final_path.display(),
                    e
                ))
            })?;
        }
        Ok(RenderingResult::None)
    } else {
        Ok(result)
    }
}

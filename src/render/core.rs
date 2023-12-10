use crate::model::{Resources, Slide, Step};
use crate::render::rendering::render_to_svg_tree;
use crate::NelsieError;
use crate::Result;
use resvg::tiny_skia;
use std::path::Path;
use std::sync::Arc;
use usvg::{TreeTextToPath, TreeWriting, XmlOptions};

pub(crate) struct RenderConfig<'a> {
    pub resources: &'a Resources,
    pub slide: &'a Slide,
    pub step: Step,
    pub default_font_name: &'a Arc<String>,

    pub output_svg: Option<&'a Path>,
    pub output_png: Option<&'a Path>,
}

pub(crate) fn render_slide_step(render_cfg: &RenderConfig) -> Result<usvg::Tree> {
    log::debug!("Rendering step {}", render_cfg.step);
    let mut tree = render_to_svg_tree(render_cfg);
    tree.convert_text(&render_cfg.resources.font_db);

    // Write SVG
    if let Some(output) = render_cfg.output_svg {
        let svg = tree.to_string(&XmlOptions::default());
        std::fs::write(output, svg).map_err(|e| {
            NelsieError::GenericError(format!(
                "Cannot write target SVG file: {}: {}",
                output.display(),
                e
            ))
        })?;
    }

    // Write PNG
    if let Some(output) = render_cfg.output_png {
        let zoom = 1.0;
        let rtree = resvg::Tree::from_usvg(&tree);
        let pixmap_size = rtree.size.to_int_size().scale_by(zoom).unwrap();
        let mut pixmap = tiny_skia::Pixmap::new(pixmap_size.width(), pixmap_size.height()).unwrap();
        let render_ts = tiny_skia::Transform::from_scale(zoom, zoom);
        rtree.render(render_ts, &mut pixmap.as_mut());
        pixmap.save_png(output).map_err(|e| {
            NelsieError::GenericError(format!(
                "Cannot write target PNG file: {}: {}",
                output.display(),
                e
            ))
        })?;
    }
    Ok(tree)
}

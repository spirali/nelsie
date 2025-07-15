use crate::node::{Node, NodeChild};
use crate::render::canvas::{Canvas, Link};
use crate::render::context::RenderContext;
use crate::render::layout::{ComputedLayout, compute_page_layout};
use crate::shapes::FillAndStroke;
use std::collections::BTreeSet;

pub(crate) fn render_node(
    render_ctx: &mut RenderContext,
    node: &Node,
    layout: &ComputedLayout,
    canvas: &mut Canvas,
) {
    if !node.show {
        return;
    }
    if let Some(color) = &node.bg_color {
        let rect = &layout.node_layout(node.node_id).unwrap().rect;
        let item = rect.draw_rounded(FillAndStroke::new_fill(*color), node.border_radius);
        canvas.add_draw_item(node.z_level, item);
    }

    if let Some(content_id) = &node.content {
        let rect = layout.node_layout(node.node_id).unwrap().rect.clone();
        canvas.add_content(node.z_level, rect, *content_id);
    }

    /*if let Some(content) = &node.content {
        let rect = layout.node_layout(node.node_id).unwrap().rect;
        match content {
            NodeContent::Text(_) => {
                self.canvas.add_text(
                    config.text_cache.get(node.node_id).unwrap().clone(),
                    rect.x,
                    rect.y,
                );
            }
            NodeContent::Image(image) => {
                render_image_to_canvas(image, step, rect, self.canvas)
            }
            NodeContent::Video(video) => {
                self.canvas.add_video(rect.clone(), video.video.clone());
            }
        }
    }*/

    if let Some(url) = &node.url {
        let rect = &layout.node_layout(node.node_id).unwrap().rect;
        canvas.add_link(Link::new(rect.clone(), url.clone()));
    }

    for child in &node.children {
        match child {
            NodeChild::Node(node) => render_node(render_ctx, node, layout, canvas),
            /*NodeChild::Draw(draw) => {
                todo!()
                //self.draw(step, node.node_id, draw)
            }*/
        }
    }
}

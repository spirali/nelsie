use crate::node::{Node, NodeChild};
use crate::render::canvas::{Canvas, Link};
use crate::render::draw::DrawItem;
use crate::render::layout::ComputedLayout;
use crate::shapes::FillAndStroke;
use crate::{NodeId, Path, Shape, ShapeRect};

pub(crate) fn render_node(node: &Node, layout: &ComputedLayout, canvas: &mut Canvas) {
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

    if let Some(url) = &node.url {
        let rect = &layout.node_layout(node.node_id).unwrap().rect;
        canvas.add_link(Link::new(rect.clone(), url.clone()));
    }

    for child in &node.children {
        match child {
            NodeChild::Node(node) => render_node(node, layout, canvas),
            NodeChild::Shape(shape) => match shape {
                Shape::Rect(rect) => render_rect(canvas, rect, layout, node.node_id),
                Shape::Oval(rect) => render_oval(canvas, rect, layout, node.node_id),
                Shape::Path(path) => render_path(canvas, path, layout, node.node_id),
            },
        }
    }
}

fn render_rect(canvas: &mut Canvas, rect: &ShapeRect, layout: &ComputedLayout, parent_id: NodeId) {
    let draw_rect = rect.eval(layout, parent_id);
    canvas.add_draw_item(rect.z_level, DrawItem::Rect(draw_rect));
}

fn render_oval(canvas: &mut Canvas, rect: &ShapeRect, layout: &ComputedLayout, parent_id: NodeId) {
    let draw_rect = rect.eval(layout, parent_id);
    canvas.add_draw_item(rect.z_level, DrawItem::Oval(draw_rect));
}

fn render_path(canvas: &mut Canvas, path: &Path, layout: &ComputedLayout, parent_id: NodeId) {
    let (draw_path, arrow1, arrow2) = path.eval(layout, parent_id);
    if let Some(draw_path) = draw_path {
        canvas.add_draw_item(path.z_level, DrawItem::Path(draw_path));
    }
    if let Some(arrow) = arrow1 {
        canvas.add_draw_item(path.z_level, DrawItem::Path(arrow));
    }
    if let Some(arrow) = arrow2 {
        canvas.add_draw_item(path.z_level, DrawItem::Path(arrow));
    }
}

use crate::color::Color;
use crate::node::Node;
use crate::render::canvas::Canvas;
use crate::render::layout::ComputedLayout;
use crate::render::node::render_node;

pub struct Page {
    pub(crate) node: Node,
    pub(crate) width: f32,
    pub(crate) height: f32,
    pub(crate) bg_color: Color,
}

impl Page {
    pub fn new(node: Node, width: f32, height: f32, bg_color: Color) -> Self {
        Page {
            node,
            width,
            height,
            bg_color,
        }
    }

    pub(crate) fn render_to_canvas(&self, layout: &ComputedLayout) -> Canvas {
        let mut canvas = Canvas::new(self.width, self.height, self.bg_color);
        render_node(&self.node, layout, &mut canvas);
        canvas.finish();
        canvas
    }

    // fn render(&self, render_ctx: &mut RenderContext) {
    //     let canvas = self.render_to_canvas(render_ctx);
    // }
}

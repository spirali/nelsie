use crate::render::draw::{DrawItem, DrawRect, PathBuilder};
use crate::shapes::FillAndStroke;

#[derive(Debug, Clone)]
pub struct Rectangle {
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
}

impl Rectangle {
    pub fn new(x: f32, y: f32, width: f32, height: f32) -> Self {
        Rectangle {
            x,
            y,
            width,
            height,
        }
    }

    pub(crate) fn draw(&self, fill_and_stroke: FillAndStroke) -> DrawItem {
        DrawItem::Rect(DrawRect {
            rectangle: self.clone(),
            fill_and_stroke,
        })
    }
    pub(crate) fn draw_rounded(
        &self,
        fill_and_stroke: FillAndStroke,
        border_radius: f32,
    ) -> DrawItem {
        if border_radius < 0.001 {
            self.draw(fill_and_stroke)
        } else {
            let mut builder = PathBuilder::new(fill_and_stroke);
            let x2 = self.x + self.width;
            let y2 = self.y + self.height;
            builder.move_to(self.x + border_radius, self.y);
            builder.line_to(x2 - border_radius, self.y);
            builder.quad_to(x2, self.y, x2, self.y + border_radius);
            builder.line_to(x2, y2 - border_radius);
            builder.quad_to(x2, y2, x2 - border_radius, y2);
            builder.line_to(self.x + border_radius, y2);
            builder.quad_to(self.x, y2, self.x, y2 - border_radius);
            builder.line_to(self.x, self.y + border_radius);
            builder.quad_to(self.x, self.y, self.x + border_radius, self.y);
            DrawItem::Path(builder.build())
        }
    }

    pub(crate) fn fit_content_with_aspect_ratio(&self, orig_w: f32, orig_h: f32) -> Rectangle {
        let target_w = self.width;
        let target_h = self.height;
        let orig_aspect = orig_w / orig_h;
        let target_aspect = target_w / target_h;

        let (new_w, new_h) = if orig_aspect > target_aspect {
            (target_w, target_w / orig_aspect)
        } else {
            (target_h * orig_aspect, target_h)
        };
        let x = self.x + (target_w - new_w) / 2.0;
        let y = self.y + (target_h - new_h) / 2.0;
        Rectangle::new(x, y, new_w, new_h)
    }

    pub(crate) fn invert_y_axis(&self, height: f32) -> Rectangle {
        Rectangle::new(
            self.x,
            height - self.y - self.height,
            self.width,
            self.height,
        )
    }
}

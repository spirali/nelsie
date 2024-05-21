use crate::model::{
    InTextAnchor, InTextAnchorId, InTextAnchorPoint, Resources, StyledLine, StyledText, TextAlign,
    TextStyle,
};

use resvg::usvg::FontStretch;
use std::collections::HashMap;

use crate::common::Rectangle;
use crate::parsers::SimpleXmlWriter;
use crate::render::canvas::{Canvas, CanvasItem};
use crate::render::layout::TextLayout;
use crate::render::pathbuilder::stroke_and_fill_svg;
use resvg::usvg;

pub(crate) fn get_in_text_anchor_point(text: &StyledText, point: &InTextAnchorPoint) -> StyledText {
    let line = &text.styled_lines[point.line_idx as usize];
    StyledText {
        styled_lines: vec![StyledLine {
            spans: line.spans[..point.span_idx as usize].to_vec(),
            text: line.text.to_string(),
        }],
        styles: text.styles.clone(),
        default_font_size: text.default_font_size,
        default_line_spacing: text.default_line_spacing,
    }
}

pub(crate) fn get_text_layout(
    resources: &Resources,
    text: &StyledText,
    align: TextAlign,
    anchors: &HashMap<InTextAnchorId, InTextAnchor>,
) -> (f32, f32, TextLayout) {
    let mut anchor_pos = HashMap::new();
    for anchor in anchors.values() {
        if !anchor_pos.contains_key(&anchor.start) {
            let sx = get_text_width(resources, &get_in_text_anchor_point(text, &anchor.start));
            anchor_pos.insert(anchor.start.clone(), sx);
        }
        if !anchor_pos.contains_key(&anchor.end) {
            let sx = get_text_width(resources, &get_in_text_anchor_point(text, &anchor.end));
            anchor_pos.insert(anchor.end.clone(), sx);
        }
    }

    let mut result_lines = Vec::with_capacity(text.styled_lines.len());

    let mut tmp_text = StyledText {
        styled_lines: Vec::new(),
        styles: text.styles.clone(),
        default_font_size: text.default_font_size,
        default_line_spacing: text.default_line_spacing,
    };

    let mut current_y = 0.0;
    for idx in 0..text.styled_lines.len() {
        tmp_text.styled_lines = vec![text.styled_lines[idx].clone()];
        let sx = get_text_width(resources, &tmp_text);
        let styled_line = &text.styled_lines[idx];
        let size = styled_line
            .font_size(&tmp_text.styles)
            .unwrap_or(tmp_text.default_font_size);
        let descender = if idx == 0 {
            0.0
        } else {
            styled_line.line_descender(&tmp_text.styles).unwrap_or(0.0)
        };

        result_lines.push(Rectangle {
            x: 0.0,
            y: current_y - descender,
            width: sx,
            height: size,
        });
        current_y += if idx == 0 {
            size
        } else {
            size * tmp_text.default_line_spacing
        }
    }

    let width = result_lines
        .iter()
        .map(|line| line.width)
        .max_by(|w1, w2| w1.partial_cmp(w2).unwrap())
        .unwrap_or(0.0);

    match align {
        TextAlign::Start => { /* Do nothing */ }
        TextAlign::Center => {
            for line in result_lines.iter_mut() {
                line.x = (width - line.width) / 2.0;
            }
        }
        TextAlign::End => {
            for line in result_lines.iter_mut() {
                line.x = width - line.width;
            }
        }
    }

    let anchor_points = anchors
        .iter()
        .map(|(anchor_id, anchor)| {
            let x = anchor_pos.get(&anchor.start).copied().unwrap();
            let start_line = &result_lines[anchor.start.line_idx as usize];
            (
                *anchor_id,
                Rectangle {
                    x: start_line.x + x,
                    y: start_line.y,
                    width: anchor_pos.get(&anchor.end).copied().unwrap() - x,
                    height: {
                        let end_line = &result_lines[anchor.end.line_idx as usize];
                        end_line.y + end_line.height - start_line.y
                    },
                },
            )
        })
        .collect();

    let result = TextLayout {
        lines: result_lines,
        anchor_points,
    };

    (width, text.height(), result)
}

fn get_text_width(resources: &Resources, text: &StyledText) -> f32 {
    assert_eq!(text.styled_lines.len(), 1);
    if text.styled_lines[0].text.is_empty() {
        return 0f32;
    }
    let mut xml = SimpleXmlWriter::new();
    xml.begin("svg");
    xml.attr("xmlns", "http://www.w3.org/2000/svg");
    render_text_to_svg(&mut xml, text, 0.0, 0.0, TextAlign::Start);
    xml.end("svg");
    let svg = xml.into_string();

    let tree = match usvg::Tree::from_str(&svg, &usvg::Options::default(), &resources.font_db) {
        Ok(tree) => tree,
        Err(_) => {
            log::debug!("Failed to parse SVG");
            return 0.0;
        }
    };

    let mut width = tree.root().abs_bounding_box().right();

    /* Because bounding box ignores span that contains only spaces, we have problem with trailing spaces if they are in separate style,
       we need to increase with for each trailing space
    */
    let line = &text.styled_lines[0];
    let line_len = line.spans.iter().map(|s| s.length as usize).sum();
    if line.text.chars().take(line_len).all(|x| x == ' ') {
        for span in &line.spans {
            let style = &text.styles[span.style_idx as usize];
            width += style.font.space_size * style.size * span.length as f32;
        }
    }

    width
}

fn stretch_to_svg(s: FontStretch) -> &'static str {
    match s {
        FontStretch::UltraCondensed => "ultra-condensed",
        FontStretch::ExtraCondensed => "extra-condensed",
        FontStretch::Condensed => "condensed",
        FontStretch::SemiCondensed => "semi-condensed",
        FontStretch::Normal => "normal",
        FontStretch::SemiExpanded => "semi-expanded",
        FontStretch::Expanded => "expanded",
        FontStretch::ExtraExpanded => "extra-expanded",
        FontStretch::UltraExpanded => "ultra-expanded",
    }
}

fn render_line_to_svg(
    xml: &mut SimpleXmlWriter,
    styles: &[TextStyle],
    x: f32,
    y: f32,
    line: &StyledLine,
    align: TextAlign,
) {
    let mut position = 0;

    xml.begin("tspan");
    xml.attr("x", x);
    xml.attr("y", y);

    match align {
        TextAlign::Start => { /* Start is default */ }
        TextAlign::Center => xml.attr("text-anchor", "middle"),
        TextAlign::End => xml.attr("text-anchor", "end"),
    };

    for span in &line.spans {
        let style = &styles[span.style_idx as usize];
        xml.begin("tspan");
        xml.attr("font-family", &style.font.family_name);
        xml.attr("font-weight", style.weight);
        if style.italic {
            xml.attr("font-style", "italic");
        }
        xml.attr("font-size", style.size);

        if style.underline || style.overline || style.line_through {
            xml.attr_buf("text-decoration", |b| {
                if style.underline {
                    b.push_str("underline ")
                }
                if style.overline {
                    b.push_str("overline ")
                }
                if style.line_through {
                    b.push_str("line-through ")
                }
            });
        }

        stroke_and_fill_svg(
            xml,
            &style.stroke.as_ref().map(|s| s.as_ref().clone()),
            &style.color,
        );
        match style.stretch {
            FontStretch::Normal => { /* do nothing */ }
            s => xml.attr("font-stretch", stretch_to_svg(s)),
        }
        let text = &line.text[position..(position + span.length as usize)];
        position += span.length as usize;
        xml.text(text);
        xml.end("tspan");
    }
    xml.end("tspan");
}

pub(crate) fn render_text_to_canvas(
    styled_text: &StyledText,
    rect: &Rectangle,
    align: TextAlign,
    canvas: &mut Canvas,
) {
    let x = match align {
        TextAlign::Start => rect.x,
        TextAlign::Center => rect.x + rect.width / 2.0,
        TextAlign::End => rect.x + rect.width,
    };
    let mut xml = SimpleXmlWriter::new();
    render_text_to_svg(&mut xml, styled_text, x, rect.y, align);
    canvas.add_item(CanvasItem::SvgChunk(xml.into_string()));
}

pub(crate) fn render_text_to_svg(
    xml: &mut SimpleXmlWriter,
    styled_text: &StyledText,
    x: f32,
    y: f32,
    align: TextAlign,
) {
    xml.begin("text");
    xml.attr("xml:space", "preserve");
    let mut current_y = y;
    for (idx, styled_line) in styled_text.styled_lines.iter().enumerate() {
        let size = styled_line
            .font_size(&styled_text.styles)
            .unwrap_or(styled_text.default_font_size);

        let height = if idx != 0 {
            size * styled_text.default_line_spacing
        } else {
            size
        };
        let descender = styled_line
            .line_descender(&styled_text.styles)
            .unwrap_or(0.0);
        current_y += height;
        render_line_to_svg(
            xml,
            &styled_text.styles,
            x,
            current_y + descender,
            styled_line,
            align,
        );
    }
    xml.end("text");
}

use std::ops::Deref;
use pyo3::{pyclass, pymethods, PyResult};
use pyo3::exceptions::{PyException, PyValueError};
use crate::model::{Node, NodeChild, Slide, SlideDeck};
use crate::parsers::parse_color;
use crate::pyinterface::r#box::BoxConfig;
use crate::render::{OutputConfig, render_slide_deck};

#[pyclass]
pub(crate) struct Deck {
    deck: SlideDeck
}

type SlideId = u32;
type BoxId = Vec<u32>;

fn resolve_box_id<'a>(node: &'a mut Node, box_id: &[u32]) -> Option<&'a mut Node> {
    if box_id.is_empty() {
        return Some(node);
    }
    node.child_node_mut(box_id[0] as usize).and_then(|child| resolve_box_id(child, &box_id[1..]))
}

#[pymethods]
impl Deck {
    #[new]
    fn new() -> PyResult<Self> {
        Ok(Deck {
            deck: SlideDeck::default()
        })
    }

    fn new_slide(&mut self, width: f32, height: f32, bg_color: &str, name: String) -> PyResult<SlideId> {
        let slide_id = self.deck.slides.len() as SlideId;
        self.deck.slides.push(Slide::new(width, height, name, parse_color(bg_color)?));
        Ok(slide_id)
    }

    fn new_box(&mut self, slide_id: SlideId, box_id: BoxId, config: BoxConfig) -> PyResult<(BoxId, u32)> {
        if let Some(slide) = self.deck.slides.get_mut(slide_id as usize) {
            let node_id = slide.new_node_id();
            if let Some(parent_node) = resolve_box_id(&mut slide.node, &box_id) {
                let node = config.make_node(node_id)?;
                let new_id = parent_node.children.len() as u32;
                let node_id = node.node_id;
                parent_node.children.push(NodeChild::Node(node));
                let mut new_box_id = box_id;
                new_box_id.push(new_id);
                Ok((new_box_id, node_id.as_u32()))
            } else {
                Err(PyException::new_err("Invalid box id"))
            }
        } else {gi
            Err(PyException::new_err("Invalid slide id"))
        }
    }

    fn render(&self, output_pdf: Option<&str>, output_svg: Option<&str>, output_png: Option<&str>) -> PyResult<()> {
        Ok(render_slide_deck(&self.deck, &OutputConfig {
            output_pdf: output_pdf.map(|p| std::path::Path::new(p)),
            output_png: output_png.map(|p| std::path::Path::new(p)),
            output_svg: output_svg.map(|p| std::path::Path::new(p)),
        })?)
    }
}
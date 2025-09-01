use crate::render::canvas::Canvas;
use crate::render::composer::Composer;
use crate::render::content::ContentMap;
use crate::render::layout::ComputedLayout;
use crate::{NodeId, Rectangle};
use std::collections::HashMap;
use std::sync::Mutex;

#[derive(Clone, Debug)]
pub struct PageLayout {
    pub node_layouts: HashMap<NodeId, Rectangle>,
}

pub(crate) struct LayoutInfoComposer {
    page_layouts: Mutex<Vec<PageLayout>>,
}

impl LayoutInfoComposer {
    pub fn new(n_pages: usize) -> Self {
        Self {
            page_layouts: Mutex::new(vec![
                PageLayout {
                    node_layouts: HashMap::new()
                };
                n_pages
            ]),
        }
    }

    pub fn finish(self) -> Vec<PageLayout> {
        self.page_layouts.into_inner().unwrap()
    }
}

impl Composer for LayoutInfoComposer {
    fn add_page(
        &self,
        page_idx: usize,
        _canvas: Canvas,
        _content_map: &ContentMap,
        compute_layout: &ComputedLayout,
    ) -> crate::Result<()> {
        let node_layouts = compute_layout
            .iter()
            .map(|(node_id, data)| (*node_id, data.rect.clone()))
            .collect();
        let page_layout = PageLayout { node_layouts };
        self.page_layouts.lock().unwrap()[page_idx] = page_layout;
        Ok(())
    }
}

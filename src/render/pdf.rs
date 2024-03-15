use pdf_writer::Finish;
use pdf_writer::{Content, Name, Rect, Ref};
use std::path::Path;
use svg2pdf::usvg;

pub(crate) struct PdfBuilder {
    pdf: pdf_writer::Pdf,
    page_ids: Vec<Ref>,
    page_idx: usize,
    alloc_ref: Ref,
    page_tree_id: Ref,
}

impl PdfBuilder {
    pub fn new(n_pages: u32) -> Self {
        let mut pdf = pdf_writer::Pdf::new();

        let mut alloc_ref = Ref::new(1);

        let catalog_id = alloc_ref.bump();
        let page_tree_id = alloc_ref.bump();

        pdf.catalog(catalog_id).pages(page_tree_id);

        let page_ids: Vec<Ref> = (0..n_pages).map(|_| alloc_ref.bump()).collect();
        pdf.pages(page_tree_id)
            .kids(page_ids.iter().copied())
            .count(page_ids.len() as i32);

        PdfBuilder {
            pdf,
            page_ids,
            page_idx: 0,
            alloc_ref,
            page_tree_id,
        }
    }

    pub fn add_page_from_svg(&mut self, tree: usvg::Tree) {
        let page_id = self.page_ids[self.page_idx];

        let content_id = self.alloc_ref.bump();
        let mut page = self.pdf.page(page_id);
        page.media_box(Rect::new(0.0, 0.0, tree.size.width(), tree.size.height()));
        page.parent(self.page_tree_id);
        // page.group()
        //     .transparency()
        //     .isolated(true)
        //     .knockout(false)
        //     .color_space()
        //     .srgb();
        page.contents(content_id);

        let name_str = format!("S{}", self.page_idx);
        let svg_name = Name(name_str.as_bytes());
        let mut resources = page.resources();

        let svg_id = self.alloc_ref.bump(); // !!! no .bump has to occur until convert_tree_into !!!

        resources.x_objects().pair(svg_name, svg_id);
        resources.finish();
        page.finish();

        self.alloc_ref =
            svg2pdf::convert_tree_into(&tree, svg2pdf::Options::default(), &mut self.pdf, svg_id);
        let mut content = Content::new();
        content
            .transform([tree.size.width(), 0.0, 0.0, tree.size.height(), 0.0, 0.0])
            .x_object(svg_name);
        self.pdf.stream(content_id, &content.finish());

        self.page_idx += 1;
    }

    pub fn write(self, path: &Path) -> crate::Result<()> {
        std::fs::write(path, self.pdf.finish())?;
        Ok(())
    }
}

use pdf_writer::{Chunk, Finish};
use pdf_writer::{Content, Name, Rect, Ref};
use resvg::usvg::fontdb;
use std::collections::HashMap;
use std::path::Path;
use svg2pdf::usvg;

pub(crate) struct PdfBuilder {
    pdf: pdf_writer::Pdf,
    page_ids: Vec<Ref>,
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
            alloc_ref,
            page_tree_id,
        }
    }

    fn add_chunk(&mut self, chunk: Chunk, chunk_ref: Ref) -> Ref {
        let mut map = HashMap::<Ref, Ref>::new();
        let chunk = chunk.renumber(|r| *map.entry(r).or_insert_with(|| self.alloc_ref.bump()));
        self.pdf.extend(&chunk);
        *map.get(&chunk_ref).unwrap()
    }

    pub fn add_page_from_svg(
        &mut self,
        page_idx: usize,
        tree: usvg::Tree,
        font_db: &fontdb::Database,
    ) {
        let (svg_chunk, svg_id) =
            svg2pdf::to_chunk(&tree, svg2pdf::ConversionOptions::default(), font_db);
        let svg_id = self.add_chunk(svg_chunk, svg_id);

        let page_id = self.page_ids[page_idx];
        let name_str = format!("S{}", page_idx);
        let svg_name = Name(name_str.as_bytes());
        let content_id = self.alloc_ref.bump();

        let mut page = self.pdf.page(page_id);
        page.media_box(Rect::new(
            0.0,
            0.0,
            tree.size().width(),
            tree.size().height(),
        ));
        page.parent(self.page_tree_id);
        page.contents(content_id);

        let mut resources = page.resources();
        resources.x_objects().pair(svg_name, svg_id);
        resources.finish();
        page.finish();

        let mut content = Content::new();
        content
            .transform([
                tree.size().width(),
                0.0,
                0.0,
                tree.size().height(),
                0.0,
                0.0,
            ])
            .x_object(svg_name);
        self.pdf.stream(content_id, &content.finish());
    }

    pub fn write(self, path: &Path) -> crate::Result<()> {
        std::fs::write(path, self.pdf.finish())?;
        Ok(())
    }
}

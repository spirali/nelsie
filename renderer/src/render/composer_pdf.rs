use crate::ContentId;
use crate::render::canvas::Canvas;
use crate::render::composer::{Composer, PngCollectorComposer};
use crate::render::content::{Content, ContentMap};
use miniz_oxide::deflate::CompressionLevel;
use pdf_writer::{Chunk, Ref};
use std::collections::HashMap;
use std::ops::DerefMut;
use std::sync::Mutex;

pub(crate) struct PdfComposer {
    chunks: Mutex<Vec<Chunk>>,
    pdf: Mutex<pdf_writer::Pdf>,
    content_to_ref_builder: Mutex<HashMap<ContentId, Ref>>,
    content_to_ref: HashMap<ContentId, Ref>,
    page_tree_ref: Ref,
    page_refs: Vec<Ref>,
    n_rendering_items: usize,
    compression_level: u8,
}

impl PdfComposer {
    pub fn new(n_pages: usize, compression_level: u8) -> Self {
        let mut alloc_ref = Ref::new(1);
        let mut pdf = pdf_writer::Pdf::new();
        let (page_tree_ref, page_refs) = init_pdf(&mut pdf, &mut alloc_ref, n_pages);
        let n_rendering_items = n_pages;
        PdfComposer {
            chunks: Mutex::new(Vec::new()),
            page_tree_ref,
            pdf: Mutex::new(pdf),
            page_refs,
            n_rendering_items,
            compression_level,
            content_to_ref: HashMap::new(),
            content_to_ref_builder: Mutex::new(HashMap::new()),
        }
    }

    pub fn add_chunk(&self, chunk: Chunk) {
        if let Ok(mut pdf) = self.pdf.try_lock() {
            pdf.extend(&chunk);
            let chunks = {
                let mut chunks = self.chunks.lock().unwrap();
                std::mem::take(chunks.deref_mut())
            };
            for chunk in chunks {
                pdf.extend(&chunk);
            }
        } else {
            self.chunks.lock().unwrap().push(chunk);
        }
    }

    pub fn finish(self) -> Vec<u8> {
        let mut pdf = self.pdf.into_inner().unwrap();
        let chunks = self.chunks.into_inner().unwrap();
        for chunk in chunks.into_iter() {
            pdf.extend(&chunk);
        }
        pdf.finish()
    }
}

impl<'a> Composer for PdfComposer {
    fn add_page(
        &self,
        page_idx: usize,
        canvas: Canvas,
        content_map: &ContentMap,
    ) -> crate::Result<()> {
        let mut ref_allocator =
            PdfRefAllocator::new(self.page_refs[page_idx], self.n_rendering_items);
        let page = canvas.into_pdf_page(
            &mut ref_allocator,
            self.page_tree_ref,
            self.compression_level,
            content_map,
            &self.content_to_ref,
        )?;
        self.add_chunk(page);
        Ok(())
    }

    fn preprocess_content(&self, content_id: ContentId, content: &Content) -> crate::Result<()> {
        //todo!()
        Ok(())
    }

    fn preprocessing_finished(&mut self) {
        let mut map = self.content_to_ref_builder.lock().unwrap();
        std::mem::swap(&mut *map, &mut self.content_to_ref);
    }
}

// pub struct PdfGlobalInfo {
//     page_refs: Vec<Ref>,
//     page_tree_ref: Ref,
//     alloc_ref: Ref,
// }

fn init_pdf(pdf: &mut pdf_writer::Pdf, alloc_ref: &mut Ref, n_pages: usize) -> (Ref, Vec<Ref>) {
    let catalog_ref = alloc_ref.bump();
    let page_tree_ref = alloc_ref.bump();
    pdf.catalog(catalog_ref).pages(page_tree_ref);
    let page_refs: Vec<Ref> = (0..n_pages).map(|_| alloc_ref.bump()).collect();
    pdf.pages(page_tree_ref)
        .kids(page_refs.iter().copied())
        .count(page_refs.len() as i32);
    (page_tree_ref, page_refs)
}

// impl PdfGlobalInfo {
//     pub fn new(pdf: &mut pdf_writer::Pdf, n_pages: usize) -> Self {
//         let mut alloc_ref = Ref::new(1);
//
//         PdfGlobalInfo {
//             page_refs,
//             page_tree_ref,
//             alloc_ref,
//         }
//     }
//
//     pub fn page_ref_allocator(&self, page_idx: usize) -> PdfRefAllocator {
//         PdfRefAllocator {
//             counter: self.page_refs[page_idx].get(),
//             step: self.page_refs.len() as i32 + 1,
//         }
//     }
//
//     pub fn ref_bump(&mut self) -> Ref {
//         let r = self.alloc_ref;
//         self.alloc_ref = Ref::new(r.get() + self.page_refs.len() as i32 + 1);
//         r
//     }
//
//     pub fn page_tree_ref(&self) -> Ref {
//         self.page_tree_ref
//     }
// }

/*
   Because rendering of PDF is done in parallel, we need to create separate
   counter for each page, so we do not need a synchronization
   Counters will be
   for page 0: page_refs[0] + (n_pages + 1) + 2 * (n_pages + 1) ...
   for page 1: page_refs[1] + (n_pages + 1) + 2 * (n_pages + 1) ...
   ...
   we are setting n_pages + 1 because the last allocator is reserved for generic purpose and not specific page
*/
pub(crate) struct PdfRefAllocator {
    counter: i32,
    step: i32,
}

impl PdfRefAllocator {
    pub fn new(initial_ref: Ref, n_rendering_items: usize) -> Self {
        PdfRefAllocator {
            counter: initial_ref.get(),
            step: n_rendering_items as i32,
        }
    }

    pub fn bump(&mut self) -> Ref {
        let rf = self.counter;
        self.counter += self.step;
        Ref::new(rf)
    }
}

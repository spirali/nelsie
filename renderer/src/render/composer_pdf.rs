use crate::render::canvas::Canvas;
use crate::render::composer::{Composer, PngCollectorComposer};
use pdf_writer::{Chunk, Ref};
use std::ops::DerefMut;
use std::sync::Mutex;

pub(crate) struct PdfComposer {
    chunks: Mutex<Vec<Chunk>>,
    pdf: Mutex<pdf_writer::Pdf>,
    pdf_global_info: PdfGlobalInfo,
}

impl PdfComposer {
    pub fn new(n_pages: usize) -> Self {
        let mut pdf = pdf_writer::Pdf::new();
        let mut pdf_global_info = PdfGlobalInfo::new(&mut pdf, n_pages);
        PdfComposer {
            chunks: Mutex::new(Vec::new()),
            pdf: Mutex::new(pdf),
            pdf_global_info,
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
    fn add_page(&self, page_idx: usize, canvas: &Canvas) -> crate::Result<()> {
        Ok(())
    }
}

pub struct PdfGlobalInfo {
    page_refs: Vec<Ref>,
    page_tree_ref: Ref,
    alloc_ref: Ref,
}

impl PdfGlobalInfo {
    pub fn new(pdf: &mut pdf_writer::Pdf, n_pages: usize) -> Self {
        let mut alloc_ref = Ref::new(1);

        let catalog_ref = alloc_ref.bump();
        let page_tree_ref = alloc_ref.bump();

        pdf.catalog(catalog_ref).pages(page_tree_ref);
        let page_refs: Vec<Ref> = (0..n_pages).map(|_| alloc_ref.bump()).collect();
        pdf.pages(page_tree_ref)
            .kids(page_refs.iter().copied())
            .count(page_refs.len() as i32);
        PdfGlobalInfo {
            page_refs,
            page_tree_ref,
            alloc_ref,
        }
    }

    pub fn page_ref_allocator(&self, page_idx: usize) -> PdfRefAllocator {
        PdfRefAllocator {
            counter: self.page_refs[page_idx].get(),
            step: self.page_refs.len() as i32 + 1,
        }
    }

    pub fn ref_bump(&mut self) -> Ref {
        let r = self.alloc_ref;
        self.alloc_ref = Ref::new(r.get() + self.page_refs.len() as i32 + 1);
        r
    }

    pub fn page_tree_ref(&self) -> Ref {
        self.page_tree_ref
    }
}

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
    pub fn bump(&mut self) -> Ref {
        let rf = self.counter;
        self.counter += self.step;
        Ref::new(rf)
    }
}

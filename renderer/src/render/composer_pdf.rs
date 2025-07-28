use crate::ContentId;
use crate::render::canvas::Canvas;
use crate::render::composer::{Composer, PngCollectorComposer};
use crate::render::content::{Content, ContentBody, ContentMap};
use crate::render::pdfdraw::{PdfWriter, init_pdf, path_to_pdf};
use crate::render::text::RenderedText;
use miniz_oxide::deflate::CompressionLevel;
use pdf_writer::{Chunk, Filter, Finish, Rect, Ref};
use std::collections::HashMap;
use std::ops::DerefMut;
use std::sync::Mutex;
use std::sync::atomic::{AtomicI32, Ordering};

pub(crate) struct PdfComposer {
    chunks: Mutex<Vec<Chunk>>,
    pdf: Mutex<pdf_writer::Pdf>,
    content_to_ref_builder: Mutex<HashMap<ContentId, Ref>>,
    content_to_ref: HashMap<ContentId, Ref>,
    page_tree_ref: Ref,
    page_refs: Vec<Ref>,
    compression_level: u8,
    ref_allocator: PdfRefAllocator,
}

impl PdfComposer {
    pub fn new(n_pages: usize, compression_level: u8) -> Self {
        let mut alloc_ref = Ref::new(1);
        let mut pdf = pdf_writer::Pdf::new();
        let (page_tree_ref, page_refs) = init_pdf(&mut pdf, &mut alloc_ref, n_pages);
        PdfComposer {
            chunks: Mutex::new(Vec::new()),
            page_tree_ref,
            pdf: Mutex::new(pdf),
            page_refs,
            compression_level,
            content_to_ref: HashMap::new(),
            content_to_ref_builder: Mutex::new(HashMap::new()),
            ref_allocator: PdfRefAllocator::new(alloc_ref),
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
        let page = canvas.into_pdf_page(
            &self.ref_allocator,
            self.page_refs[page_idx],
            self.page_tree_ref,
            self.compression_level,
            content_map,
            &self.content_to_ref,
        )?;
        self.add_chunk(page);
        Ok(())
    }

    fn preprocess_content(&self, content_id: ContentId, content: &Content) -> crate::Result<()> {
        match content.body() {
            ContentBody::Text((text, is_shared)) if *is_shared => {
                let (width, height) = content.size();
                let (chunk, rf) = create_text_xobject(
                    text,
                    width,
                    height,
                    &self.ref_allocator,
                    self.compression_level,
                );
                self.content_to_ref_builder
                    .lock()
                    .unwrap()
                    .insert(content_id, rf);
                self.add_chunk(chunk);
            }
            _ => {}
        }
        Ok(())
    }

    fn preprocessing_finished(&mut self) {
        let mut map = self.content_to_ref_builder.lock().unwrap();
        std::mem::swap(&mut *map, &mut self.content_to_ref);
    }
    
    fn needs_image_preprocessing(&self) -> bool {
        true
    }
}

// pub struct PdfGlobalInfo {
//     page_refs: Vec<Ref>,
//     page_tree_ref: Ref,
//     alloc_ref: Ref,
// }

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
    counter: AtomicI32,
}

impl PdfRefAllocator {
    pub fn new(rf: Ref) -> Self {
        Self {
            counter: AtomicI32::new(rf.get()),
        }
    }
    pub fn bump(&self) -> Ref {
        let rf = self.counter.fetch_add(1, Ordering::Relaxed);
        Ref::new(rf)
    }
}

fn create_text_xobject(
    text: &RenderedText,
    width: f32,
    height: f32,
    allocator: &PdfRefAllocator,
    compression_level: u8,
) -> (Chunk, Ref) {
    let obj_ref = allocator.bump();
    let mut pdf_writer = PdfWriter::new(allocator);
    pdf_writer.content.save_state();
    pdf_writer
        .content
        .transform([1.0 / width, 0.0, 0.0, 1.0 / height, 0.0, 0.0]);
    for path in text.paths() {
        path_to_pdf(&mut pdf_writer, path)
    }
    pdf_writer.content.restore_state();

    let mut content_data = pdf_writer.content.finish();
    if compression_level > 0 {
        content_data = miniz_oxide::deflate::compress_to_vec_zlib(&content_data, compression_level)
    }
    let mut x_obj = pdf_writer.chunk.form_xobject(obj_ref, &content_data);
    x_obj.bbox(Rect::new(0.0, 0.0, 1.0, 1.0));
    if compression_level > 0 {
        x_obj.filter(Filter::FlateDecode);
    }
    x_obj.finish();
    (pdf_writer.chunk, obj_ref)
}

use crate::render::canvas::Canvas;
use crate::render::composer::Composer;
use crate::render::content::{Content, ContentBody, ContentMap};
use crate::render::layout::ComputedLayout;
use crate::render::pdfdraw::{PdfWriter, init_pdf, path_to_pdf};
use crate::render::text::RenderedText;
use crate::{ContentId, InMemoryBinImage, InMemorySvgImage, Resources};
use image::GenericImageView;
use miniz_oxide::deflate::{CompressionLevel, compress_to_vec_zlib};
use pdf_writer::{Chunk, Filter, Finish, Rect, Ref};
use std::borrow::Cow;
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

impl Composer for PdfComposer {
    fn add_page(
        &self,
        page_idx: usize,
        canvas: Canvas,
        content_map: &ContentMap,
        _layout: &ComputedLayout,
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

    fn preprocess_content(
        &self,
        resources: &Resources,
        content_id: ContentId,
        content: &Content,
    ) -> crate::Result<()> {
        let (chunk, rf) = match content.body() {
            ContentBody::Text((text, is_shared)) if *is_shared => {
                let (width, height) = content.size();
                create_text_xobject(
                    text,
                    width,
                    height,
                    &self.ref_allocator,
                    self.compression_level,
                )
            }
            ContentBody::Text(_) => {
                // not shared, do nothing
                return Ok(());
            }
            ContentBody::BinImage(image) => create_image_xobject(image, &self.ref_allocator),
            ContentBody::SvgImage(image) => {
                create_svg_xobject(resources, image, &self.ref_allocator)?
            }
            ContentBody::Composition(_) => {
                return Ok(());
            }
        };
        self.content_to_ref_builder
            .lock()
            .unwrap()
            .insert(content_id, rf);
        self.add_chunk(chunk);
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
    pub fn bump_pair(&self) -> (Ref, Ref) {
        let rf = self.counter.fetch_add(2, Ordering::Relaxed);
        (Ref::new(rf), Ref::new(rf + 1))
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
        .transform([1.0 / width, 0.0, 0.0, -1.0 / height, 0.0, 1.0]);
    for path in text.paths() {
        path_to_pdf(&mut pdf_writer, path)
    }
    pdf_writer.content.restore_state();

    let mut content_data = pdf_writer.content.finish();
    if compression_level > 0 {
        content_data = compress_to_vec_zlib(&content_data, compression_level)
    }
    let mut x_obj = pdf_writer.chunk.form_xobject(obj_ref, &content_data);
    x_obj.bbox(Rect::new(0.0, 0.0, 1.0, 1.0));
    if compression_level > 0 {
        x_obj.filter(Filter::FlateDecode);
    }
    x_obj.finish();
    (pdf_writer.chunk, obj_ref)
}

pub fn create_svg_xobject(
    resources: &Resources,
    svg_image: &InMemorySvgImage,
    pdf_ref_allocator: &PdfRefAllocator,
) -> crate::Result<(Chunk, Ref)> {
    let options = svg2pdf::usvg::Options {
        fontdb: resources.font_db.as_ref().unwrap().clone(),
        ..Default::default()
    };
    let tree = svg2pdf::usvg::Tree::from_str(&svg_image.as_string(), &options)?;
    let (svg_chunk, svg_ref) = svg2pdf::to_chunk(&tree, svg2pdf::ConversionOptions::default())
        .map_err(|e| crate::Error::generic_err(format!("PDF conversion error: {}", e)))?;
    let mut chunk = Chunk::with_capacity(svg_chunk.len());
    let svg_ref = renumber_into(&svg_chunk, &mut chunk, pdf_ref_allocator, svg_ref);
    Ok((chunk, svg_ref))
}

fn renumber_into(
    chunk: &Chunk,
    target: &mut Chunk,
    alloc_ref: &PdfRefAllocator,
    top_ref: Ref,
) -> Ref {
    let mut map = HashMap::<Ref, Ref>::new();
    chunk.renumber_into(target, |r| {
        *map.entry(r).or_insert_with(|| alloc_ref.bump())
    });
    *map.get(&top_ref).unwrap()
}

pub fn create_image_xobject(
    bin_image: &InMemoryBinImage,
    pdf_ref_allocator: &PdfRefAllocator,
) -> (Chunk, Ref) {
    let (filter, encoded, mask, w, h) = match bin_image {
        InMemoryBinImage::Jpeg(data) => {
            let dynamic =
                image::load_from_memory_with_format(data, image::ImageFormat::Jpeg).unwrap();
            assert_eq!(dynamic.color(), image::ColorType::Rgb8);
            (
                Filter::DctDecode,
                Cow::Borrowed(data.as_slice()),
                None,
                dynamic.width(),
                dynamic.height(),
            )
        }

        InMemoryBinImage::Png(data) => {
            let level = CompressionLevel::DefaultLevel as u8;
            let dynamic =
                image::load_from_memory_with_format(data, image::ImageFormat::Png).unwrap();
            let w = dynamic.width();
            let h = dynamic.height();
            let encoded = compress_to_vec_zlib(dynamic.to_rgb8().as_raw(), level);

            let mask = dynamic.color().has_alpha().then(|| {
                let alphas: Vec<_> = dynamic.pixels().map(|p| (p.2).0[3]).collect();
                compress_to_vec_zlib(&alphas, level)
            });

            (Filter::FlateDecode, Cow::Owned(encoded), mask, w, h)
        }
    };

    let (image_ref, mask_ref) = pdf_ref_allocator.bump_pair();
    let mut chunk = Chunk::new();
    let mut image = chunk.image_xobject(image_ref, &encoded);
    image.filter(filter);
    image.width(w as i32);
    image.height(h as i32);
    image.color_space().device_rgb();
    image.bits_per_component(8);
    if mask.is_some() {
        image.s_mask(mask_ref);
    };
    image.finish();

    if let Some(encoded) = &mask {
        let mut s_mask = chunk.image_xobject(mask_ref, encoded);
        s_mask.filter(filter);
        s_mask.width(w as i32);
        s_mask.height(h as i32);
        s_mask.color_space().device_gray();
        s_mask.bits_per_component(8);
        s_mask.finish();
    }
    (chunk, image_ref)
}

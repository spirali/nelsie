use image::GenericImageView;
use miniz_oxide::deflate::{compress_to_vec_zlib, CompressionLevel};
use pdf_writer::Ref;
use pdf_writer::{Chunk, Filter, Finish};

use std::borrow::Cow;
use std::path::Path;
use std::sync::atomic::{AtomicI32, Ordering};

pub(crate) struct PdfBuilder {
    pdf: pdf_writer::Pdf,
    page_refs: Vec<Ref>,
    alloc_ref: PdfRefAllocator,
    page_tree_ref: Ref,
}

pub(crate) struct PdfRefAllocator {
    counter: AtomicI32,
}

impl PdfRefAllocator {
    pub fn new(rf: Ref) -> Self {
        PdfRefAllocator {
            counter: AtomicI32::new(rf.get()),
        }
    }

    pub fn bump(&self) -> Ref {
        Ref::new(self.counter.fetch_add(1, Ordering::Relaxed))
    }
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
            page_refs: page_ids,
            alloc_ref: PdfRefAllocator::new(alloc_ref),
            page_tree_ref: page_tree_id,
        }
    }

    pub fn page_ref(&self, page_idx: u32) -> Ref {
        self.page_refs[page_idx as usize]
    }

    pub fn page_tree_ref(&self) -> Ref {
        self.page_tree_ref
    }

    pub fn add_chunk(&mut self, chunk: Chunk) {
        self.pdf.extend(&chunk);
    }

    pub fn alloc_ref(&self) -> &PdfRefAllocator {
        &self.alloc_ref
    }

    pub fn ref_bump(&mut self) -> Ref {
        self.alloc_ref.bump()
    }

    pub fn write(self, path: &Path) -> crate::Result<()> {
        std::fs::write(path, self.pdf.finish())?;
        Ok(())
    }
}

pub fn image_to_pdf_chunk(
    image_format: image::ImageFormat,
    data: &[u8],
    image_ref: Ref,
    mask_ref: Option<Ref>,
) -> Chunk {
    let dynamic = image::load_from_memory_with_format(data, image_format).unwrap();

    let (filter, encoded, mask) = match image_format {
        image::ImageFormat::Jpeg => {
            assert_eq!(dynamic.color(), image::ColorType::Rgb8);
            (Filter::DctDecode, Cow::Borrowed(data), None)
        }

        image::ImageFormat::Png => {
            let level = CompressionLevel::DefaultLevel as u8;
            let encoded = compress_to_vec_zlib(dynamic.to_rgb8().as_raw(), level);

            // If there's an alpha channel, extract the pixel alpha values.
            let mask = dynamic.color().has_alpha().then(|| {
                let alphas: Vec<_> = dynamic.pixels().map(|p| (p.2).0[3]).collect();
                compress_to_vec_zlib(&alphas, level)
            });

            (Filter::FlateDecode, Cow::Owned(encoded), mask)
        }
        _ => panic!("unsupported image format"),
    };

    let mut chunk = Chunk::new();
    let mut image = chunk.image_xobject(image_ref, &encoded);
    image.filter(filter);
    image.width(dynamic.width() as i32);
    image.height(dynamic.height() as i32);
    image.color_space().device_rgb();
    image.bits_per_component(8);
    if mask.is_some() {
        image.s_mask(mask_ref.unwrap());
    }
    image.finish();

    if let Some(encoded) = &mask {
        let mut s_mask = chunk.image_xobject(mask_ref.unwrap(), encoded);
        s_mask.filter(filter);
        s_mask.width(dynamic.width() as i32);
        s_mask.height(dynamic.height() as i32);
        s_mask.color_space().device_gray();
        s_mask.bits_per_component(8);
        s_mask.finish();
    }
    chunk
}

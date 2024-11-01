use image::GenericImageView;
use miniz_oxide::deflate::{compress_to_vec_zlib, CompressionLevel};
use pdf_writer::Ref;
use pdf_writer::{Chunk, Filter, Finish};

use std::borrow::Cow;

pub(crate) struct PdfGlobalInfo {
    page_refs: Vec<Ref>,
    page_tree_ref: Ref,
    alloc_ref: Ref,
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

impl PdfGlobalInfo {
    pub fn new(pdf: &mut pdf_writer::Pdf, n_pages: u32) -> Self {
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

    pub fn page_ref_allocator(&self, page_idx: u32) -> PdfRefAllocator {
        PdfRefAllocator {
            counter: self.page_refs[page_idx as usize].get(),
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

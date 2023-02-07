use std::{
    hash::{BuildHasher, Hash, Hasher},
    mem::ManuallyDrop,
    ops::{Bound, RangeBounds},
};

use image::ImageEncoder;
use rand::{Rng, SeedableRng};

const COLORS: [Color; 4] = [
    Color { r: 255, g: 0, b: 0 },
    Color { r: 0, g: 0, b: 255 },
    Color {
        r: 255,
        g: 255,
        b: 0,
    },
    Color {
        r: 255,
        g: 255,
        b: 255,
    },
];

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(C)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl Color {
    pub fn new(r: u8, g: u8, b: u8) -> Self {
        Self { r, g, b }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct Colors {
    colors: Vec<Color>,
}

#[cfg(test)]
#[doc(hidden)]
const __DUMMY: () = {
    use std::mem::{align_of, size_of};
    assert!(
        size_of::<Color>() == 3,
        "Color is not the right size (expected 3 bytes)"
    );
    assert!(
        align_of::<Color>() == 1,
        "Color is not packed (expected alignment of 1)"
    );
};

impl Colors {
    pub fn new() -> Self {
        Self { colors: vec![] }
    }

    pub fn push(&mut self, color: Color) {
        self.colors.push(color);
    }

    pub fn as_bytes(&self) -> Vec<u8> {
        self.colors.iter().flat_map(|c| [c.r, c.g, c.b]).collect()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Canvas {
    buffer: Vec<Color>,
    width: u32,
    height: u32,
}

impl Canvas {
    pub(crate) fn paint<X: RangeBounds<u32>, Y: RangeBounds<u32>>(
        &mut self,
        x: X,
        y: Y,
        color: Color,
    ) {
        let x = match x.start_bound() {
            Bound::Included(x) => *x,
            Bound::Excluded(x) => x + 1,
            Bound::Unbounded => 0,
        }..match x.end_bound() {
            Bound::Included(x) => (x + 1).min(self.width),
            Bound::Excluded(x) => (*x).min(self.width),
            Bound::Unbounded => self.width,
        };
        let y = match y.start_bound() {
            Bound::Included(y) => *y,
            Bound::Excluded(y) => y + 1,
            Bound::Unbounded => 0,
        }..match y.end_bound() {
            Bound::Included(y) => (y + 1).min(self.height),
            Bound::Excluded(y) => (*y).min(self.height),
            Bound::Unbounded => self.height,
        };
        for row in y {
            for column in x.clone() {
                let index = (row * self.width + column) as usize;
                self.buffer[index] = color;
            }
        }
    }
}

#[cfg_attr(target_family = "wasm", wasm_bindgen::prelude::wasm_bindgen)]
pub fn generate(
    seed: &[u8],
    width: u32,
    height: u32,
    max_depth: u32,
    sep_width: u32,
    min_middle: u32,
    max_middle: u32,
) -> String {
    let mut hasher = ahash::RandomState::with_seeds(1, 2, 3, 4).build_hasher();
    seed.hash(&mut hasher);
    let seed = hasher.finish();
    let mut rng = rand::rngs::StdRng::seed_from_u64(seed);

    let mut canvas = Canvas {
        buffer: vec![Color { r: 0, g: 0, b: 0 }; (width * height) as usize],
        width,
        height,
    };
    generate_part_on_x(
        &mut canvas,
        &mut rng,
        max_depth,
        0..width,
        0..height,
        sep_width,
        min_middle,
        max_middle,
        &COLORS,
    );
    //SAFETY: The Color is repr(C)
    let buffer = unsafe {
        let b = ManuallyDrop::new(canvas.buffer);
        let (ptr, len, cap) = (b.as_ptr(), b.len(), b.capacity());
        Vec::from_raw_parts(ptr as *mut u8, len * 3, cap * 3)
    };
    let mut img_out =
        base64::write::EncoderStringWriter::new(&base64::engine::general_purpose::STANDARD);
    image::codecs::png::PngEncoder::new(&mut img_out)
        .write_image(&buffer, width, height, image::ColorType::Rgb8)
        .unwrap();
    img_out.into_inner()
}

fn generate_part_on_y(
    canvas: &mut Canvas,
    rng: &mut (impl Rng + ?Sized),
    max_depth: u32,
    x_range: std::ops::Range<u32>,
    y_range: std::ops::Range<u32>,
    sep_width: u32,
    min_middle: u32,
    max_middle: u32,
    colors: &[Color],
) {
    if rng.gen_ratio(1, max_depth + 2) || max_depth == 0 {
        canvas.paint(x_range, y_range, colors[rng.gen_range(0..colors.len())]);
        return;
    }
    let middle_percent = rng.gen_range(min_middle..max_middle);
    let middle_pixel = y_range.start + (y_range.end - y_range.start) * middle_percent / 100;
    let end_top = middle_pixel - sep_width;
    let start_bottom = middle_pixel + sep_width;
    canvas.paint(
        x_range.clone(),
        end_top..start_bottom,
        Color { r: 0, g: 0, b: 0 },
    );
    if y_range.start < end_top {
        generate_part_on_x(
            canvas,
            rng,
            max_depth - 1,
            x_range.clone(),
            y_range.start..end_top,
            sep_width,
            min_middle,
            max_middle,
            colors,
        );
    }
    if start_bottom < y_range.end {
        generate_part_on_x(
            canvas,
            rng,
            max_depth - 1,
            x_range,
            start_bottom..y_range.end,
            sep_width,
            min_middle,
            max_middle,
            colors,
        );
    }
}

fn generate_part_on_x(
    canvas: &mut Canvas,
    rng: &mut (impl Rng + ?Sized),
    max_depth: u32,
    x_range: std::ops::Range<u32>,
    y_range: std::ops::Range<u32>,
    sep_width: u32,
    min_middle: u32,
    max_middle: u32,
    colors: &[Color],
) {
    if rng.gen_ratio(1, max_depth + 2) || max_depth == 0 {
        canvas.paint(x_range, y_range, colors[rng.gen_range(0..colors.len())]);
        return;
    }
    let middle_percent = rng.gen_range(min_middle..max_middle);
    let middle_pixel = x_range.start + (x_range.end - x_range.start) * middle_percent / 100;
    let end_left = middle_pixel - sep_width;
    let start_right = middle_pixel + sep_width;
    canvas.paint(
        end_left..start_right,
        y_range.clone(),
        Color { r: 0, g: 0, b: 0 },
    );
    if x_range.start < end_left {
        generate_part_on_y(
            canvas,
            rng,
            max_depth - 1,
            x_range.start..end_left,
            y_range.clone(),
            sep_width,
            min_middle,
            max_middle,
            colors,
        );
    }
    if start_right < x_range.end {
        generate_part_on_y(
            canvas,
            rng,
            max_depth - 1,
            start_right..x_range.end,
            y_range,
            sep_width,
            min_middle,
            max_middle,
            colors,
        );
    }
}

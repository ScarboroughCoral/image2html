mod utils;

use wasm_bindgen::prelude::*;
extern crate image;

use image::{ImageFormat}

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
extern {
    fn alert(s: &str);
}

#[wasm_bindgen]
pub fn greet() {
    alert("Hello, image2html!");
}

#[wasm_bindgen]
pub fn translate_image(data: Vec<u8>) {
    image::load_from_memory_with_format(data, ImageFormat.PNG)
}
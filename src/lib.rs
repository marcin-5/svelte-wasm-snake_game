mod utils;

use wasm_bindgen::prelude::*;

#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
struct World {
    width: usize,
}

#[wasm_bindgen]
impl World {
    pub fn new(width: usize) -> World {
        World { width: 8 }
    }
    pub fn width(&self) -> usize {
        self.width
    }
}


use wasm_bindgen::prelude::*;

#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
pub struct World {
    pub width: usize
}

#[wasm_bindgen]
impl World {
    pub fn new() -> World{
        World {
            width: 8
        }
    }

    pub fn width(&self) -> usize {
        self.width
    }
}

// #[wasm_bindgen]
// pub fn greet(name: &str){
//     alert(name);
// }

// #[wasm_bindgen]
// extern {
//     pub fn alert(s: &str);
// }
//wasm-pack build --target web
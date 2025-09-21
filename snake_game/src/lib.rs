use wasm_bindgen::prelude::*;

#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

struct SnakeCells(usize);

struct Snake {
    body: Vec<SnakeCells>,
}

impl Snake {
    fn new(spawn_index: usize) -> Snake {
        Snake {
            body: vec![SnakeCells(spawn_index)],
        }
    }
}

#[wasm_bindgen]
pub struct World {
    width: usize,
    snake: Snake,
}

#[wasm_bindgen]
impl World {
    pub fn new() -> World {
        World {
            width: 8,
            snake: Snake::new(10),
        }
    }

    pub fn width(&self) -> usize {
        self.width
    }

    pub fn snake_head_idx(&self) -> usize {
        self.snake.body[0].0
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

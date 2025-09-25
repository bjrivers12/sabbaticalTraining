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
    size: usize,
    snake: Snake,
}

#[wasm_bindgen]
impl World {
    pub fn new() -> World {
        let width: usize = 8;
        World {
            width,
            size: width * width,
            snake: Snake::new(10),
        }
    }

    pub fn width(&self) -> usize {
        self.width
    }

    pub fn snake_head_idx(&self) -> usize {
        self.snake.body[0].0
    }

    pub fn update(&mut self){
        let snake_idx = self.snake_head_idx();
        self.snake.body[0].0 = (snake_idx + self.size - 1) % self.size;
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
//cargo watch -s "wasm-pack build --target web --out-dir www/pkg --dev"

//cargo watch -s "wasm-pack build --target web --out-dir www/pkg --dev && touch reload.trigger"

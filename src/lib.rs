use wasm_bindgen::prelude::*;
use wee_alloc::WeeAlloc;

//Diminuir o tamanho do binário
#[global_allocator]
static ALLOC: WeeAlloc = WeeAlloc::INIT;

struct SnakeCell(usize);

/* Struct Snake*/
struct Snake {
    body: Vec<SnakeCell>,
}

impl Snake {
    fn new(spaw_index: usize) -> Snake {
        Snake {
            body: vec![SnakeCell(spaw_index)],
        }
    }
}
/* Struct Snake*/

/* Struct World*/

//wasmbindgen é uma macro que permite a comunicação entre o código Rust e o JavaScript
#[wasm_bindgen]
pub struct World {
    pub width: usize,
    snake: Snake,
    size: usize
}

#[wasm_bindgen]
impl World {
    pub fn new(width: usize, snake_idx: usize) -> World {
        World {
            width,
            size: width * width,
            snake: Snake::new(snake_idx),
        }
    }

    pub fn update(&mut self){
        let snake_idx = self.snake_head_idx();
        self.snake.body[0].0 = (snake_idx + 1) % self.size;
    }

    pub fn width(&self) -> usize {
        self.width
    }

    pub fn snake_head_idx(&self) -> usize {
        self.snake.body[0].0
    }
}
/* Struct World*/

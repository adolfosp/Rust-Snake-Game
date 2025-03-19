use wasm_bindgen::prelude::*;
use wee_alloc::WeeAlloc;

//Diminuir o tamanho do binário
#[global_allocator]
static ALLOC: WeeAlloc = WeeAlloc::INIT;

#[derive(PartialEq)]
enum Direction {
    Up = 0,
    Down = 1,
    Left = 2,
    Right = 3,
}

struct SnakeCell(usize);

/* Struct Snake*/
struct Snake {
    body: Vec<SnakeCell>,
    direction: Direction,
}

impl Snake {
    fn new(spaw_index: usize) -> Snake {
        Snake {
            body: vec![SnakeCell(spaw_index)],
            direction: Direction::Right,
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
    size: usize,
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

    pub fn width(&self) -> usize {
        self.width
    }

    pub fn snake_head_idx(&self) -> usize {
        self.snake.body[0].0
    }

    pub fn update(&mut self) {
        let snake_idx = self.snake_head_idx();
        let (row, col) = self.index_to_cell(snake_idx);
        let (row, col) = match self.snake.direction {
            Direction::Right => (row, (col + 1) % self.width),
            Direction::Left => (row, (col - 1) % self.width),
            Direction::Up => ((row - 1) % self.width, col),
            Direction::Down => ((row + 1) % self.width, col),
        };

        let next_idx = self.cell_to_index(row, col);
        self.set_snake_head(next_idx);
   
    }

    fn set_snake_head(&mut self, idx: usize) {
        self.snake.body[0].0 = idx;
    }

    fn index_to_cell(&self, idx: usize) -> (usize, usize) {
        (idx / self.width, idx % self.width)
    }

    fn cell_to_index(&self, row: usize, col: usize) -> usize {
        (row * self.width) + col
    }
}
/* Struct World*/

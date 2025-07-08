use wasm_bindgen::prelude::*;
use wee_alloc::WeeAlloc;

// Diminuir o tamanho do binário
#[global_allocator]
static ALLOC: WeeAlloc = WeeAlloc::INIT;

// Definição da função externa em JavaScript para gerar números aleatórios
#[wasm_bindgen(module = "/www/utils/rnd.js")]
extern {
    fn rnd(max: usize) -> usize;
}

// Definição do Enum para as direções do movimento da cobra
#[wasm_bindgen]
#[derive(PartialEq)]
pub enum Direction {
    Up,
    Right,
    Down,
    Left
}

// Definição do Enum para o status do jogo
#[wasm_bindgen]
#[derive(Clone, Copy)]
pub enum GameStatus {
    Won,
    Lost,
    Played,
}

// Definição de uma célula da cobra com um índice
#[derive(PartialEq, Clone, Copy)]
pub struct SnakeCell(usize);

// Estrutura que representa a cobra, contendo seu corpo e direção
struct Snake {
    body: Vec<SnakeCell>,
    direction: Direction,
}

impl Snake {
    // Função para inicializar a cobra com um tamanho e uma posição inicial
    fn new(spawn_index: usize, size: usize) -> Snake {
        let mut body = vec!();

        for i in 0..size {
            body.push(SnakeCell(spawn_index - i));
        }

        Snake {
            body,
            direction: Direction::Right,
        }
    }
}

// Estrutura que representa o mundo do jogo, com a cobra, a célula de recompensa e o status do jogo
#[wasm_bindgen]
pub struct World {
    width: usize,
    size: usize,
    snake: Snake,
    next_cell: Option<SnakeCell>,
    reward_cell: Option<usize>,
    status: Option<GameStatus>,
    points: usize,
}

#[wasm_bindgen]
impl World {
    // Função para inicializar o mundo com uma largura e a posição inicial da cobra
    pub fn new(width: usize, snake_idx: usize) -> World {

        let snake = Snake::new(snake_idx, 3);
        let size = width * width;

        World {
            width,
            size,
            reward_cell: World::gen_reward_cell(size, &snake.body),
            snake,
            next_cell: None,
            status: None,
            points: 0,
        }
    }

    // Função para gerar a célula de recompensa, evitando que seja sobreposta pela cobra
    fn gen_reward_cell(max: usize, snake_body: &Vec<SnakeCell>) -> Option<usize> {
        let mut reward_cell;

        loop {
          reward_cell = rnd(max);
          if !snake_body.contains(&SnakeCell(reward_cell)) { break; }
        }

        Some(reward_cell)
    }

    // Função para retornar a largura do mundo
    pub fn width(&self) -> usize {
        self.width
    }

    // Função para retornar a pontuação do jogador
    pub fn points(&self) -> usize {
        self.points
    }

    // Função para retornar a célula de recompensa
    pub fn reward_cell(&self) -> Option<usize> {
        self.reward_cell
    }

    // Função para retornar o índice da cabeça da cobra
    pub fn snake_head_idx(&self) -> usize {
       self.snake.body[0].0
    }

    // Função para iniciar o jogo
    pub fn start_game(&mut self) {
        self.status = Some(GameStatus::Played);
    }

    // Função para obter o status atual do jogo
    pub fn game_status(&self) -> Option<GameStatus> {
        self.status
    }

    // Função para retornar uma mensagem com o status atual do jogo
    pub fn game_status_text(&self) -> String {
        match self.status {
            Some(GameStatus::Won) => String::from("You have won!"),
            Some(GameStatus::Lost) => String::from("You have lost!"),
            Some(GameStatus::Played) => String::from("Playing"),
            None => String::from("No Status"),
        }
    }

    // Função para alterar a direção da cobra
    pub fn change_snake_dir(&mut self, direction: Direction) {
        let next_cell = self.gen_next_snake_cell(&direction);

        if self.snake.body[1].0 == next_cell.0 { return; }

        self.next_cell = Some(next_cell);
        self.snake.direction = direction;
    }

    // Função para obter o comprimento da cobra
    pub fn snake_length(&self) -> usize {
        self.snake.body.len()
    }

    // Função para acessar diretamente as células da cobra
    pub fn snake_cells(&self) -> *const SnakeCell {
        self.snake.body.as_ptr()
    }

    // Função que atualiza o estado do jogo, avançando a cobra e verificando as condições
    pub fn step(&mut self) {
        match self.status {
            Some(GameStatus::Played) => {
                let temp = self.snake.body.clone();

                match self.next_cell {
                    Some(cell) => {
                        self.snake.body[0] = cell;
                        self.next_cell = None;
                    },
                    None => {
                        self.snake.body[0] = self.gen_next_snake_cell(&self.snake.direction);
                    }
                }

                for i in 1..self.snake_length() {
                    self.snake.body[i] = SnakeCell(temp[i - 1].0);
                }

                // Verifica se a cobra colidiu com ela mesma
                if self.snake.body[1..self.snake_length()].contains(&self.snake.body[0]) {
                    self.status = Some(GameStatus::Lost)
                }

                // Verifica se a cobra atingiu a célula de recompensa
                if self.reward_cell == Some(self.snake_head_idx()) {
                    if self.snake_length() < self.size {
                        self.points += 1;
                        self.reward_cell = World::gen_reward_cell(self.size, &self.snake.body);
                    } else {
                        self.reward_cell = None;
                        self.status = Some(GameStatus::Won)
                    }

                    self.snake.body.push(SnakeCell(self.snake.body[1].0));
                }
            },
            _ => {}
        }
    }

    // Função para calcular a próxima célula com base na direção da cobra
    fn gen_next_snake_cell(&self, direction: &Direction) -> SnakeCell {
        let snake_idx = self.snake_head_idx();
        let row = snake_idx / self.width;

        return match direction {
            Direction::Right => {
                let treshold = (row + 1) * self.width;
                if snake_idx + 1 == treshold {
                    SnakeCell(treshold - self.width)
                } else {
                    SnakeCell(snake_idx + 1)
                }
            },
            Direction::Left => {
                let treshold = row * self.width;
                if snake_idx == treshold {
                    SnakeCell(treshold + (self.width - 1))
                } else {
                    SnakeCell(snake_idx - 1)
                }
            },
            Direction::Up => {
                let treshold = snake_idx - (row * self.width);
                if snake_idx == treshold {
                    SnakeCell((self.size - self.width) + treshold)
                } else {
                    SnakeCell(snake_idx - self.width)
                }
            },
            Direction::Down => {
                let treshold = snake_idx + ((self.width - row) * self.width);
                if snake_idx + self.width == treshold {
                    SnakeCell(treshold - ((row + 1) * self.width))
                } else {
                    SnakeCell(snake_idx + self.width)
                }
            },
        };
    }

}

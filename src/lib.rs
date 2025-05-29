use wasm_bindgen::prelude::*;

// Use a more memory-efficient allocator for WebAssembly
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
#[derive(PartialEq)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Clone, Copy)]
struct SnakeCell(usize);

struct Snake {
    body: Vec<SnakeCell>,
    direction: Direction,
}

impl Snake {
    fn new(spawn_index: usize, length: usize) -> Snake {
        let mut body = vec!();

        for i in 0..length {
            body.push(SnakeCell(spawn_index - i));
        }

        Snake {
            body,
            direction: Direction::Down,
        }
    }
}

#[wasm_bindgen]
pub struct World {
    size: usize,
    grid_capacity: usize,
    snake: Snake,
    next_cell: Option<SnakeCell>,
    reward_cell: usize,
}

#[wasm_bindgen]
impl World {
    /// Creates a new world with the specified grid size
    ///
    /// # Arguments
    ///
    /// * `grid_size` - The size of the grid (number of cells in each dimension)
    /// * `snake_idx` - The initial spawn index for the snake
    ///
    /// # Returns
    ///
    /// A new World instance with the specified size and snake position
    pub fn new(grid_size: usize, snake_idx: usize) -> World {
        // Ensure minimum size for stability
        let size = if grid_size < 2 { 2 } else { grid_size };

        World {
            size,
            grid_capacity: size * size,
            snake: Snake::new(snake_idx, 3),
            next_cell: None,
            reward_cell: 10,
        }
    }

    pub fn width(&self) -> usize {
        self.size
    }

    pub fn height(&self) -> usize {
        self.size
    }

    pub fn reward_cell(&self) -> usize {
        self.reward_cell
    }

    pub fn snake_head_idx(&self) -> usize {
        self.snake.body[0].0
    }

    pub fn set_snake_direction(&mut self, direction: Direction) {
        let next_cell = self.gen_next_snake_cell(&direction);
        if self.snake.body[1].0 == next_cell.0 { return; }

        self.next_cell = Some(next_cell);
        self.snake.direction = direction;
    }

    pub fn snake_cells(&self) -> Vec<usize> {
        self.snake.body.iter().map(|cell| cell.0).collect()
    }

    fn gen_next_snake_cell(&self, direction: &Direction) -> SnakeCell {
        let snake_idx = self.snake_head_idx();
        let row = snake_idx / self.size;
        return match direction {
            Direction::Up => {
                let threshold = snake_idx - (row * self.size);
                if snake_idx == threshold {
                    SnakeCell((self.grid_capacity - self.size) + threshold)
                } else {
                    SnakeCell(snake_idx - self.size)
                }
            }
            Direction::Down => {
                let threshold = snake_idx + ((self.size - row) * self.size);
                if snake_idx + self.size == threshold {
                    SnakeCell(threshold - ((row + 1) * self.size))
                } else {
                    SnakeCell(snake_idx + self.size)
                }
            }
            Direction::Right => {
                let threshold = (row + 1) * self.size;
                if snake_idx + 1 == threshold {
                    SnakeCell(threshold - self.size)
                } else {
                    SnakeCell(snake_idx + 1)
                }
            }
            Direction::Left => {
                let threshold = row * self.size;
                if snake_idx == threshold {
                    SnakeCell(threshold + (self.size - 1))
                } else {
                    SnakeCell(snake_idx - 1)
                }
            }
        };
    }

    pub fn step(&mut self) {
        let temp = self.snake.body.clone();
        match self.next_cell {
            Some(cell) => {
                self.snake.body[0] = cell;
                self.next_cell = None;
            }
            None => {
                self.snake.body[0] = self.gen_next_snake_cell(&self.snake.direction);
            }
        }

        let len = self.snake.body.len();
        for i in 1..len {
            self.snake.body[i] = SnakeCell(temp[i - 1].0);
        }
    }
}

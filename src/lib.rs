use wasm_bindgen::prelude::*;

// Use a more memory-efficient allocator for WebAssembly
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[derive(PartialEq)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

struct SnakeCell(usize);
struct Snake {
    body: Vec<SnakeCell>,
    direction: Direction,
}

impl Snake {
    fn new(spawn_index: usize) -> Snake {
        Snake {
            body: vec![SnakeCell(spawn_index)],
            direction: Direction::Right,
        }
    }
}

#[wasm_bindgen]
pub struct World {
    size: usize,
    grid_capacity: usize,
    snake: Snake,
}

#[wasm_bindgen]
impl World {
    /// Creates a new world with the specified grid size
    ///
    /// # Arguments
    /// * `grid_size` - The size of the grid (number of cells in each dimension)
    ///
    /// # Returns
    /// A new World instance with the specified size
    pub fn new(grid_size: usize, snake_idx: usize) -> World {
        // Ensure minimum size for stability
        let size = if grid_size < 2 { 2 } else { grid_size };

        World { size, grid_capacity: size * size, snake: Snake::new(snake_idx) }
    }

    pub fn width(&self) -> usize {
        self.size
    }

    pub fn height(&self) -> usize {
        self.size
    }

    pub fn snake_head_idx(&self) -> usize {
        self.snake.body[0].0
    }

    pub fn update(&mut self) {
        let snake_idx = self.snake_head_idx();
        let row = snake_idx / self.size;

        if self.snake.direction == Direction::Left {
            let next_col = (snake_idx - 1) % self.size;
            self.snake.body[0].0 = row * self.size + next_col;
        }
        if self.snake.direction == Direction::Right {
            let next_col = (snake_idx + 1) % self.size;
            self.snake.body[0].0 = row * self.size + next_col;
        }
    }
}

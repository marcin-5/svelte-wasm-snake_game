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
            direction: Direction::Down,
        }
    }
}

#[wasm_bindgen]
pub struct World {
    size: usize,
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

        World { size, snake: Snake::new(snake_idx) }
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

    fn index_to_cell(&self, idx: usize) -> (usize, usize) {
        (idx / self.size, idx % self.size)
    }

    fn cell_to_index(&self, row: usize, col: usize) -> usize {
        row * self.size + col
    }

    pub fn update(&mut self) {
        let snake_idx = self.snake_head_idx();
        let (row, col) = self.index_to_cell(snake_idx);

        let next_position = match self.snake.direction {
            Direction::Up => {
                let next_row = (row + self.size - 1) % self.size;
                self.cell_to_index(next_row, col)
            }
            Direction::Down => {
                let next_row = (row + 1) % self.size;
                self.cell_to_index(next_row, col)
            }
            Direction::Left => {
                let next_col = (col + self.size - 1) % self.size;
                self.cell_to_index(row, next_col)
            }
            Direction::Right => {
                let next_col = (col + 1) % self.size;
                self.cell_to_index(row, next_col)
            }
        };

        self.snake.body[0].0 = next_position;
    }
}

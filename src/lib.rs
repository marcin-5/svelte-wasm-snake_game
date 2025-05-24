use wasm_bindgen::prelude::*;

// Use a more memory-efficient allocator for WebAssembly
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

struct SnakeCell(usize);
struct Snake {
    body: Vec<SnakeCell>,
}

impl Snake {
    fn new(spawn_index: usize) -> Snake {
        Snake {
            body: vec![SnakeCell(spawn_index)],
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
    pub fn new(grid_size: usize) -> World {
        // Ensure minimum size for stability
        let size = if grid_size < 2 { 2 } else { grid_size };

        World { size, snake: Snake::new(size / 2) }
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
}

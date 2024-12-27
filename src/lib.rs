use wasm_bindgen::prelude::*;
use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement, console, window};

// enum with ALIVE, ZOMBIE, DEAD
#[wasm_bindgen]
#[derive(Clone)]
#[derive(PartialEq)]
pub enum CellState {
    ALIVE,
    ZOMBIE,
    DEAD
}

#[wasm_bindgen]
#[derive(Clone)]
pub struct GameOfLifeCell {
    x: usize, // position in the grid (from 0 to WIDTH)
    y: usize,
    zombie_score: i8,
    state: CellState
}

#[wasm_bindgen]
pub struct GameOfLife {
    cells: Vec<GameOfLifeCell>,
    width: usize,
    height: usize,
    cell_size: usize
}

#[wasm_bindgen]
impl GameOfLife {
    // setup all the cells in the grid (we want a checkerboard pattern as the base)
    #[wasm_bindgen(constructor)]
    pub fn new(width: usize, height: usize, cell_size: usize) -> Self {
        let mut cells = Vec::with_capacity(width * height);
        for y in 0..height {
            for x in 0..width {
                let state = if (x + y) % 2 == 0 {
                    CellState::ALIVE
                } else {
                    CellState::DEAD
                };
                cells.push(GameOfLifeCell {
                    x,
                    y,
                    zombie_score: -1,
                    state
                });
            }
        }
        Self { cells, width, height, cell_size }
    }

    pub fn resetCells (&mut self) {
        let mut cells = Vec::with_capacity(self.width * self.height);
        for y in 0..self.height {
            for x in 0..self.width {
                let state = if (x + y) % 2 == 0 {
                    CellState::ALIVE
                } else {
                    CellState::DEAD
                };
                cells.push(GameOfLifeCell {
                    x,
                    y,
                    zombie_score: -1,
                    state
                });
            }
        }
        self.cells = cells;
        console::log_1(&JsValue::from_str(&format!("== Reset the cells")));
    }

    pub fn life(&mut self, iteration: u8, canvas: HtmlCanvasElement) {
        self.resetCells();

        for i in 0..iteration {
            self.iterate();    
            console::log_1(&JsValue::from_str(&format!("Iteration: {}", i)));
            self.render(&canvas);
        }
    }

    fn get_cell(&self, x: usize, y: usize) -> Option<&GameOfLifeCell> {
        self.cells.get(y * &self.width + x)
    }

    fn iterate(&mut self) {
        /*
        If the cell is Alive and has fewer than two live neighbors it dies and becomes a Zombie(3)
        If the cell is Alive with two or three live neighbors it survives and stays Alive
        If the cell is Alive and has more than three live neighbors it dies and becomes a Zombie(3)
        If the cell is Dead or a Zombie(x) and has exactly three live neighbors, it becomes Alive
        If the cell is Zombie(x) and has not exactly three live neighbors, it becomes a Zombie(x-1)
        If the cell is a Zombie(0) and it does not have exactly three live neighbors, it becomes Dead
        */

        let mut new_cells = self.cells.clone();

        for i in 0..self.height {
            for j in 0..self.width {
                let current_cell = self.get_cell(i, j).unwrap();
                let mut alive_neighbors = 0;

                // go through all the possible neighbors of the current cell
                for x in -1..=1 {
                    for y in -1..=1 {
                        if x == 0 && y == 0 {
                            continue;
                        }

                        if i + x as usize >= self.width || j + y as usize >= self.height {
                            continue;
                        }

                        let neighbor = self.get_cell(i + x as usize, j + y as usize);
                        if let Some(neighbor) = neighbor {
                            if neighbor.state == CellState::ALIVE {
                                alive_neighbors += 1;
                            }
                        }
                    }
                }

                let current_cell_idx = j * self.width + i;
                // apply the rules
                match current_cell.state {
                    CellState::ALIVE => {
                        if alive_neighbors < 2 || alive_neighbors > 3 {
                            new_cells[current_cell_idx].state = CellState::ZOMBIE;
                            new_cells[current_cell_idx].zombie_score = 3;
                        }
                    },
                    CellState::ZOMBIE => {
                        if alive_neighbors != 3 && current_cell.zombie_score > 0 {
                            new_cells[current_cell_idx].zombie_score -= 1;
                        } else if alive_neighbors != 3 && current_cell.zombie_score == 0 {
                            new_cells[current_cell_idx].state = CellState::DEAD;
                        } else {
                            new_cells[current_cell_idx].state = CellState::ALIVE;
                            new_cells[current_cell_idx].zombie_score = -1;
                        }
                    },
                    CellState::DEAD => {
                        if alive_neighbors == 3 {
                            new_cells[current_cell_idx].state = CellState::ALIVE;
                            new_cells[current_cell_idx].zombie_score = -1;
                        }
                    }
                }
            }
        }

        // set the new_cells as current cells and render
        self.cells = new_cells;
    }

    // draw the grid on the canvas
    fn render(&self, canvas: &HtmlCanvasElement) -> Result<(), JsValue> {
        let window = window().expect("no global `window` exists");

        let pixel_ratio = window.device_pixel_ratio();
        // Set the canvas size in CSS pixels
        let canvas_width = self.width * self.cell_size;
        let canvas_height = self.height * self.cell_size;

        canvas.set_attribute("style", &format!("width: {}px; height: {}px;", canvas_width, canvas_height))?;
        
        // Scale the canvas dimensions
        canvas.set_width((canvas_width as f64 * pixel_ratio) as u32);
        canvas.set_height((canvas_height as f64 * pixel_ratio) as u32);

        let context = canvas
            .get_context("2d")?
            .unwrap()
            .dyn_into::<CanvasRenderingContext2d>()?;

        // Scale the context
        context.scale(pixel_ratio, pixel_ratio)?;

        context.clear_rect(0.0, 0.0, (self.width * self.cell_size) as f64, (self.height * self.cell_size) as f64);
        for cell in &self.cells {
            context.begin_path();
            context.rect(
                cell.x as f64 * self.cell_size as f64,
                cell.y as f64 * self.cell_size as f64,
                self.cell_size as f64,
                self.cell_size as f64
            );
            context.set_fill_style(&JsValue::from_str(match cell.state {
                CellState::ALIVE => "black",
                CellState::ZOMBIE => match cell.zombie_score {
                    3 => "#3a5a40",
                    2 => "#588157",
                    1 => "#a3b18a",
                    _ => "#dad7cd"
                },
                CellState::DEAD => "white"
            }));
            context.fill();
        }

        Ok(())
    }
}
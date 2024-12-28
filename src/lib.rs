use wasm_bindgen::prelude::*;
use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement, console, window};

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
    x: usize,
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

    pub fn reset_cells (&mut self) {
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
        self.reset_cells();

        for i in 0..iteration {
            self.iterate();    
            console::log_1(&JsValue::from_str(&format!("Iteration: {}", i)));
            let _ = self.render(&canvas);
        }
    }

    /*
        this is an optional function i just wanted to try :) instead of getting an iteration parameter and computing all the steps each time, i just append one iteration and render
        => see interactive.html
     */
    pub fn append_life_iteration(&mut self, canvas: HtmlCanvasElement) {
        self.iterate();    
        console::log_1(&JsValue::from_str(&format!("Iteration")));
        let _ = self.render(&canvas);
    }

    fn iterate(&mut self) {
        let mut new_cells = self.cells.clone();

        for i in 0..self.height {
            for j in 0..self.width {
                let current_cell = self.get_cell(j, i).unwrap();
                let alive_neighbors = self.get_alive_neighbors(j, i);
                let current_cell_idx = self.get_current_cell_index(j, i);

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

        self.cells = new_cells;
    }

    // draw the grid on the canvas
    fn render(&self, canvas: &HtmlCanvasElement) -> Result<(), JsValue> {
        let window = window().expect("no global `window` exists");

        let pixel_ratio = window.device_pixel_ratio();
        let canvas_width = self.width * self.cell_size;
        let canvas_height = self.height * self.cell_size;

        canvas.set_attribute("style", &format!("width: {}px; height: {}px;", canvas_width, canvas_height))?;
        canvas.set_width((canvas_width as f64 * pixel_ratio) as u32);
        canvas.set_height((canvas_height as f64 * pixel_ratio) as u32);

        let context = canvas
            .get_context("2d")?
            .unwrap()
            .dyn_into::<CanvasRenderingContext2d>()?;

        context.scale(pixel_ratio, pixel_ratio)?;

        context.clear_rect(0.0, 0.0, (self.width * self.cell_size) as f64, (self.height * self.cell_size) as f64);
        for cell in &self.cells {
            context.begin_path();
            context.rect(
                (cell.x as f64 * self.cell_size as f64),
                (cell.y as f64 * self.cell_size as f64),
                self.cell_size as f64,
                self.cell_size as f64
            );

            /* 
            let rgb_alpha_value = &(3.0/cell.zombie_score as f32);
            let rgb_full =  format!("{}{}{}",  "rgba(58, 90, 64, ", rgb_alpha_value,  ")");
            */
            
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

    fn get_cell(&self, row: usize, column: usize) -> Option<&GameOfLifeCell> {
        self.cells.get(column * &self.width + row)
    }
    
    fn get_current_cell_index(&self, row: usize, column: usize) -> usize {
        (column * self.width + row) as usize
    }

    fn get_alive_neighbors(&self, row: usize, column: usize) -> i16 {
        let mut alive_neighbors = 0;
        for x in -1..=1 {
            for y in -1..=1 {
                if x == 0 && y == 0 {
                    continue;
                }

                if row as isize + x < 0 || column as isize + y < 0 || row + x as usize >= self.width || column + y as usize >= self.height {
                    continue;
                }

                let neighbor = self.get_cell(row + x as usize, column + y as usize);
                if let Some(neighbor) = neighbor {
                    if neighbor.state == CellState::ALIVE {
                        alive_neighbors += 1;
                    }
                }
            }
        }

        alive_neighbors
    }
}
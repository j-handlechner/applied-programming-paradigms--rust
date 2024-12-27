use wasm_bindgen::prelude::*;
use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement, console, window};

// enum with ALIVE, ZOMBIE, DEAD
#[wasm_bindgen]
pub enum CellState {
    ALIVE,
    ZOMBIE,
    DEAD
}

#[wasm_bindgen]
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

    pub fn life(&self, iteration: u8, canvas: HtmlCanvasElement) {
        // console log the iteration to js 
        console::log_1(&JsValue::from_str(&format!("Iteration: {}", iteration)));
        self.render(&canvas);
    }

    fn get_cell(&self, x: usize, y: usize) -> Option<&GameOfLifeCell> {
        self.cells.get(y * &self.width + x)
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
                CellState::ZOMBIE => "red",
                CellState::DEAD => "white"
            }));
            context.fill();
        }

        Ok(())
    }
}
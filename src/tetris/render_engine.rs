use std::time::Duration;

use crate::tetris::{tetronimo::Tetromino, tetronimo::Position, Tetris};

pub trait RenderEngine {
    fn init(&self);
    fn render(&self, tetris: &Tetris);
}

use flo_draw::*;
use flo_canvas::*;

use crate::tetris;

const CELL_SIZE: f32 = 30.;


pub struct FloDrawRenderEngine {
    pub canvas: DrawingTarget,
    width: f32,
    height: f32,
}

const colours: [Color; 4] = [Color::Rgba(1.0, 0.0, 0.0, 1.0), Color::Rgba(0.0, 1.0, 0.0, 1.0), Color::Rgba(0.0, 1.0, 1.0, 1.0), Color::Rgba(1.0, 1.0, 0.0, 1.0)];

impl FloDrawRenderEngine {
    pub fn new(canvas: DrawingTarget) -> Self {
        Self { canvas: canvas, width: CELL_SIZE * tetris::PLAY_FIELD_WIDTH as f32, height: CELL_SIZE * tetris::PLAY_FIELD_HEIGHT as f32 }
    }

    fn draw_current_tetrimino(self: &Self, gc: &mut Vec<Draw>, current_tetromino: &Tetromino) {
        let (x, y) = &current_tetromino.position;
        let mut i: usize = 0;
        for (x_offset, y_offset) in current_tetromino.get_positions() {
            self.draw_cell(gc, (x + x_offset, y + y_offset), colours[i]);
            i += 1;
        }
    }

    fn draw_cell(self: &Self, gc: &mut Vec<Draw>, position: Position, color: Color) {
        let (x_i32 , y_i32) = position;

        let (x, y) = (x_i32 as f32, y_i32 as f32);

        let (screen_x, screen_y) = (x * CELL_SIZE, y * CELL_SIZE);

        self.draw_rect(gc, (screen_x, screen_y), (CELL_SIZE, CELL_SIZE), color);
    }
    
    fn draw_rect(self: &Self, gc: &mut Vec<Draw>, position: (f32, f32), dimensions: (f32, f32), color: Color) {
        let (pos_x, pos_y) = position;
        let (width, height) = dimensions;

        gc.new_path();
        gc.move_to(pos_x, pos_y);
        gc.line_to(pos_x + width, pos_y);
        gc.line_to(pos_x + width, pos_y + height);
        gc.line_to(pos_x, pos_y + height);

        gc.fill_color(color);
        gc.fill();
        gc.stroke();
    }

    fn draw_gridlines(self: &Self, gc: &mut Vec<Draw>) {
        gc.stroke_color(Color::Rgba(0.3, 0.3, 0.3, 1.0)); // Dark gray gridlines
        gc.line_width(1.0);

        // Draw vertical lines
        for x in 0..=tetris::PLAY_FIELD_WIDTH {
            let x_pos = x as f32 * CELL_SIZE;
            gc.new_path();
            gc.move_to(x_pos, 0.0);
            gc.line_to(x_pos, self.height);
            gc.stroke();
        }

        // Draw horizontal lines
        for y in 0..=tetris::PLAY_FIELD_HEIGHT {
            let y_pos = y as f32 * CELL_SIZE;
            gc.new_path();
            gc.move_to(0.0, y_pos);
            gc.line_to(self.width, y_pos);
            gc.stroke();
        }
    }
}

impl RenderEngine for FloDrawRenderEngine {
    fn init(self: &Self) {
        self.canvas.draw(|gc| {
        });
    }
    fn render(self: &Self, tetris: &Tetris) {
        self.canvas.draw(|gc| {
            gc.clear_canvas(Color::Rgba(0.0, 0.0, 0.0, 1.0));
            gc.canvas_height(self.height);
            gc.center_region(0., 0., self.width, self.height);

            

            gc.rect(0., 0., self.width, self.height);
            gc.fill_color(Color::Rgba(0.1, 0.1, 0.1, 1.0));
            gc.fill();

            self.draw_gridlines(gc);

            self.draw_current_tetrimino(gc, tetris.get_current_tetromino());
        });
    }
}
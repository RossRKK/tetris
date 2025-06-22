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
impl FloDrawRenderEngine {
    pub fn new(canvas: DrawingTarget) -> Self {
        Self { canvas: canvas, width: CELL_SIZE * tetris::PLAY_FIELD_WIDTH as f32, height: CELL_SIZE * tetris::PLAY_FIELD_HEIGHT as f32 }
    }

    fn draw_current_tetrimino(self: &Self, gc: &mut Vec<Draw>, current_tetromino: &Tetromino) {
        let (x, y) = &current_tetromino.position;
        for (x_offset, y_offset) in current_tetromino.get_positions() {
            self.draw_cell(gc, (x + x_offset, y + y_offset));
        }
    }

    fn draw_cell(self: &Self, gc: &mut Vec<Draw>, position: Position) {
        let (x_i32 , y_i32) = position;

        let (x, y) = (x_i32 as f32, y_i32 as f32);

        let (screen_x, screen_y) = (x * CELL_SIZE, y * CELL_SIZE);

        self.draw_rect(gc, (screen_x, screen_y), (CELL_SIZE, CELL_SIZE));
    }
    
    fn draw_rect(self: &Self, gc: &mut Vec<Draw>, position: (f32, f32), dimensions: (f32, f32)) {
        let (pos_x, pos_y) = position;
        let (width, height) = dimensions;
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
            gc.fill_color(Color::Rgba(0.0, 0.0, 1.0, 1.0));
            gc.fill();

            self.draw_current_tetrimino(gc, tetris.get_current_tetromino());
        });
    }
}
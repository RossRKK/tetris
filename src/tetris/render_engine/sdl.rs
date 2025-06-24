use crate::tetris::{tetronimo::Tetromino, tetronimo::Position, Tetris, Cell};
use crate::tetris::render_engine::RenderEngine;

use sdl2::pixels::Color as SDL2Color;
use sdl2::rect::Rect;
use sdl2::render::Canvas;
use sdl2::video::Window;

use crate::tetris;

const CELL_SIZE: i32 = 30;

pub struct SDL2RenderEngine {
    canvas: Canvas<Window>,
    width: i32,
    height: i32,
}

impl SDL2RenderEngine {
    pub fn new(canvas: Canvas<Window>) -> Self {
        let width = CELL_SIZE * tetris::PLAY_FIELD_WIDTH as i32;
        let height = CELL_SIZE * tetris::PLAY_FIELD_HEIGHT as i32;
        
        Self {
            canvas: canvas,
            width,
            height,
        }
    }

    fn draw_cell(&mut self, position: Position, color: SDL2Color) {
        let (x_i32, y_i32) = position;
        let screen_x = x_i32 * CELL_SIZE;
        let screen_y = (tetris::PLAY_FIELD_HEIGHT as i32 - 1 - y_i32) * CELL_SIZE;


        let rect = Rect::new(screen_x, screen_y, CELL_SIZE as u32, CELL_SIZE as u32);
        self.canvas.set_draw_color(color);
        let _ = self.canvas.fill_rect(rect);
        
        self.canvas.set_draw_color(SDL2Color::RGB(100, 100, 100));
        let _ = self.canvas.draw_rect(rect);
    }

    fn draw_current_tetrimino(&mut self, current_tetromino: &Tetromino) {
        let (x, y) = &current_tetromino.position;

        for (x_offset, y_offset) in current_tetromino.get_positions() {
            self.draw_cell((x + x_offset, y + y_offset), SDL2Color::RGB(200, 0, 0));
        }
    }

    fn draw_gridlines(&mut self) {
        self.canvas.set_draw_color(SDL2Color::RGB(50, 50, 50));

        for x in 0..=tetris::PLAY_FIELD_WIDTH {
            let x_pos = x as i32 * CELL_SIZE;
            let _ = self.canvas.draw_line((x_pos, 0), (x_pos, self.height));
        }

        for y in 0..=tetris::PLAY_FIELD_HEIGHT {
            let y_pos = y as i32 * CELL_SIZE;
            let _ = self.canvas.draw_line((0, y_pos), (self.width, y_pos));
        }
    }
}

impl RenderEngine for SDL2RenderEngine {
    fn render(&mut self, tetris: &Tetris) {
        
        self.canvas.set_draw_color(SDL2Color::RGB(20, 20, 20));
        self.canvas.clear();

        self.draw_gridlines();

        for x in 0..tetris::PLAY_FIELD_WIDTH {
            for y in 0..tetris::PLAY_FIELD_HEIGHT {
                match tetris.play_field[[x, y]] {
                    Cell::Block => {
                        self.draw_cell((x as i32, y as i32), SDL2Color::RGB(128, 128, 128));
                    }
                    _ => {}
                }
            }
        }

        self.draw_current_tetrimino(tetris.get_current_tetromino());
        
        self.canvas.present();
    }
}
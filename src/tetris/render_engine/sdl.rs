use crate::tetris::render_engine::RenderEngine;
use crate::tetris::{
    Cell, Tetris, tetromino::Position, tetromino::Tetromino, tetromino::TetrominoType,
};

use sdl2::Sdl;
use sdl2::pixels::Color as SDL2Color;
use sdl2::rect::Rect;
use sdl2::render::Canvas;
use sdl2::video::Window;

use crate::tetris;

pub const CELL_SIZE: i32 = 30;

const GRID_LINE_COLOR: SDL2Color = SDL2Color::RGB(128, 128, 128);
const BACKGROUND_COLOR: SDL2Color = SDL2Color::RGB(20, 20, 20);

pub struct SDL2RenderEngine {
    canvas: Canvas<Window>,
    width: i32,
    height: i32,
}

fn get_tetronimo_colour(tetromino_type: TetrominoType) -> SDL2Color {
    match tetromino_type {
        TetrominoType::Square => SDL2Color::RGB(247, 211, 8),
        TetrominoType::Line => SDL2Color::RGB(49, 199, 239),
        TetrominoType::T => SDL2Color::RGB(173, 77, 156),
        TetrominoType::L => SDL2Color::RGB(239, 121, 33),
        TetrominoType::J => SDL2Color::RGB(90, 101, 173),
        TetrominoType::S => SDL2Color::RGB(66, 182, 66),
        TetrominoType::Z => SDL2Color::RGB(239, 32, 41),
    }
}

impl SDL2RenderEngine {
    pub fn new(sdl_context: &Sdl) -> Self {
        let width = CELL_SIZE * tetris::PLAY_FIELD_WIDTH as i32;
        let height = CELL_SIZE * tetris::PLAY_FIELD_HEIGHT as i32;
        let video_subsystem = sdl_context.video().unwrap();

        let window = video_subsystem
            .window("Tetris", width as u32, height as u32)
            .position_centered()
            .build()
            .unwrap();

        let canvas = window.into_canvas().build().unwrap();

        Self {
            canvas,
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

        // self.canvas.set_draw_color(GRID_LINE_COLOR);
        // let _ = self.canvas.draw_rect(rect);
    }

    fn draw_current_tetrimino(&mut self, current_tetromino: &Tetromino) {
        let (x, y) = &current_tetromino.position;

        for (x_offset, y_offset) in current_tetromino.get_positions() {
            self.draw_cell(
                (x + x_offset, y + y_offset),
                get_tetronimo_colour(current_tetromino.tetromino_type),
            );
        }
    }

    fn draw_gridlines(&mut self) {
        self.canvas.set_draw_color(GRID_LINE_COLOR);

        for x in 0..=tetris::PLAY_FIELD_WIDTH {
            let x_pos = x as i32 * CELL_SIZE;
            let _ = self.canvas.draw_line((x_pos, 0), (x_pos, self.height));
            let _ = self.canvas.draw_line(
                (x_pos + CELL_SIZE - 1, 0),
                (x_pos + CELL_SIZE - 1, self.height),
            );
        }

        for y in 0..=tetris::PLAY_FIELD_HEIGHT {
            let y_pos = y as i32 * CELL_SIZE;
            let _ = self.canvas.draw_line(
                (0, y_pos + CELL_SIZE - 1),
                (self.width, y_pos + CELL_SIZE - 1),
            );
        }
    }
}

impl RenderEngine for SDL2RenderEngine {
    fn render(&mut self, tetris: &Tetris) {
        self.canvas.set_draw_color(BACKGROUND_COLOR);
        self.canvas.clear();

        for x in 0..tetris::PLAY_FIELD_WIDTH {
            for y in 0..tetris::PLAY_FIELD_HEIGHT {
                if let Cell::Block(tetromino_type) = tetris.play_field[[x, y]] {
                    self.draw_cell((x as i32, y as i32), get_tetronimo_colour(tetromino_type));
                }
            }
        }

        self.draw_current_tetrimino(tetris.get_current_tetromino());

        self.draw_gridlines();

        self.canvas.present();
    }
}

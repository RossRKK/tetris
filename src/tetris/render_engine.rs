use crate::tetris::Tetris;

pub trait RenderEngine {
    fn render(&mut self, tetris: &Tetris);
}

#[cfg(feature = "sdl")]
pub mod sdl;

#[cfg(feature = "flo_draw")]
pub mod flo_draw;
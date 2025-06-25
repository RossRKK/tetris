use crate::tetris::Tetris;

pub trait RenderEngine {
    fn render(&mut self, tetris: &Tetris);
}

pub mod sdl;

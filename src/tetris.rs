use std::time::Duration;

use crate::tetris::render_engine::RenderEngine;

pub mod render_engine;

pub struct Tetris<R: RenderEngine> {
    render_engine:  R,
}

impl <R: RenderEngine> Tetris<R> {
    pub fn new(render_engine: R) -> Self {
        Tetris { render_engine: render_engine }
    }

    pub fn game_tick(self: &mut Self, delta_time: Duration) {
        // println!("{:?}", delta_time);
    }

    pub fn render(self: &Self) {
        self.render_engine.draw_square();
    }
}
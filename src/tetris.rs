use std::time::Duration;

pub mod render_engine;

pub struct Tetris {
    render_engine: render_engine::RenderEngine,
}

impl Tetris {
    pub fn new(render_engine: render_engine::RenderEngine) -> Self {
        Tetris { render_engine: render_engine }
    }

    pub fn game_tick(self: &mut Tetris, delta_time: Duration) {
        println!("{:?}", delta_time);
    }

    pub fn render(self: &Tetris) {
        self.render_engine.draw_square();
    }
}
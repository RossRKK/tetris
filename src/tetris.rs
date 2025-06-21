use std::time::Duration;

pub struct Tetris {

}

impl Tetris {
    pub fn game_tick(self: &mut Tetris, delta_time: Duration) {
        println!("{:?}", delta_time);
    }

    pub fn render(self: &mut Tetris) {

    }
}
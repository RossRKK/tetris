mod tetris;

use flo_draw::*;
use flo_canvas::*;

use std::time::{SystemTime, Duration};

fn game_loop(mut tetris: tetris::Tetris) {
    let mut loop_start = SystemTime::now();

    //60fps
    let frame_time = Duration::from_secs_f64(0.0166666666667);

    //TODO stop looping if the window closes
    loop {
        let delta_time = loop_start.elapsed().unwrap();
        loop_start = SystemTime::now();

        tetris.game_tick(delta_time);

        tetris.render();

        std::thread::sleep(frame_time - loop_start.elapsed().unwrap());
    }
}

fn main() {
    with_2d_graphics(|| {
        let (canvas, events) = create_drawing_window_with_events("Tetris");

        let render_engine = tetris::render_engine::RenderEngine::new(canvas);

        let tetris = tetris::Tetris::new(render_engine);

        game_loop(tetris);
    });
}


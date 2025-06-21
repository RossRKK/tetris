mod tetris;

use flo_draw::*;
use flo_canvas::*;

use std::time::{SystemTime, Duration};

fn game_loop() {
    let mut tetris = tetris::Tetris {};

    let mut loop_start = SystemTime::now();

    //60fps
    let frame_time = Duration::from_secs_f64(0.0166666666667);

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

        canvas.draw(|gc| {
            gc.clear_canvas(Color::Rgba(0.0, 0.0, 0.0, 1.0));
            gc.canvas_height(1000.0);
            gc.center_region(0.0, 0.0, 1000.0, 1000.0);

            // Draw a rectangle...
            gc.new_path();
            gc.move_to(0.0, 0.0);
            gc.line_to(1000.0, 0.0);
            gc.line_to(1000.0, 1000.0);
            gc.line_to(0.0, 1000.0);
            gc.line_to(0.0, 0.0);

            gc.fill_color(Color::Rgba(1.0, 1.0, 0.8, 1.0));
            gc.fill();

            // Draw a triangle on top
            gc.new_path();
            gc.move_to(200.0, 200.0);
            gc.line_to(800.0, 200.0);
            gc.line_to(500.0, 800.0);
            gc.line_to(200.0, 200.0);

            gc.fill_color(Color::Rgba(0.0, 0.0, 0.8, 1.0));
            gc.fill();
        });

        game_loop();
    });
}


mod tetris;

use flo_draw::*;
use flo_canvas::*;

use std::time::{SystemTime, Duration};
use std::sync::mpsc;
use std::thread;

fn main() {
    with_2d_graphics(|| {
        let (canvas, events) = create_drawing_window_with_events("Tetris");

        let render_engine = tetris::render_engine::RenderEngine::new(canvas);
        let tetris = tetris::Tetris::new(render_engine);

        // Create a channel for sending input events to the game loop
        let (input_sender, input_receiver) = mpsc::channel();

        // Spawn event handler thread
        thread::spawn(move || {
            futures::executor::block_on(async {
                use futures::stream::StreamExt;
                let mut events = events;
                
                while let Some(event) = events.next().await {
                    match event {
                        DrawEvent::Closed => {
                            let _ = input_sender.send(InputEvent::WindowClosed);
                            break;
                        }
                        _ => {
                            // Handle other events as needed
                        }
                    }
                }
            });
        });

        // Run the game loop in the main thread
        game_loop(tetris, input_receiver);
    });
}

#[derive(Debug)]
enum InputEvent {
    WindowClosed,
}

fn game_loop(mut tetris: tetris::Tetris, input_receiver: mpsc::Receiver<InputEvent>) {
    let mut loop_start = SystemTime::now();
    let frame_time = Duration::from_secs_f64(1.0 / 60.0); // 60fps

    loop {
        let delta_time = loop_start.elapsed().unwrap();
        loop_start = SystemTime::now();

        // Process all pending input events
        while let Ok(input_event) = input_receiver.try_recv() {
            match input_event {
                InputEvent::WindowClosed => {
                    println!("Window closed, exiting game loop");
                    return;
                }
            }
        }

        // Update game state
        tetris.game_tick(delta_time);

        // Render the game
        tetris.render();

        // Frame rate limiting
        let elapsed = loop_start.elapsed().unwrap();
        if elapsed < frame_time {
            std::thread::sleep(frame_time - elapsed);
        }
    }
}

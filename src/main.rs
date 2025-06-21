mod tetris;

use flo_draw::*;
use flo_canvas::*;

use std::time::{Instant, Duration};
use std::sync::mpsc;
use std::thread;

type TetrisGame = tetris::Tetris<tetris::render_engine::FloDrawRenderEngine>;

fn main() {
    with_2d_graphics(|| {
        let (canvas, events) = create_drawing_window_with_events("Tetris");

        let render_engine = tetris::render_engine::FloDrawRenderEngine::new(canvas);
        let tetris: TetrisGame = tetris::Tetris::new(render_engine);

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
                        },
                        DrawEvent::KeyUp(_, Some(key)) => {
                            let _ = input_sender.send(InputEvent::KeyUp(key));
                        },
                        DrawEvent::KeyDown(_, Some(key)) => {
                            let _ = input_sender.send(InputEvent::KeyDown(key));
                        },
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
    KeyDown(Key),
    KeyUp(Key),
    WindowClosed,
}

fn game_loop(mut tetris: TetrisGame, input_receiver: mpsc::Receiver<InputEvent>) {
    let mut loop_start = Instant::now();
    let frame_time = Duration::from_secs_f64(1.0 / 60.0); // 60fps

    loop {
        let delta_time = loop_start.elapsed();
        loop_start = Instant::now();

        // Process all pending input events
        while let Ok(input_event) = input_receiver.try_recv() {
            match input_event {
                InputEvent::WindowClosed => {
                    println!("Window closed, exiting game loop");
                    return;
                },
                InputEvent::KeyDown(key) => {
                    println!("{:?} key down", key);
                },
                InputEvent::KeyUp(key) => {
                    println!("{:?} key up", key);
                },
                _ => {
                    println!("Unrecognised event");
                }
            }
        }

        // Update game state
        tetris.game_tick(delta_time);

        // Render the game
        tetris.render();

        // Frame rate limiting
        let elapsed = loop_start.elapsed();
        if elapsed < frame_time {
            std::thread::sleep(frame_time - elapsed);
        }
    }
}

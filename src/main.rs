mod tetris;

use flo_draw::*;
use flo_canvas::*;

use std::time::{Instant, Duration};
use std::sync::mpsc;
use std::thread;

use crate::tetris::render_engine::RenderEngine;
use crate::tetris::{InputEvent, Tetris};

fn main() {
    with_2d_graphics(|| {
        let (canvas, events) = create_drawing_window_with_events("Tetris");

        let render_engine = tetris::render_engine::FloDrawRenderEngine::new(canvas);
        

        // Create a channel for sending input events to the game loop
        let (input_sender, input_receiver) = mpsc::channel();

        let tetris = Tetris::new();

        // Spawn event handler thread
        thread::spawn(move || {
            futures::executor::block_on(async {
                use futures::stream::StreamExt;
                let mut events = events;
                
                while let Some(event) = events.next().await {
                    match event {
                        DrawEvent::Closed => {
                            let _ = input_sender.send(tetris::InputEvent::Quit);
                            break;
                        },
                        // DrawEvent::KeyUp(_, Some(key)) => {
                        //     let _ = input_sender.send(InputEvent::KeyUp(key));
                        // },
                        // DrawEvent::KeyDown(_, Some(key)) => {
                        //     let _ = input_sender.send(InputEvent::KeyDown(key));
                        // },
                        _ => {
                            // Handle other events as needed
                        }
                    }
                }
            });
        });

        // Run the game loop in the main thread
        game_loop(tetris, render_engine, input_receiver);
    });
}

fn game_loop(mut tetris: Tetris, render_engine: impl RenderEngine, event_queue: mpsc::Receiver<InputEvent>) {
    let mut loop_start = Instant::now();
    let frame_time = Duration::from_secs_f64(1.0 / 60.0); // 60fps

    loop {
        let delta_time = loop_start.elapsed();
        loop_start = Instant::now();

        while let Ok(input_event) = event_queue.try_recv() {
            tetris.recieve_event(input_event);
        }

        // Update game state
        let output_event = tetris.game_tick(delta_time);

        match output_event {
            tetris::OutputEvent::Exit => {
                break;
            },
            _ => {}
        }

        // Render the gamex
        render_engine.render(&tetris);

        // Frame rate limiting
        let elapsed = loop_start.elapsed();
        if elapsed < frame_time {
            std::thread::sleep(frame_time - elapsed);
        }
        break;
    }
}

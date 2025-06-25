mod tetris;


use std::time::{Instant, Duration};
use std::sync::mpsc;
use std::thread;

use crate::tetris::render_engine::RenderEngine;
use crate::tetris::{GameAction, InputEvent, Tetris};

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use crate::tetris::render_engine::sdl::SDL2RenderEngine;


fn main() {
    run_with_sdl();
}


fn run_with_sdl() {
    let sdl_context = sdl2::init().unwrap();

    let render_engine = SDL2RenderEngine::new(&sdl_context);

    let tetris = Tetris::new();

    let event_pump = sdl_context.event_pump().unwrap();

    game_loop(tetris, render_engine, event_pump);
}

fn game_loop(mut tetris: Tetris, mut render_engine: impl RenderEngine, mut event_pump: sdl2::EventPump) {
    let mut loop_start = Instant::now();
    let frame_time = Duration::from_secs_f64(1.0 / 60.0);

    loop {
        let delta_time = loop_start.elapsed();
        loop_start = Instant::now();

        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. } => {
                    return;
                }
                Event::KeyDown { keycode: Some(keycode), .. } => {
                    match keycode {
                        Keycode::Up => {
                            tetris.recieve_event(InputEvent::TakeAction(GameAction::Rotate));
                        }
                        Keycode::Left => {
                            tetris.recieve_event(InputEvent::TakeAction(GameAction::MoveLeft));
                        }
                        Keycode::Right => {
                            tetris.recieve_event(InputEvent::TakeAction(GameAction::MoveRight));
                        }
                        Keycode::Down => {
                            tetris.recieve_event(InputEvent::TakeAction(GameAction::MoveDown));
                        }
                        _ => {}
                    }
                }
                _ => {}
            }
        }

        let output_event = tetris.game_tick(delta_time);

        match output_event {
            tetris::OutputEvent::Exit => {
                break;
            },
            _ => {}
        }

        render_engine.render(&tetris);

        let elapsed = loop_start.elapsed();
        if elapsed < frame_time {
            std::thread::sleep(frame_time - elapsed);
        }
    }
}


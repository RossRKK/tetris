mod tetris;

#[cfg(feature = "flo_draw")]
use flo_draw::*;

use std::time::{Instant, Duration};
use std::sync::mpsc;
use std::thread;

use crate::tetris::render_engine::RenderEngine;
use crate::tetris::{GameAction, InputEvent, Tetris};

#[cfg(feature = "sdl")]
use sdl2::event::Event;
#[cfg(feature = "sdl")]
use sdl2::keyboard::Keycode;
#[cfg(feature = "sdl")]
use crate::tetris::render_engine::sdl::SDL2RenderEngine;

use rustysynth::MidiFile;
use rustysynth::MidiFileSequencer;
use rustysynth::SoundFont;
use rustysynth::Synthesizer;
use rustysynth::SynthesizerSettings;
use std::fs::File;
use std::io::Write;
use std::sync::Arc;

fn simple_chord() {
    // Load the SoundFont.
    let mut sf2 = File::open("TimGM6mb.sf2").unwrap();
    let sound_font = Arc::new(SoundFont::new(&mut sf2).unwrap());

    // Create the synthesizer.
    let settings = SynthesizerSettings::new(44100);
    let mut synthesizer = Synthesizer::new(&sound_font, &settings).unwrap();

    // Play some notes (middle C, E, G).
    synthesizer.note_on(0, 60, 100);
    synthesizer.note_on(0, 64, 100);
    synthesizer.note_on(0, 67, 100);

    // The output buffer (3 seconds).
    let sample_count = (3 * settings.sample_rate) as usize;
    let mut left: Vec<f32> = vec![0_f32; sample_count];
    let mut right: Vec<f32> = vec![0_f32; sample_count];

    // Render the waveform.
    synthesizer.render(&mut left[..], &mut right[..]);
}

fn main() {
    simple_chord();

    #[cfg(feature = "flo_draw")]
    run_with_flo_draw();

    #[cfg(feature = "sdl")]
    run_with_sdl();
}

#[cfg(feature = "flo_draw")]
fn run_with_flo_draw() {
    with_2d_graphics(|| {
        let (canvas, events) = create_drawing_window_with_events("Tetris");

        let render_engine = tetris::render_engine::flo_draw::FloDrawRenderEngine::new(canvas);
        
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
                        DrawEvent::KeyDown(_, Some(Key::KeyUp)) => {
                            let _ = input_sender.send(InputEvent::TakeAction(GameAction::Rotate));
                        },
                        DrawEvent::KeyDown(_, Some(Key::KeyLeft)) => {
                            let _ = input_sender.send(InputEvent::TakeAction(GameAction::MoveLeft));
                        },
                        DrawEvent::KeyDown(_, Some(Key::KeyRight)) => {
                            let _ = input_sender.send(InputEvent::TakeAction(GameAction::MoveRight));
                        },
                        DrawEvent::KeyDown(_, Some(Key::KeyDown)) => {
                            let _ = input_sender.send(InputEvent::TakeAction(GameAction::MoveDown));
                        },
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

#[cfg(feature = "sdl")]
fn run_with_sdl() {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem
        .window("Tetris", 300, 600)
        .position_centered()
        .build()
        .unwrap();

    let canvas = window.into_canvas().build().unwrap();
    let render_engine = SDL2RenderEngine::new(canvas);

    let tetris = Tetris::new();

    let event_pump = sdl_context.event_pump().unwrap();

    game_loop_with_events(tetris, render_engine, event_pump);
}

#[cfg(feature = "sdl")]
fn game_loop_with_events(mut tetris: Tetris, mut render_engine: impl RenderEngine, mut event_pump: sdl2::EventPump) {
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

fn game_loop(mut tetris: Tetris, mut render_engine: impl RenderEngine, event_queue: mpsc::Receiver<InputEvent>) {
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
    }
}

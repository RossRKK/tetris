use std::time::Duration;

use std::sync::mpsc;

pub mod render_engine;

#[derive(Debug)]
pub enum InputEvent {
    Quit,
}

#[derive(Debug)]
pub enum OutputEvent {
    NoOp,
    Exit,
}

pub struct Tetris<R: render_engine::RenderEngine> {
    render_engine:  R,
    event_queue: mpsc::Receiver<InputEvent>,
}

impl <R: render_engine::RenderEngine> Tetris<R> {
    pub fn new(render_engine: R, event_queue: mpsc::Receiver<InputEvent>) -> Self {
        Tetris { render_engine: render_engine, event_queue: event_queue }
    }

    pub fn game_tick(self: &mut Self, delta_time: Duration) -> OutputEvent {
        // println!("{:?}", delta_time);
        while let Ok(input_event) = self.event_queue.try_recv() {
            match input_event {
                InputEvent::Quit => {
                    println!("Window closed, exiting game loop");
                    return OutputEvent::Exit;
                },
                // InputEvent::KeyDown(key) => {
                //     println!("{:?} key down", key);
                // },
                // InputEvent::KeyUp(key) => {
                //     println!("{:?} key up", key);
                // },
                _ => {
                    println!("Unrecognised event");
                }
            }
        }

        OutputEvent::NoOp
    }

    pub fn render(self: &Self) {
        self.render_engine.draw_square();
    }
}
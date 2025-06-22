use std::time::Duration;

use std::sync::mpsc;

use ndarray::Array2;


mod tetronimo;
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


#[derive(Default)]
#[derive(Clone)]
pub enum Cell {
    #[default]
    Empty,
    Block,
}


pub struct Tetris<R: render_engine::RenderEngine> {
    render_engine:  R,
    event_queue: mpsc::Receiver<InputEvent>,
    play_field: ndarray::Array2<Cell>,
    current_tetromino: tetronimo::Tetromino,
}

impl <R: render_engine::RenderEngine> Tetris<R> {
    pub fn new(render_engine: R, event_queue: mpsc::Receiver<InputEvent>) -> Self {
        let play_field: Array2<Cell> = ndarray::Array2::<Cell>::from_elem((10, 20), Cell::Empty);

        Tetris { 
            render_engine, 
            event_queue, 
            play_field,
            current_tetromino: tetronimo::Tetromino::random(),
        }
    }

    pub fn game_tick(self: &mut Self, delta_time: Duration) -> OutputEvent {
        // println!("{:?}", delta_time);

        println!("{:?}", self.current_tetromino.get_positions());
        self.current_tetromino.rotate(tetronimo::RotationDirection::Clockwise);

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
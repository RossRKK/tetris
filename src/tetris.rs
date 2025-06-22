use std::time::Duration;

use ndarray::Array2;

use crate::tetris::tetronimo::Tetromino;

mod tetronimo;
pub mod render_engine;

pub const PLAY_FIELD_WIDTH: usize = 10;
pub const PLAY_FIELD_HEIGHT: usize = 20;

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


pub struct Tetris {
    play_field: ndarray::Array2<Cell>,
    current_tetromino: Tetromino,
    should_exit: bool,
}

impl Tetris {
    pub fn new() -> Self {
        let play_field: Array2<Cell> = ndarray::Array2::<Cell>::from_elem((PLAY_FIELD_WIDTH, PLAY_FIELD_HEIGHT), Cell::Empty);

        Tetris { 
            play_field,
            current_tetromino: Tetromino::random(),
            should_exit: false,
        }
    }

    pub fn recieve_event(self: &mut Self, input_event: InputEvent) {
        match input_event {
            InputEvent::Quit => {
                println!("Window closed, exiting game loop");
                self.should_exit = true;
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

    pub fn get_current_tetromino(self: &Self) -> &Tetromino {
        &self.current_tetromino
    }

    pub fn game_tick(self: &mut Self, delta_time: Duration) -> OutputEvent {
        if self.should_exit {
            return OutputEvent::Exit;
        }

        println!("{:?}", self.current_tetromino.get_positions());
        self.current_tetromino.rotate(tetronimo::RotationDirection::Clockwise);

        OutputEvent::NoOp
    }
}
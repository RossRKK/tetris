use std::time::Duration;

use ndarray::Array2;

use crate::tetris::tetronimo::Tetromino;

mod tetronimo;
pub mod render_engine;

pub const PLAY_FIELD_WIDTH: usize = 10;
pub const PLAY_FIELD_HEIGHT: usize = 20;

#[derive(Debug, Copy, Clone)]
pub enum GameAction {
    RotateClockwise,
    RotateAntiClockwise,
}

#[derive(Debug)]
pub enum InputEvent {
    Quit,
    TakeAction(GameAction),
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
    action_queue: Vec<GameAction>,
}

impl Tetris {
    pub fn new() -> Self {
        let play_field: Array2<Cell> = ndarray::Array2::<Cell>::from_elem((PLAY_FIELD_WIDTH, PLAY_FIELD_HEIGHT), Cell::Empty);

        Tetris { 
            play_field,
            current_tetromino: Tetromino::random(),
            should_exit: false,
            action_queue: Vec::<GameAction>::new(),
        }
    }

    pub fn recieve_event(self: &mut Self, input_event: InputEvent) {
        match input_event {
            InputEvent::Quit => {
                println!("Window closed, exiting game loop");
                self.should_exit = true;
            },
            InputEvent::TakeAction(action) => {
                self.action_queue.push(action);
            },
            _ => {
                println!("Unrecognised event");
            }
        }
    }

    pub fn get_current_tetromino(self: &Self) -> &Tetromino {
        &self.current_tetromino
    }

    fn take_action(self: &mut Self, action: GameAction) {
        match action {
            GameAction::RotateClockwise => {
                self.current_tetromino.rotate(tetronimo::RotationDirection::Clockwise);
            },
            GameAction::RotateAntiClockwise => {
                self.current_tetromino.rotate(tetronimo::RotationDirection::AntiClockwise);
            }
        }
    }

    pub fn game_tick(self: &mut Self, delta_time: Duration) -> OutputEvent {
        if self.should_exit {
            return OutputEvent::Exit;
        }

        let actions: Vec<GameAction> = self.action_queue.drain(..).collect();
        for action in actions {
            self.take_action(action);
        }

        OutputEvent::NoOp
    }
}
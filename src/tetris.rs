use std::time::Duration;

use ndarray::Array2;

use crate::tetris::tetronimo::Tetromino;

mod tetronimo;
pub mod render_engine;

pub const PLAY_FIELD_WIDTH: usize = 10;
pub const PLAY_FIELD_HEIGHT: usize = 20;

#[derive(Debug, Copy, Clone)]
pub enum GameAction {
    Rotate,
    MoveDown,
    MoveRight,
    MoveLeft,
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
#[derive(PartialEq)]
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
        let tetronimo_backup = self.current_tetromino.clone();
        match action {
            GameAction::Rotate => {
                self.current_tetromino.rotate(tetronimo::RotationDirection::Clockwise);
            },
            GameAction::MoveDown => {
                let (x, y) = self.current_tetromino.position;
                self.current_tetromino.position = (x, y - 1);
            },
            GameAction::MoveRight => {
                let (x, y) = self.current_tetromino.position;
                self.current_tetromino.position = (x + 1, y);
            },
            GameAction::MoveLeft => {
                let (x, y) = self.current_tetromino.position;
                self.current_tetromino.position = (x - 1, y);
            },
        }

        let mut legal_move = true;
        for (x_offset, y_offset) in self.current_tetromino.get_positions() {
            let (x_origin, y_origin) = self.current_tetromino.position;
            let (x, y) = (x_origin + x_offset, y_origin + y_offset);

            if x < 0 || x >= PLAY_FIELD_WIDTH as i32 {
                legal_move = false;
                break;
            }

            if y < 0 {
                legal_move = false;
                break;
            }

            if y < PLAY_FIELD_HEIGHT as i32 && self.play_field[[x as usize, y as usize]] == Cell::Block {
                legal_move = false;
                break;
            }
        }

        //if the move isn't legal undo it
        if !legal_move {
            self.current_tetromino = tetronimo_backup;
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
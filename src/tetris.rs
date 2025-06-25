use std::{collections::HashSet, time::{Duration, Instant}};

use ndarray::{Array2, Axis};

use crate::tetris::tetromino::{Tetromino, TetrominoType};

pub mod tetromino;
pub mod render_engine;

pub const PLAY_FIELD_WIDTH: usize = 10;
pub const PLAY_FIELD_HEIGHT: usize = 20;

const fn from_frames(frames: u64) -> Duration {
    Duration::from_nanos(016666666 * frames)
}

const LEVEL_DROP_DURATIONS: [Duration; 21] = [from_frames(53), from_frames(49), from_frames(45), from_frames(41), from_frames(37), from_frames(33), from_frames(28), from_frames(22), from_frames(17), from_frames(11), from_frames(10), from_frames(9), from_frames(8), from_frames(7), from_frames(6), from_frames(6), from_frames(5), from_frames(5), from_frames(4), from_frames(4), from_frames(3)];

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
pub enum Cell {
    #[default]
    Empty,
    Block(tetromino::TetrominoType),
}

impl Cell {
    fn is_block(self: &Self) -> bool {
        match self {
            Cell::Empty => { false }
            Cell::Block(_) => { true }
        }
    }
}


pub struct Tetris {
    level: u32,
    score: u32,
    play_field: ndarray::Array2<Cell>,
    current_tetromino: Tetromino,
    should_exit: bool,
    action_queue: Vec<GameAction>,
    move_down_delay: Duration,
    time_of_last_move: Instant,
}

impl Tetris {
    pub fn new() -> Self {
        let play_field: Array2<Cell> = ndarray::Array2::<Cell>::from_elem((PLAY_FIELD_WIDTH, PLAY_FIELD_HEIGHT), Cell::Empty);

        Tetris { 
            level: 0,
            score: 0,
            play_field,
            current_tetromino: Tetromino::random(),
            should_exit: false,
            action_queue: Vec::<GameAction>::new(),
            move_down_delay: Duration::from_secs(1),
            time_of_last_move: Instant::now(),
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

    fn take_action(self: &mut Self, action: GameAction) -> bool {
        let tetronimo_backup = self.current_tetromino.clone();
        match action {
            GameAction::Rotate => {
                self.current_tetromino.rotate(tetromino::RotationDirection::Clockwise);
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

            if y < PLAY_FIELD_HEIGHT as i32 && self.play_field[[x as usize, y as usize]].is_block() {
                legal_move = false;
                break;
            }
        }

        //if the move isn't legal undo it
        if !legal_move {
            self.current_tetromino = tetronimo_backup;

            //if the game or the player was trying to move the piece down and couldn't
            //commit it to the board
            match action {
                GameAction::MoveDown => {
                    self.commit_current_tetromino();
                },
                _ => {}
            }

            return false;
        }
        true
    }

    fn commit_current_tetromino(self: &mut Self) {
         for (x_offset, y_offset) in self.current_tetromino.get_positions() {
            let (x_origin, y_origin) = self.current_tetromino.position;
            let (x, y) = (x_origin + x_offset, y_origin + y_offset);

            if y < PLAY_FIELD_HEIGHT as i32 {
                self.play_field[[x as usize, y as usize]] = Cell::Block(self.current_tetromino.tetromino_type);
            } else {
                todo!("Game Over");
            }
        }

        self.current_tetromino = Tetromino::random();

        self.clear_lines();
    }

    fn clear_lines(self: &mut Self) {
        let mut row_index: usize = 0;

        let mut cleared_lines: HashSet<usize> = HashSet::<usize>::new();

        for row in self.play_field.lanes(Axis(0)) {
            let line_complete = row.iter().all(|cell| cell.is_block());
            
            if line_complete {
                cleared_lines.insert(row_index);
            }
            row_index += 1;
        }


        if cleared_lines.len() == 4 {
            println!("Tetris!");
        } else {
            println!("Cleared {} lines", cleared_lines.len());
        }

        // based on nintendo gameboy scoring system
        //TODO: soft drop scoring
        match cleared_lines.len() {
            1 => { self.score += 40 * (self.level + 1)},
            2 => { self.score += 100 * (self.level + 1) },
            3 => { self.score += 300 * (self.level + 1) },
            4 => { self.score += 1200 * (self.level + 1) },
            _ => {}
        }

        let new_field = self.play_field.select(Axis(1), &(0..PLAY_FIELD_HEIGHT).filter(|x| !cleared_lines.contains(x)).collect::<Vec<_>>());
        let empty_rows = Array2::from_elem((PLAY_FIELD_WIDTH, cleared_lines.len()), Cell::Empty);

        self.play_field = ndarray::concatenate![Axis(1), new_field, empty_rows];
    }

    pub fn game_tick(self: &mut Self, _: Duration) -> OutputEvent {
        if self.should_exit {
            return OutputEvent::Exit;
        }

        let actions: Vec<GameAction> = self.action_queue.drain(..).collect();
        for action in actions {
            let _ = self.take_action(action);
        }

        if self.time_of_last_move.elapsed() > LEVEL_DROP_DURATIONS[self.level as usize] {
            //auto-tick down
            let _ = self.take_action(GameAction::MoveDown);

            self.time_of_last_move = Instant::now();
        }

        OutputEvent::NoOp
    }
}
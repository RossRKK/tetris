use std::{
    collections::HashSet,
    time::{Duration, Instant},
};

use ndarray::{Array2, Axis};

use crate::tetris::tetromino::Tetromino;

pub mod render_engine;
pub mod tetromino;

pub const PLAY_FIELD_WIDTH: usize = 10;
pub const PLAY_FIELD_HEIGHT: usize = 20;

const fn from_frames(frames: u64) -> Duration {
    Duration::from_nanos(16666666 * frames)
}

const MAX_LEVEL: usize = 21;

const LEVEL_DROP_DURATIONS: [Duration; MAX_LEVEL] = [
    from_frames(53),
    from_frames(49),
    from_frames(45),
    from_frames(41),
    from_frames(37),
    from_frames(33),
    from_frames(28),
    from_frames(22),
    from_frames(17),
    from_frames(11),
    from_frames(10),
    from_frames(9),
    from_frames(8),
    from_frames(7),
    from_frames(6),
    from_frames(6),
    from_frames(5),
    from_frames(5),
    from_frames(4),
    from_frames(4),
    from_frames(3),
];

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

#[derive(Default, Clone)]
pub enum Cell {
    #[default]
    Empty,
    Block(tetromino::TetrominoType),
}

impl Cell {
    fn is_block(&self) -> bool {
        match self {
            Cell::Empty => false,
            Cell::Block(_) => true,
        }
    }
}

pub struct Tetris {
    level: u32,
    score: u32,
    lines_to_clear_before_next_level: u32,
    play_field: ndarray::Array2<Cell>,
    current_tetromino: Tetromino,
    should_exit: bool,
    action_queue: Vec<GameAction>,
    time_of_last_move: Instant,
}

impl Tetris {
    pub fn new(level: u32) -> Self {
        let play_field: Array2<Cell> =
            ndarray::Array2::<Cell>::from_elem((PLAY_FIELD_WIDTH, PLAY_FIELD_HEIGHT), Cell::Empty);

        Tetris {
            level,
            score: 0,
            lines_to_clear_before_next_level: (level * 10) + 10, //based on gameboy marathon mode
            play_field,
            current_tetromino: Tetromino::random(),
            should_exit: false,
            action_queue: Vec::<GameAction>::new(),
            time_of_last_move: Instant::now(),
        }
    }

    pub fn recieve_event(&mut self, input_event: InputEvent) {
        match input_event {
            InputEvent::Quit => {
                println!("Window closed, exiting game loop");
                self.should_exit = true;
            }
            InputEvent::TakeAction(action) => {
                self.action_queue.push(action);
            }
        }
    }

    pub fn get_current_tetromino(&self) -> &Tetromino {
        &self.current_tetromino
    }

    fn take_action(&mut self, action: GameAction) -> OutputEvent {
        let tetronimo_backup = self.current_tetromino.clone();
        match action {
            GameAction::Rotate => {
                self.current_tetromino.rotate();
            }
            GameAction::MoveDown => {
                let (x, y) = self.current_tetromino.position;
                self.current_tetromino.position = (x, y - 1);
            }
            GameAction::MoveRight => {
                let (x, y) = self.current_tetromino.position;
                self.current_tetromino.position = (x + 1, y);
            }
            GameAction::MoveLeft => {
                let (x, y) = self.current_tetromino.position;
                self.current_tetromino.position = (x - 1, y);
            }
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

            if y < PLAY_FIELD_HEIGHT as i32 && self.play_field[[x as usize, y as usize]].is_block()
            {
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
                    return self.commit_current_tetromino();
                }
                _ => {
                    return OutputEvent::NoOp;
                }
            }
        }
        OutputEvent::NoOp
    }

    fn commit_current_tetromino(&mut self) -> OutputEvent {
        for (x_offset, y_offset) in self.current_tetromino.get_positions() {
            let (x_origin, y_origin) = self.current_tetromino.position;
            let (x, y) = (x_origin + x_offset, y_origin + y_offset);

            if y < PLAY_FIELD_HEIGHT as i32 {
                self.play_field[[x as usize, y as usize]] =
                    Cell::Block(self.current_tetromino.tetromino_type);
            } else {
                //game over
                println!("Level: {}", self.level);
                println!("Score: {}", self.score);
                return OutputEvent::Exit;
            }
        }

        self.current_tetromino = Tetromino::random();

        self.clear_lines();
        OutputEvent::NoOp
    }

    fn clear_lines(&mut self) {
        let mut cleared_lines: HashSet<usize> = HashSet::<usize>::new();

        for (row_index, row) in self.play_field.lanes(Axis(0)).into_iter().enumerate() {
            let line_complete = row.iter().all(|cell| cell.is_block());

            if line_complete {
                cleared_lines.insert(row_index);
            }
        }

        if cleared_lines.len() == 4 {
            println!("Tetris!");
        }

        // based on nintendo gameboy scoring system
        //TODO: soft drop scoring
        match cleared_lines.len() {
            1 => self.score += 40 * (self.level + 1),
            2 => self.score += 100 * (self.level + 1),
            3 => self.score += 300 * (self.level + 1),
            4 => self.score += 1200 * (self.level + 1),
            _ => {}
        }

        if cleared_lines.len() as u32 > self.lines_to_clear_before_next_level {
            self.lines_to_clear_before_next_level = 10;
            if self.level < MAX_LEVEL as u32 {
                self.level += 1;
            }
            println!("Level: {}", self.level);
            println!("Score: {}", self.score);
        } else {
            self.lines_to_clear_before_next_level -= cleared_lines.len() as u32;
        }

        let new_field = self.play_field.select(
            Axis(1),
            &(0..PLAY_FIELD_HEIGHT)
                .filter(|x| !cleared_lines.contains(x))
                .collect::<Vec<_>>(),
        );
        let empty_rows = Array2::from_elem((PLAY_FIELD_WIDTH, cleared_lines.len()), Cell::Empty);

        self.play_field = ndarray::concatenate![Axis(1), new_field, empty_rows];
    }

    pub fn game_tick(&mut self, _: Duration) -> OutputEvent {
        if self.should_exit {
            return OutputEvent::Exit;
        }

        let actions: Vec<GameAction> = self.action_queue.drain(..).collect();
        for action in actions {
            let output_event = self.take_action(action);

            if let OutputEvent::Exit = output_event {
                return output_event;
            }
        }

        if self.time_of_last_move.elapsed() > LEVEL_DROP_DURATIONS[self.level as usize] {
            //auto-tick down
            let output_event = self.take_action(GameAction::MoveDown);

            if let OutputEvent::Exit = output_event {
                return output_event;
            }

            self.time_of_last_move = Instant::now();
        }

        OutputEvent::NoOp
    }
}

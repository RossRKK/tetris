use rand::Rng;

const TETROMINO_SIZE: usize = 4;

pub type PositionIndex = i32;

pub type Position = (PositionIndex, PositionIndex);


// array representing a Tetromino in a specific orientiation
pub type TetrominoRotation = [(PositionIndex, PositionIndex); TETROMINO_SIZE];

// Square piece (O) - only 1 rotation needed
const SQUARE_PIECE_POSITIONS: [TetrominoRotation; 1] = [
    [(0, 0), (0, 1), (1, 0), (1, 1)]
];

// Line piece (I) - 2 rotations needed
const LINE_PIECE_POSITIONS: [TetrominoRotation; 2] = [
    [(0, -1), (0, 0), (0, 1), (0, 2)], // Vertical
    [(-1, 0), (0, 0), (1, 0), (2, 0)]  // Horizontal
];

// T piece - 4 rotations needed
const T_PIECE_POSITIONS: [TetrominoRotation; 4] = [
    [(-1, 0), (0, 0), (1, 0), (0, 1)],  // T upright
    [(0, -1), (0, 0), (0, 1), (1, 0)],  // T right
    [(-1, 0), (0, 0), (1, 0), (0, -1)], // T upside down
    [(0, -1), (0, 0), (0, 1), (-1, 0)]  // T left
];

// L piece - 4 rotations needed
const L_PIECE_POSITIONS: [TetrominoRotation; 4] = [
    [(0, -1), (0, 0), (0, 1), (1, 1)],   // L upright
    [(-1, 0), (0, 0), (1, 0), (-1, 1)],  // L right
    [(0, -1), (0, 0), (0, 1), (-1, -1)], // L upside down
    [(-1, 0), (0, 0), (1, 0), (1, -1)]   // L left
];

// J piece (reverse L) - 4 rotations needed
const J_PIECE_POSITIONS: [TetrominoRotation; 4] = [
    [(0, -1), (0, 0), (0, 1), (-1, 1)],  // J upright
    [(-1, 0), (0, 0), (1, 0), (-1, -1)], // J right
    [(0, -1), (0, 0), (0, 1), (1, -1)],  // J upside down
    [(-1, 0), (0, 0), (1, 0), (1, 1)]    // J left
];

// S piece - 2 rotations needed
const S_PIECE_POSITIONS: [TetrominoRotation; 2] = [
    [(1, 0), (0, 0), (0, 1), (-1, 1)],   // S horizontal
    [(-1, -1), (-1, 0), (0, 0), (0, 1)]    // S vertical
];

// Z piece - 2 rotations needed
const Z_PIECE_POSITIONS: [TetrominoRotation; 2] = [
    [(-1, 0), (0, 0), (0, 1), (1, 1)],   // Z horizontal
    [(0, -1), (0, 0), (-1, 0), (-1, 1)]    // Z vertical
];

#[derive(Clone, Copy)]
#[derive(Debug)]
pub enum TetrominoType {
    Square,
    Line,
    T,
    L,
    J,
    S,
    Z,
}

impl TetrominoType {
    fn get_num_rotations(&self) -> usize {
        match self {
            TetrominoType::Square => SQUARE_PIECE_POSITIONS.len(),
            TetrominoType::Line => LINE_PIECE_POSITIONS.len(),
            TetrominoType::T => T_PIECE_POSITIONS.len(),
            TetrominoType::L => L_PIECE_POSITIONS.len(),
            TetrominoType::J => J_PIECE_POSITIONS.len(),
            TetrominoType::S => S_PIECE_POSITIONS.len(),
            TetrominoType::Z => Z_PIECE_POSITIONS.len(),
        }
    }

    const ALL_TYPES: [TetrominoType; 7] = [
        TetrominoType::Square,
        TetrominoType::Line,
        TetrominoType::T,
        TetrominoType::L, 
        TetrominoType::J,
        TetrominoType::S,
        TetrominoType::Z,
    ];

    pub fn random() -> Self {
        let mut rng = rand::rng();
        Self::ALL_TYPES[rng.random_range(0..Self::ALL_TYPES.len())]
    }
}


#[derive(Debug)]
#[derive(Clone)]
pub struct Tetromino {
    pub tetromino_type: TetrominoType,
    rotation: usize,
    pub position: Position,
}

impl Tetromino {
    pub fn new(tetromino_type: TetrominoType) -> Self {
        Self { tetromino_type, rotation: 0, position: (5, 20) }
    }

    pub fn random() -> Self {
        Self::new(TetrominoType::random())
    }

    pub fn rotate(&mut self) {
        self.rotation = (self.rotation + 1) % self.tetromino_type.get_num_rotations();
    }

    pub fn get_positions(&self) -> &TetrominoRotation {
        match self.tetromino_type {
            TetrominoType::Square => &SQUARE_PIECE_POSITIONS[self.rotation],
            TetrominoType::Line => &LINE_PIECE_POSITIONS[self.rotation],
            TetrominoType::T => &T_PIECE_POSITIONS[self.rotation],
            TetrominoType::L => &L_PIECE_POSITIONS[self.rotation],
            TetrominoType::J => &J_PIECE_POSITIONS[self.rotation],
            TetrominoType::S => &S_PIECE_POSITIONS[self.rotation],
            TetrominoType::Z => &Z_PIECE_POSITIONS[self.rotation],
        }
    }
}

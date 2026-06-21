pub const BOARD_WIDTH: usize = 12;
pub const BOARD_HEIGHT: usize = 20;
pub const BOARD_WIDTH_I32: i32 = 12;
pub const BOARD_HEIGHT_I32: i32 = 20;
pub const HIGHLIGHT_COLOUR: &str = "white";
pub const EMPTY_CELL: &str = "";

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ShapeCell {
    pub val: u8,
    pub idx: usize,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Block {
    pub color: String,
    pub shape: Vec<ShapeCell>,
    pub x: i32,
    pub y: i32,
}

impl Block {
    pub fn is_empty(&self) -> bool {
        self.shape.is_empty()
    }

    /// Side length of the shape matrix (tetromino matrices are always square).
    pub fn matrix_side(&self) -> usize {
        if self.shape.is_empty() {
            return 0;
        }
        self.shape.len().isqrt()
    }
}

pub fn empty_block() -> Block {
    Block {
        color: String::new(),
        shape: Vec::new(),
        x: 0,
        y: 0,
    }
}

pub type Board = Vec<String>;

pub fn empty_board() -> Board {
    vec![EMPTY_CELL.to_string(); BOARD_WIDTH * BOARD_HEIGHT]
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AppState {
    pub board: Board,
    pub counter: u32,
    pub gamespeed: u32,
    pub score: u32,
    pub block: Block,
    pub paused: bool,
    pub gameover: bool,
    pub(crate) rng: u32,
}

impl AppState {
    pub fn initial() -> Self {
        Self::with_seed(1)
    }

    pub fn with_seed(seed: u32) -> Self {
        Self {
            board: empty_board(),
            counter: 0,
            gamespeed: 12,
            score: 0,
            block: empty_block(),
            paused: false,
            gameover: false,
            rng: seed.max(1),
        }
    }

    /// Inclusive upper bound `max`.
    pub fn next_random(&mut self, max: u32) -> u32 {
        self.rng = self.rng
            .wrapping_mul(1_664_525)
            .wrapping_add(1_013_904_223);
        if max == 0 {
            return 0;
        }
        self.rng % (max + 1)
    }

    pub fn reset_game(&self) -> Self {
        Self::with_seed(self.rng)
    }
}

/// Board index for a non-negative cell position.
pub fn cell_index(row: i32, col: i32) -> Option<usize> {
    if row < 0 || col < 0 {
        return None;
    }
    let row = usize::try_from(row).ok()?;
    let col = usize::try_from(col).ok()?;
    if row >= BOARD_HEIGHT || col >= BOARD_WIDTH {
        return None;
    }
    Some(row * BOARD_WIDTH + col)
}

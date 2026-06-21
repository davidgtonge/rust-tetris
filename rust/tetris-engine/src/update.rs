use crate::board::{highlighted_row, is_row_full, map_rows, merge_board_and_block};
use crate::effect::EffectCommand;
use crate::event::AppEvent;
use crate::shapes::{block_from_index, shape_count};
use crate::state::{
    cell_index, empty_block, AppState, Block, ShapeCell, BOARD_HEIGHT, BOARD_HEIGHT_I32,
    BOARD_WIDTH, BOARD_WIDTH_I32, HIGHLIGHT_COLOUR,
};

pub struct Transition {
    pub effects: Vec<EffectCommand>,
}

fn idle() -> Transition {
    Transition { effects: vec![] }
}

pub fn reduce(state: &mut AppState, event: &AppEvent) -> Transition {
    let current = state.clone();
    *state = match event {
        AppEvent::MoveLeft => shift_x(current, -1),
        AppEvent::MoveRight => shift_x(current, 1),
        AppEvent::MoveDown => shift_y(current, 1),
        AppEvent::Rotate => guard_block(current, rotate_block),
        AppEvent::Pause => AppState { paused: true, ..current },
        AppEvent::Resume => AppState { paused: false, ..current },
        AppEvent::Restart => current.reset_game(),
        AppEvent::Tick => tick_pipeline(current),
    };
    idle()
}

// --- Tick pipeline ------------------------------------------------------------

fn tick_pipeline(state: AppState) -> AppState {
    if state.paused || state.gameover {
        return state;
    }

    state
        .pipe(increment_counter)
        .pipe(slow(skip_if_empty_block(move_blocks)))
        .pipe(slow(clear_rows))
        .pipe(slow(mark_rows))
        .pipe(slow(run_if_empty_block(add_block)))
        .pipe(slow(when(|s| !s.gameover, increment_score)))
}

trait Pipe: Sized {
    fn pipe(self, step: impl FnOnce(Self) -> Self) -> Self;
}

impl Pipe for AppState {
    fn pipe(self, step: impl FnOnce(Self) -> Self) -> Self {
        step(self)
    }
}

// --- Combinators --------------------------------------------------------------

fn slow(step: impl Fn(AppState) -> AppState) -> impl Fn(AppState) -> AppState {
    move |state| {
        if on_slow_tick(&state) {
            step(state)
        } else {
            state
        }
    }
}

fn skip_if_empty_block(
    step: impl Fn(AppState) -> AppState,
) -> impl Fn(AppState) -> AppState {
    move |state| {
        if state.block.is_empty() {
            state
        } else {
            step(state)
        }
    }
}

fn run_if_empty_block(step: impl Fn(AppState) -> AppState) -> impl Fn(AppState) -> AppState {
    move |state| {
        if state.block.is_empty() {
            step(state)
        } else {
            state
        }
    }
}

fn when(
    pred: fn(&AppState) -> bool,
    step: impl Fn(AppState) -> AppState,
) -> impl Fn(AppState) -> AppState {
    move |state| {
        if pred(&state) {
            step(state)
        } else {
            state
        }
    }
}

fn guard_block(state: AppState, mutate: impl FnOnce(Block) -> Block) -> AppState {
    if state.paused || state.gameover || state.block.is_empty() {
        return state;
    }
    let trial = mutate(state.block.clone());
    if is_valid(&state.board, &trial) {
        AppState { block: trial, ..state }
    } else {
        state
    }
}

fn shift_x(state: AppState, dx: i32) -> AppState {
    guard_block(state, |b| Block { x: b.x + dx, ..b })
}

fn shift_y(state: AppState, dy: i32) -> AppState {
    guard_block(state, |b| Block { y: b.y + dy, ..b })
}

// --- Steps --------------------------------------------------------------------

fn increment_counter(state: AppState) -> AppState {
    AppState {
        counter: state.counter + 1,
        ..state
    }
}

fn on_slow_tick(state: &AppState) -> bool {
    state.counter.is_multiple_of(state.gamespeed)
}

pub fn add_block(mut state: AppState) -> AppState {
    let max = shape_count().saturating_sub(1);
    let index = state.next_random(max) as usize;
    state.block = block_from_index(index, None, 0);
    state
}

fn move_blocks(state: AppState) -> AppState {
    let trial = Block {
        y: state.block.y + 1,
        ..state.block.clone()
    };
    if is_valid(&state.board, &trial) {
        return AppState { block: trial, ..state };
    }

    if state.block.y == 0 {
        return AppState {
            paused: true,
            gameover: true,
            ..state
        };
    }

    AppState {
        board: merge_board_and_block(&state),
        block: empty_block(),
        ..state
    }
}

fn clear_rows(state: AppState) -> AppState {
    let rows: Vec<_> = state
        .board
        .chunks(BOARD_WIDTH)
        .map(<[String]>::to_vec)
        .collect();
    let cleared: Vec<_> = rows
        .into_iter()
        .filter(|row| !row.iter().any(|c| c == HIGHLIGHT_COLOUR))
        .collect();
    let removed = BOARD_HEIGHT - cleared.len();
    if removed == 0 {
        return state;
    }
    let mut flat: Vec<String> = cleared.into_iter().flatten().collect();
    flat.splice(
        0..0,
        (0..removed * BOARD_WIDTH).map(|_| String::new()),
    );
    let removed_u32 = u32::try_from(removed).expect("removed rows fit u32");
    AppState {
        board: flat,
        score: state.score + 25u32.pow(removed_u32),
        ..state
    }
}

fn mark_rows(state: AppState) -> AppState {
    AppState {
        board: map_rows(&state.board, |row| {
            if is_row_full(row) {
                highlighted_row()
            } else {
                row.to_vec()
            }
        }),
        ..state
    }
}

fn increment_score(state: AppState) -> AppState {
    AppState {
        score: state.score + 1,
        ..state
    }
}

// --- Block geometry -----------------------------------------------------------

fn is_valid(board: &[String], block: &Block) -> bool {
    if block.is_empty() {
        return true;
    }
    let size = block.matrix_side();
    for cell in &block.shape {
        if cell.val == 0 {
            continue;
        }
        let row = block.y + i32::try_from(cell.idx / size).expect("row offset fits i32");
        let col = block.x + i32::try_from(cell.idx % size).expect("col offset fits i32");
        if !(0..BOARD_WIDTH_I32).contains(&col) {
            return false;
        }
        if row >= BOARD_HEIGHT_I32 {
            return false;
        }
        if row < 0 {
            continue;
        }
        let Some(idx) = cell_index(row, col) else {
            return false;
        };
        if !board[idx].is_empty() && board[idx] != HIGHLIGHT_COLOUR {
            return false;
        }
    }
    true
}

fn rotate_block(block: Block) -> Block {
    let size = block.matrix_side();
    Block {
        shape: block
            .shape
            .iter()
            .map(|cell| {
                let row = cell.idx / size;
                let col = cell.idx % size;
                let new_idx = size - 1 - row + col * size;
                ShapeCell {
                    val: cell.val,
                    idx: new_idx,
                }
            })
            .collect(),
        ..block
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn restart_resets_score() {
        let mut state = AppState::initial();
        state.score = 100;
        reduce(&mut state, &AppEvent::Restart);
        assert_eq!(state.score, 0);
    }

    #[test]
    fn pause_stops_counter_on_tick() {
        let mut state = AppState::initial();
        state.paused = true;
        let before = state.counter;
        reduce(&mut state, &AppEvent::Tick);
        assert_eq!(state.counter, before);
    }

    #[test]
    fn add_block_spawns_piece() {
        let state = add_block(AppState::with_seed(42));
        assert!(!state.block.is_empty());
    }
}

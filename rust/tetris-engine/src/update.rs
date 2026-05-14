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

pub fn reduce(state: &mut AppState, event: &AppEvent) -> Transition {
    match event {
        AppEvent::MoveLeft => {
            guard_move(state, |b| b.x -= 1);
            Transition { effects: vec![] }
        }
        AppEvent::MoveRight => {
            guard_move(state, |b| b.x += 1);
            Transition { effects: vec![] }
        }
        AppEvent::MoveDown => {
            guard_move(state, |b| b.y += 1);
            Transition { effects: vec![] }
        }
        AppEvent::Rotate => {
            guard_move(state, rotate_block);
            Transition { effects: vec![] }
        }
        AppEvent::Pause => {
            state.paused = true;
            Transition { effects: vec![] }
        }
        AppEvent::Resume => {
            state.paused = false;
            Transition { effects: vec![] }
        }
        AppEvent::Restart => {
            *state = AppState::initial();
            request_next_shape(state)
        }
        AppEvent::Tick => tick_pipeline(state),
        AppEvent::EffectCompleted { effect_id, result } => {
            handle_effect_result(state, effect_id, result)
        }
    }
}

fn tick_pipeline(state: &mut AppState) -> Transition {
    if state.paused || state.gameover {
        return Transition { effects: vec![] };
    }

    state.counter += 1;
    let mut effects = Vec::new();

    if state.counter.is_multiple_of(state.gamespeed) {
        if !state.block.is_empty() {
            move_blocks(state);
        }
        clear_rows(state);
        mark_rows(state);
        if state.block.is_empty() && !state.awaiting_shape {
            effects.extend(request_next_shape(state).effects);
        }
        if !state.gameover {
            state.score += 1;
        }
    }

    Transition { effects }
}

fn handle_effect_result(
    state: &mut AppState,
    effect_id: &str,
    result: &crate::effect::EffectResult,
) -> Transition {
    if effect_id != "spawn-shape" {
        return Transition { effects: vec![] };
    }
    let crate::effect::EffectResult::RandomInt { value } = result;
    state.block = block_from_index(*value as usize, None, 0);
    state.awaiting_shape = false;
    Transition { effects: vec![] }
}

pub fn request_next_shape(state: &mut AppState) -> Transition {
    if state.awaiting_shape {
        return Transition { effects: vec![] };
    }
    state.awaiting_shape = true;
    Transition {
        effects: vec![EffectCommand::RandomInt {
            id: "spawn-shape".to_string(),
            min: 0,
            max: shape_count().saturating_sub(1),
        }],
    }
}

fn guard_move(state: &mut AppState, mutate: impl FnOnce(&mut Block)) {
    if state.paused || state.gameover || state.block.is_empty() {
        return;
    }
    let mut trial = state.block.clone();
    mutate(&mut trial);
    if is_valid(&state.board, &trial) {
        state.block = trial;
    }
}

fn move_blocks(state: &mut AppState) {
    let mut trial = state.block.clone();
    trial.y += 1;
    if is_valid(&state.board, &trial) {
        state.block = trial;
        return;
    }

    if state.block.y == 0 {
        state.paused = true;
        state.gameover = true;
        return;
    }

    state.board = merge_board_and_block(state);
    state.block = empty_block();
}

fn clear_rows(state: &mut AppState) {
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
        return;
    }
    let mut flat: Vec<String> = cleared.into_iter().flatten().collect();
    flat.splice(
        0..0,
        (0..removed * BOARD_WIDTH).map(|_| String::new()),
    );
    state.board = flat;
    let removed_u32 = u32::try_from(removed).expect("removed rows fit u32");
    state.score += 25u32.pow(removed_u32);
}

fn mark_rows(state: &mut AppState) {
    let rows: Vec<_> = state
        .board
        .chunks(BOARD_WIDTH)
        .map(|row| {
            if row.iter().all(|c| !c.is_empty() && c != HIGHLIGHT_COLOUR) {
                vec![HIGHLIGHT_COLOUR.to_string(); BOARD_WIDTH]
            } else {
                row.to_vec()
            }
        })
        .collect();
    state.board = rows.into_iter().flatten().collect();
}

fn merge_board_and_block(state: &AppState) -> Vec<String> {
    let mut board = state.board.clone();
    let block = &state.block;
    let size = block.matrix_side();
    for cell in &block.shape {
        if cell.val == 0 {
            continue;
        }
        let row = block.y + i32::try_from(cell.idx / size).expect("row offset fits i32");
        let col = block.x + i32::try_from(cell.idx % size).expect("col offset fits i32");
        if let Some(idx) = cell_index(row, col) {
            board[idx].clone_from(&block.color);
        }
    }
    board
}

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

fn rotate_block(block: &mut Block) {
    let size = block.matrix_side();
    block.shape = block
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
        .collect();
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::effect::EffectResult;

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
    fn spawn_effect_applies_block() {
        let mut state = AppState::initial();
        state.awaiting_shape = true;
        reduce(
            &mut state,
            &AppEvent::EffectCompleted {
                effect_id: "spawn-shape".to_string(),
                result: EffectResult::RandomInt { value: 2 },
            },
        );
        assert!(!state.block.is_empty());
        assert!(!state.awaiting_shape);
    }
}

use crate::state::{cell_index, AppState, BOARD_WIDTH};
use serde::{Deserialize, Serialize};

#[cfg_attr(feature = "typegen", derive(ts_rs::TS))]
#[cfg_attr(feature = "typegen", ts(rename_all = "camelCase"))]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ViewModel {
    pub rows: Vec<Vec<String>>,
    pub score: u32,
    pub paused: bool,
    pub game_over: bool,
}

pub fn select_view_model(state: &AppState) -> ViewModel {
    let merged = merge_board_and_block(state);
    let rows = merged.chunks(BOARD_WIDTH).map(<[String]>::to_vec).collect();
    ViewModel {
        rows,
        score: state.score,
        paused: state.paused,
        game_over: state.gameover,
    }
}

fn merge_board_and_block(state: &AppState) -> Vec<String> {
    let mut board = state.board.clone();
    let block = &state.block;
    if block.is_empty() {
        return board;
    }
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::shapes::block_from_index;
    use crate::state::empty_board;

    #[test]
    fn merged_board_includes_active_block() {
        let mut state = AppState::initial();
        state.board = empty_board();
        state.block = block_from_index(0, None, 0);
        let vm = select_view_model(&state);
        assert_eq!(vm.rows.len(), crate::state::BOARD_HEIGHT);
        assert!(vm.rows[0].iter().any(|c| c == "cyan"));
    }
}

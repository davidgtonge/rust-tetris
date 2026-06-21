use crate::board::merge_board_and_block;
use crate::state::{AppState, BOARD_WIDTH};
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

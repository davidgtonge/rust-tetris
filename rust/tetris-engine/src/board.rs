use crate::state::{cell_index, AppState, Block, Board, BOARD_WIDTH, HIGHLIGHT_COLOUR};

/// Board with the active block painted on top (for display and lock-in).
pub fn merge_board_and_block(state: &AppState) -> Board {
    let mut board = state.board.clone();
    if !state.block.is_empty() {
        paint_block(&mut board, &state.block);
    }
    board
}

pub fn map_rows(board: &Board, f: impl Fn(&[String]) -> Vec<String>) -> Board {
    board.chunks(BOARD_WIDTH).flat_map(f).collect()
}

fn paint_block(board: &mut Board, block: &Block) {
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
}

pub fn is_row_full(row: &[String]) -> bool {
    row.iter().all(|c| !c.is_empty() && c != HIGHLIGHT_COLOUR)
}

pub fn highlighted_row() -> Vec<String> {
    vec![HIGHLIGHT_COLOUR.to_string(); BOARD_WIDTH]
}

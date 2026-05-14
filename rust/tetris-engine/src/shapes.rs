use crate::state::{Block, ShapeCell, BOARD_WIDTH_I32};

const SHAPE_COUNT: u32 = 7;

struct ShapeDef {
    color: &'static str,
    matrix: &'static [u8],
}

const SHAPES: [ShapeDef; SHAPE_COUNT as usize] = [
    ShapeDef {
        color: "cyan",
        matrix: &[1, 0, 0, 0, 1, 0, 0, 0, 1, 0, 0, 0, 1, 0, 0, 0],
    },
    ShapeDef {
        color: "blue",
        matrix: &[0, 0, 0, 1, 1, 1, 0, 0, 1],
    },
    ShapeDef {
        color: "orange",
        matrix: &[0, 0, 0, 1, 1, 1, 1, 0, 0],
    },
    ShapeDef {
        color: "yellow",
        matrix: &[1, 1, 1, 1],
    },
    ShapeDef {
        color: "green",
        matrix: &[0, 0, 0, 0, 1, 1, 1, 1, 0],
    },
    ShapeDef {
        color: "purple",
        matrix: &[0, 0, 0, 1, 1, 1, 0, 1, 0],
    },
    ShapeDef {
        color: "red",
        matrix: &[0, 0, 0, 1, 1, 0, 0, 1, 1],
    },
];

fn to_cells(matrix: &[u8]) -> Vec<ShapeCell> {
    matrix
        .iter()
        .enumerate()
        .map(|(idx, &val)| ShapeCell { val, idx })
        .collect()
}

pub fn block_from_index(index: usize, x: Option<i32>, y: i32) -> Block {
    let def = &SHAPES[index % SHAPES.len()];
    let side = i32::try_from(def.matrix.len().isqrt()).expect("matrix side fits i32");
    let center_x = x.unwrap_or((BOARD_WIDTH_I32 - side) / 2);
    Block {
        color: def.color.to_string(),
        shape: to_cells(def.matrix),
        x: center_x,
        y,
    }
}

pub fn shape_count() -> u32 {
    SHAPE_COUNT
}

import type { ViewModel } from "../protocol/types";

const BOARD_HEIGHT = 20;
const BOARD_WIDTH = 12;

export function emptyViewModel(): ViewModel {
  return {
    rows: Array.from({ length: BOARD_HEIGHT }, () =>
      Array.from({ length: BOARD_WIDTH }, () => ""),
    ),
    score: 0,
    paused: false,
    gameOver: false,
  };
}

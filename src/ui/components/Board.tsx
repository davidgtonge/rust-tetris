const CELL_SIZE = 20;
const EMPTY = "var(--cell-empty)";

export type BoardInput = {
  rows: string[][];
  gameOver: boolean;
};

import type { MovementEvent } from "../../protocol/types";

export type BoardEvent = MovementEvent;

type BoardProps = {
  input: BoardInput;
  onEvent: (event: BoardEvent) => void;
};

function cellColor(cell: string): string {
  return cell || EMPTY;
}

export function Board({ input }: BoardProps) {
  return (
    <section className={`playfield${input.gameOver ? " gameover" : ""}`}>
      <div className="board" role="grid" aria-label="Tetris board">
        {input.rows.map((row, rowIndex) => (
          <div key={rowIndex} className="board-row" role="row">
            {row.map((cell, colIndex) => (
              <div
                key={colIndex}
                className="cell"
                role="gridcell"
                style={{
                  background: cellColor(cell),
                  width: CELL_SIZE,
                  height: CELL_SIZE,
                }}
              />
            ))}
          </div>
        ))}
      </div>
    </section>
  );
}

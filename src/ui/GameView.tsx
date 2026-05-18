import type { WireDebugState } from "../debug/wire-debug";
import type { AppEvent, ViewModel } from "../protocol/types";
import { Board } from "./components/Board";
import { Controls } from "./components/Controls";
import { DebugPanel } from "./components/DebugPanel";

type GameViewProps = {
  vm: ViewModel;
  send: (event: AppEvent) => void;
  debug: WireDebugState;
};

export function GameView({ vm, send, debug }: GameViewProps) {
  return (
    <main className="app">
      <header>
        <h1>Rust Tetris</h1>
        <p className="lede">
          Rust/Wasm canonical state · CBOR worker wire · view-model patches
        </p>
      </header>

      <Board input={{ rows: vm.rows, gameOver: vm.gameOver }} onEvent={send} />

      <p className="score" aria-live="polite">
        Score: {vm.score}
        {vm.gameOver ? " — game over" : vm.paused ? " — paused" : ""}
      </p>

      <Controls onEvent={send} />

      <p className="hint">Arrow keys also work. Events travel as CBOR bytes through a Web Worker.</p>

      <DebugPanel input={debug} />
    </main>
  );
}

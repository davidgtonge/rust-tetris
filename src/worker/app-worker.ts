import { installWasmWorker } from "@dtonge/engine-shell";
import init, { TetrisEngine } from "../../pkg/tetris_engine.js";

installWasmWorker({
  loadWasm: init,
  createEngine: () => new TetrisEngine(),
  handleInput: (engine, payload) => engine.handle_input(payload),
});

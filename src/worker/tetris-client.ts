import {
  createWorkerClient,
  decodeWorkerOutput,
  encodeWorkerInput,
  type EngineUpdate,
} from "@dtonge/engine-shell";
import type {
  AppEvent,
  EffectCommand,
  ViewModel,
  ViewModelPatch,
  WorkerInput,
  WorkerOutput,
} from "../protocol/types";

export type TetrisUpdate = EngineUpdate<ViewModel, ViewModelPatch, EffectCommand> & {
  debug?: WorkerOutput;
};

export function createTetrisWorkerClient(
  onDebug?: (output: WorkerOutput, wireMs: number) => void,
) {
  return createWorkerClient<WorkerInput, WorkerOutput, ViewModel, ViewModelPatch, EffectCommand>({
    createWorker: () =>
      new Worker(new URL("./app-worker.ts", import.meta.url), { type: "module" }),
    encodeInput: encodeWorkerInput,
    decodeOutput: decodeWorkerOutput,
    onDebug,
    parseUpdate: (output): TetrisUpdate => {
      if (output.kind === "error") {
        throw new Error(output.message);
      }
      if (output.kind === "initialized") {
        return {
          viewModel: output.viewModel,
          patches: [],
          effects: output.effects,
          diagnostics: [],
          debug: output,
        };
      }
      return {
        patches: output.patches,
        effects: output.effects,
        diagnostics: output.diagnostics,
        debug: output,
      };
    },
  });
}

export type TetrisWorkerClient = ReturnType<typeof createTetrisWorkerClient>;

export function tetrisInitInput(seed: number): WorkerInput {
  return { kind: "init", seed };
}

export function tetrisEventInput(event: AppEvent): WorkerInput {
  return { kind: "event", event };
}

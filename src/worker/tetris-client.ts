import {
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

type Job = {
  input: WorkerInput;
  resolve: (u: TetrisUpdate) => void;
  reject: (e: Error) => void;
};

function parseUpdate(output: WorkerOutput): TetrisUpdate {
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
}

/** Worker must be constructed here so Vite bundles app-worker.ts and Wasm. */
export function createTetrisWorkerClient(
  onDebug?: (output: WorkerOutput, wireMs: number) => void,
) {
  const worker = new Worker(new URL("./app-worker.ts", import.meta.url), { type: "module" });
  const jobs: Job[] = [];
  let busy = false;

  worker.onmessage = (event: MessageEvent<{ bytes: ArrayBuffer }>) => {
    const job = jobs.shift();
    if (!job) {
      busy = false;
      return;
    }

    const t0 = performance.now();
    const output = decodeWorkerOutput<WorkerOutput>(event.data.bytes);
    const elapsedMs = performance.now() - t0;
    onDebug?.(output, elapsedMs);

    try {
      job.resolve(parseUpdate(output));
    } catch (err) {
      job.reject(err instanceof Error ? err : new Error(String(err)));
    }

    busy = false;
    pump();
  };

  worker.onerror = (err) => {
    jobs.forEach((j) => j.reject(new Error(err.message || "Worker failed")));
    jobs.length = 0;
    busy = false;
  };

  function pump(): void {
    if (busy || jobs.length === 0) return;
    const job = jobs[0]!;
    busy = true;
    const bytes = encodeWorkerInput(job.input);
    const buffer = bytes.buffer.slice(bytes.byteOffset, bytes.byteOffset + bytes.byteLength) as ArrayBuffer;
    worker.postMessage({ bytes: buffer }, [buffer]);
  }

  function enqueue(input: WorkerInput): Promise<TetrisUpdate> {
    return new Promise((resolve, reject) => {
      jobs.push({ input, resolve, reject });
      pump();
    });
  }

  return {
    init: enqueue,
    dispatch: enqueue,
    dispose: () => worker.terminate(),
  };
}

export type TetrisWorkerClient = ReturnType<typeof createTetrisWorkerClient>;

export function tetrisInitInput(seed: number): WorkerInput {
  return { kind: "init", seed };
}

export function tetrisEventInput(event: AppEvent): WorkerInput {
  return { kind: "event", event };
}

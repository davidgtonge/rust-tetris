import { useCallback, useMemo, useState } from "preact/hooks";
import type { AppEvent, WorkerOutput } from "../protocol/types";
import { createTetrisWorkerClient } from "../worker/tetris-client";

export type WireDebugState = {
  lastEvent: string;
  patchCount: number;
  effectCount: number;
  wireMs: number;
  raw: WorkerOutput | null;
};

const EMPTY_DEBUG: WireDebugState = {
  lastEvent: "—",
  patchCount: 0,
  effectCount: 0,
  wireMs: 0,
  raw: null,
};

export function useWireDebug() {
  const [debug, setDebug] = useState<WireDebugState>(EMPTY_DEBUG);

  const onWorkerOutput = useCallback((output: WorkerOutput, wireMs: number) => {
    setDebug((prev) => ({
      ...prev,
      wireMs,
      raw: output,
      patchCount: output.kind === "response" ? output.patches.length : prev.patchCount,
      effectCount:
        output.kind === "response" || output.kind === "initialized"
          ? output.effects.length
          : prev.effectCount,
    }));
  }, []);

  const client = useMemo(() => createTetrisWorkerClient(onWorkerOutput), [onWorkerOutput]);

  const trackDispatch = useCallback(
    (dispatch: (event: AppEvent) => Promise<void>) =>
      (event: AppEvent) => {
        setDebug((prev) => ({ ...prev, lastEvent: event.type }));
        void dispatch(event);
      },
    [],
  );

  return { debug, client, trackDispatch };
}

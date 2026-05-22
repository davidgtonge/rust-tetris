import {
  createViewModelStore,
  useEngineRuntime,
  useSelector,
} from "@dtonge/engine-shell";
import { useEffect, useMemo } from "preact/hooks";
import { useWireDebug } from "../debug/wire-debug";
import { tetrisEffects } from "../effects";
import { tetrisEventInput, tetrisInitInput } from "../worker/tetris-client";
import { emptyViewModel } from "./empty-view-model";
import { useArrowKeys } from "./keyboard";

export function useTetrisGame() {
  const { debug, client, trackDispatch } = useWireDebug();
  const store = useMemo(() => createViewModelStore(emptyViewModel()), []);
  const vm = useSelector(store, (s) => s);

  const { ready, error, dispatch, init } = useEngineRuntime({
    store,
    client,
    effects: tetrisEffects,
    toEventInput: tetrisEventInput,
  });

  const send = useMemo(() => trackDispatch(dispatch), [trackDispatch, dispatch]);

  useEffect(() => {
    void init(tetrisInitInput((Date.now() >>> 0) as number));
  }, [init]);

  useArrowKeys(send);

  return { vm, send, ready, error, debug };
}

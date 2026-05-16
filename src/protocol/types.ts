/**
 * Protocol types generated from Rust (`src/generated/engine-types.ts`).
 */

export type {
  AppEvent,
  EffectCommand,
  EffectResult,
  PatchSegment,
  ViewModel,
  ViewModelPatch,
  WorkerInput,
  WorkerOutput,
} from "../generated/engine-types";

export type { WireMessage } from "@dtonge/engine-shell";

import type { PatchSegment } from "../generated/engine-types";

/** Path segment for generic view-model patches. */
export type Path = PatchSegment[];

/** Events a movement control can emit. */
export type MovementEvent = Extract<
  import("../generated/engine-types").AppEvent,
  { type: "moveLeft" | "moveRight" | "moveDown" | "rotate" }
>;

/** Events game controls can emit. */
export type GameControlEvent = Extract<
  import("../generated/engine-types").AppEvent,
  { type: "pause" | "resume" | "restart" }
>;

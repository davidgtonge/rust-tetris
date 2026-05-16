import { createBuiltinEffectRegistry } from "@dtonge/engine-shell";
import type { AppEvent, EffectCommand } from "./protocol/types";

export const tetrisEffects = createBuiltinEffectRegistry<EffectCommand, AppEvent>({
  match: (effect) => {
    if (
      effect.type === "timerStart" ||
      effect.type === "timerStop" ||
      effect.type === "randomInt"
    ) {
      return effect;
    }
    return null;
  },
  onTimerTick: () => ({ type: "tick" }),
  onRandomInt: (effectId, result) => ({
    type: "effectCompleted",
    effectId,
    result,
  }),
});

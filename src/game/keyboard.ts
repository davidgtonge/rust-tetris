import { useEffect } from "preact/hooks";
import type { AppEvent } from "../protocol/types";

const KEY_EVENTS: Record<string, AppEvent> = {
  ArrowUp: { type: "rotate" },
  ArrowDown: { type: "moveDown" },
  ArrowLeft: { type: "moveLeft" },
  ArrowRight: { type: "moveRight" },
};

export function useArrowKeys(send: (event: AppEvent) => void): void {
  useEffect(() => {
    const onKeyDown = (event: KeyboardEvent) => {
      const mapped = KEY_EVENTS[event.key];
      if (!mapped) return;
      event.preventDefault();
      send(mapped);
    };
    window.addEventListener("keydown", onKeyDown);
    return () => window.removeEventListener("keydown", onKeyDown);
  }, [send]);
}

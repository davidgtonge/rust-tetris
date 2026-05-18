import type { GameControlEvent, MovementEvent } from "../../protocol/types";

export type ControlsEvent = MovementEvent | GameControlEvent;

type ControlsProps = {
  onEvent: (event: ControlsEvent) => void;
};

export function Controls({ onEvent }: ControlsProps) {
  return (
    <div className="controls">
      <div className="button-row" role="group" aria-label="Movement">
        <button type="button" onClick={() => onEvent({ type: "rotate" })}>
          Rotate
        </button>
        <button type="button" onClick={() => onEvent({ type: "moveLeft" })}>
          Left
        </button>
        <button type="button" onClick={() => onEvent({ type: "moveRight" })}>
          Right
        </button>
        <button type="button" onClick={() => onEvent({ type: "moveDown" })}>
          Down
        </button>
      </div>
      <div className="button-row" role="group" aria-label="Game">
        <button type="button" onClick={() => onEvent({ type: "pause" })}>
          Pause
        </button>
        <button type="button" onClick={() => onEvent({ type: "resume" })}>
          Resume
        </button>
        <button type="button" onClick={() => onEvent({ type: "restart" })}>
          Restart
        </button>
      </div>
    </div>
  );
}

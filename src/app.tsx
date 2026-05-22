import { useTetrisGame } from "./game/use-tetris-game";
import { GameView } from "./ui/GameView";
import { StatusScreen } from "./ui/StatusScreen";

export function App() {
  const { vm, send, ready, error, debug } = useTetrisGame();

  if (error) return <StatusScreen kind="error" message={error} />;
  if (!ready) return <StatusScreen kind="loading" />;

  return <GameView vm={vm} send={send} debug={debug} />;
}

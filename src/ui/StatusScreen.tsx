type StatusScreenProps =
  | { kind: "loading" }
  | { kind: "error"; message: string };

export function StatusScreen(props: StatusScreenProps) {
  return (
    <main className="app">
      {props.kind === "error" ? (
        <p className="error">Engine error: {props.message}</p>
      ) : (
        <p className="hint">Loading Wasm engine…</p>
      )}
    </main>
  );
}

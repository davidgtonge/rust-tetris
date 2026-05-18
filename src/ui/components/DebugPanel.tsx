import type { WireDebugState } from "../../debug/wire-debug";

export function DebugPanel({ input }: { input: WireDebugState }) {
  return (
    <aside className="debug" aria-label="Wire debug">
      <h2>Wire</h2>
      <dl>
        <div>
          <dt>Last event</dt>
          <dd>{input.lastEvent}</dd>
        </div>
        <div>
          <dt>Patches</dt>
          <dd>{input.patchCount}</dd>
        </div>
        <div>
          <dt>Effects</dt>
          <dd>{input.effectCount}</dd>
        </div>
        <div>
          <dt>Decode (main)</dt>
          <dd>{input.wireMs.toFixed(2)} ms</dd>
        </div>
      </dl>
      {import.meta.env.DEV && input.raw ? (
        <pre className="debug-raw">{JSON.stringify(input.raw, null, 2)}</pre>
      ) : null}
    </aside>
  );
}

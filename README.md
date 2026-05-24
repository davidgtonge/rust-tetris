# Rust Tetris

Tetris with a **Rust/Wasm engine** in a Web Worker, **CBOR-first** wire encoding, and **view-model patches** on the main thread. Game rules are ported from [redux-tetris](https://github.com/davidgtonge/redux-tetris).

**Live demo:** https://davidgtonge.github.io/rust-tetris/

## Architecture

```txt
Preact UI  →  CBOR AppEvent  →  Worker  →  Wasm engine
                ↑                              ↓
         applyPatchBatch  ←  CBOR patches + effects  ←
```

- Canonical `AppState` lives in Wasm (Rust).
- Main thread holds only a renderable `ViewModel`.
- Effects (`timer`, `random`) run on the main thread and complete as events.

Built on [@dtonge/engine-shell](https://github.com/davidgtonge/engine-shell) — shared Rust/Wasm + TypeScript worker scaffold.

## Quick start

```bash
git clone --recurse-submodules https://github.com/davidgtonge/rust-tetris.git
cd rust-tetris
npm install
npm run dev        # http://localhost:5173
npm run build
npm run test:rust
```

`AppEvent`, `EffectCommand`, `ViewModel`, and other wire types are defined in Rust and exported with **ts-rs**. Do not edit `src/generated/engine-types.ts` by hand.

Requires [wasm-pack](https://rustwasm.github.io/wasm-pack/installer/) for `npm run build:wasm`.

## Stack

- Rust: `tetris-engine` (serde + ciborium + wasm-bindgen)
- Preact + Vite
- CBOR wire encoding between worker and main thread

## License

MIT

mod effect;
mod engine;
mod event;
mod protocol;
mod shapes;
mod state;
mod update;
mod view_model;

#[cfg(feature = "typegen")]
mod typegen;

#[cfg(feature = "typegen")]
pub use typegen::export_types;

pub use effect::{EffectCommand, EffectResult};
pub use engine::Engine;
pub use event::AppEvent;
pub use engine_kernel::ViewModelPatch;
pub use protocol::{decode_input, decode_output, encode_input, encode_output, WorkerInput, WorkerOutput};
pub use view_model::ViewModel;

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct TetrisEngine {
    inner: Engine,
}

impl Default for TetrisEngine {
    fn default() -> Self {
        Self::new()
    }
}

#[wasm_bindgen]
impl TetrisEngine {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        Self {
            inner: Engine::new(),
        }
    }

    /// CBOR `WorkerInput` bytes in, CBOR `WorkerOutput` bytes out.
    pub fn handle_input(&mut self, payload: &[u8]) -> Vec<u8> {
        match decode_input(payload) {
            Ok(input) => encode_output(&self.inner.dispatch(&input)),
            Err(message) => encode_output(&WorkerOutput::Error { message }),
        }
    }
}

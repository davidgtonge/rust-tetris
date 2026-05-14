use crate::effect::EffectCommand;
use crate::event::AppEvent;
use engine_kernel::ViewModelPatch;
use crate::view_model::ViewModel;
use serde::{Deserialize, Serialize};

#[cfg_attr(feature = "typegen", derive(ts_rs::TS))]
#[cfg_attr(
    feature = "typegen",
    ts(tag = "kind", rename_all = "camelCase")
)]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "camelCase")]
pub enum WorkerInput {
    Init { seed: u32 },
    Event { event: AppEvent },
}

#[cfg_attr(feature = "typegen", derive(ts_rs::TS))]
#[cfg_attr(
    feature = "typegen",
    ts(tag = "kind", rename_all = "camelCase")
)]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "camelCase")]
pub enum WorkerOutput {
    Initialized {
        #[serde(rename = "viewModel")]
        #[cfg_attr(feature = "typegen", ts(rename = "viewModel"))]
        view_model: ViewModel,
        effects: Vec<EffectCommand>,
    },
    Response {
        patches: Vec<ViewModelPatch>,
        effects: Vec<EffectCommand>,
        diagnostics: Vec<String>,
    },
    Error { message: String },
}

pub fn encode_input(input: &WorkerInput) -> Vec<u8> {
    let mut buf = Vec::new();
    ciborium::into_writer(input, &mut buf).expect("encode WorkerInput");
    buf
}

pub fn decode_input(bytes: &[u8]) -> Result<WorkerInput, String> {
    ciborium::from_reader(bytes).map_err(|e| e.to_string())
}

pub fn encode_output(output: &WorkerOutput) -> Vec<u8> {
    let mut buf = Vec::new();
    ciborium::into_writer(output, &mut buf).expect("encode WorkerOutput");
    buf
}

pub fn decode_output(bytes: &[u8]) -> Result<WorkerOutput, String> {
    ciborium::from_reader(bytes).map_err(|e| e.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::state::AppState;
    use crate::view_model::select_view_model;

    #[test]
    fn cbor_round_trip_init() {
        let input = WorkerInput::Init { seed: 42 };
        let bytes = encode_input(&input);
        let decoded = decode_input(&bytes).unwrap();
        assert_eq!(decoded, input);
    }

    #[test]
    fn cbor_round_trip_response() {
        let vm = select_view_model(&AppState::initial());
        let output = WorkerOutput::Initialized {
            view_model: vm,
            effects: vec![],
        };
        let bytes = encode_output(&output);
        let decoded = decode_output(&bytes).unwrap();
        assert_eq!(decoded, output);
    }
}

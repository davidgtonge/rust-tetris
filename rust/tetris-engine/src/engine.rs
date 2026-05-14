use engine_kernel::diff_serializable_checked;
use crate::effect::EffectCommand;
use crate::protocol::{WorkerInput, WorkerOutput};
use crate::state::AppState;
use crate::update::{reduce, request_next_shape};
use crate::view_model::{select_view_model, ViewModel};

const TICK_INTERVAL_MS: u32 = 16;

pub struct Engine {
    state: AppState,
    view_model: ViewModel,
}

impl Default for Engine {
    fn default() -> Self {
        Self::new()
    }
}

impl Engine {
    pub fn new() -> Self {
        let state = AppState::initial();
        let view_model = select_view_model(&state);
        Self { state, view_model }
    }

    pub fn dispatch(&mut self, input: &WorkerInput) -> WorkerOutput {
        match input {
            WorkerInput::Init { seed: _ } => {
                self.state = AppState::initial();
                let mut effects = startup_effects();
                effects.extend(request_next_shape(&mut self.state).effects);
                self.view_model = select_view_model(&self.state);
                WorkerOutput::Initialized {
                    view_model: self.view_model.clone(),
                    effects,
                }
            }
            WorkerInput::Event { event } => {
                let prev_vm = self.view_model.clone();
                let transition = reduce(&mut self.state, event);
                self.view_model = select_view_model(&self.state);
                let patches = diff_serializable_checked(&prev_vm, &self.view_model);
                WorkerOutput::Response {
                    patches,
                    effects: transition.effects,
                    diagnostics: vec![],
                }
            }
        }
    }
}

fn startup_effects() -> Vec<EffectCommand> {
    vec![EffectCommand::TimerStart {
        id: "tick".to_string(),
        interval_ms: TICK_INTERVAL_MS,
    }]
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::protocol::decode_output;
    use crate::event::AppEvent;

    #[test]
    fn move_left_produces_patches_or_empty() {
        let mut engine = Engine::new();
        let init = WorkerInput::Init { seed: 0 };
        let _ = engine.dispatch(&init);
        let event = WorkerInput::Event {
            event: AppEvent::MoveLeft,
        };
        let out = engine.dispatch(&event);
        match out {
            WorkerOutput::Response { .. } => {}
            other => panic!("unexpected {other:?}"),
        }
    }

    #[test]
    fn cbor_handle_input_round_trip() {
        let mut engine = Engine::new();
        let init = WorkerInput::Init { seed: 1 };
        engine.dispatch(&init);
        let tick = WorkerInput::Event {
            event: AppEvent::Tick,
        };
        let output = engine.dispatch(&tick);
        let bytes = crate::protocol::encode_output(&output);
        let decoded = decode_output(&bytes).unwrap();
        match decoded {
            WorkerOutput::Response { .. } => {}
            other => panic!("unexpected {other:?}"),
        }
    }
}

use crate::effect::EffectResult;
use serde::{Deserialize, Serialize};

#[cfg_attr(feature = "typegen", derive(ts_rs::TS))]
#[cfg_attr(
    feature = "typegen",
    ts(tag = "type", rename_all = "camelCase")
)]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "camelCase")]
pub enum AppEvent {
    Tick,
    MoveLeft,
    MoveRight,
    MoveDown,
    Rotate,
    Pause,
    Resume,
    Restart,
    EffectCompleted {
        #[serde(rename = "effectId")]
        effect_id: String,
        result: EffectResult,
    },
}

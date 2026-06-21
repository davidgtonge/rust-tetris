use serde::{Deserialize, Serialize};

#[cfg_attr(feature = "typegen", derive(ts_rs::TS))]
#[cfg_attr(
    feature = "typegen",
    ts(tag = "type", rename_all = "camelCase")
)]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "camelCase")]
pub enum EffectCommand {
    TimerStart {
        id: String,
        #[serde(rename = "intervalMs")]
        interval_ms: u32,
    },
    TimerStop {
        id: String,
    },
}

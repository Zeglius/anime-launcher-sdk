use serde::{Serialize, Deserialize};
use serde_json::Value as JsonValue;

use super::window_mode::WindowMode;
use super::fps::Fps;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct FpsUnlockerConfig {
    pub fps: Fps,
    pub power_saving: bool,
    pub monitor: u64,
    pub window_mode: WindowMode,
    pub priority: u64
}

impl Default for FpsUnlockerConfig {
    #[inline]
    fn default() -> Self {
        Self {
            fps: Fps::HundredTwenty,
            power_saving: false,
            monitor: 1,
            window_mode: WindowMode::default(),
            priority: 3
        }
    }
}

impl From<&JsonValue> for FpsUnlockerConfig {
    fn from(value: &JsonValue) -> Self {
        let default = Self::default();

        Self {
            fps: match value.get("fps") {
                Some(value) => value.as_u64().map(Fps::from_num).unwrap_or(default.fps),
                None => default.fps
            },

            power_saving: match value.get("power_saving") {
                Some(value) => value.as_bool().unwrap_or(default.power_saving),
                None => default.power_saving
            },

            monitor: match value.get("monitor") {
                Some(value) => value.as_u64().unwrap_or(default.monitor),
                None => default.monitor
            },

            window_mode: match value.get("window_mode") {
                Some(value) => WindowMode::from(value),
                None => default.window_mode
            },

            priority: match value.get("priority") {
                Some(value) => value.as_u64().unwrap_or(default.priority),
                None => default.priority
            }
        }
    }
}
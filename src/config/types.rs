use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::ui;

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct Config {
    pub license: Option<String>,
    pub ui: Option<ui::types::UI>,
}

impl Config {
    pub fn merge(self, other: Self) -> Self {
        Self {
            license: other.license.or(self.license),
            ui: match (self.ui, other.ui) {
                (Some(base), Some(override_)) => Some(base.merge(override_)),
                (None, Some(override_)) => Some(override_),
                (Some(base), None) => Some(base),
                (None, None) => None,
            },
        }
    }
}

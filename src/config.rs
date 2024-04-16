//! Module holding everything related to the config file.

use crate::types::Config;

impl Default for Config {
    fn default() -> Self {
        Self {
            column_width: 100,
        }
    }
}

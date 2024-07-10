//! Account Field

use serde::{Deserialize, Serialize};

/// Account field
#[derive(Default, Debug, Serialize, Deserialize, PartialEq, Eq, Clone)]
pub struct Field {
    pub name: String,
    pub value: String,
    pub verified_at: String,
}

use serde::{Deserialize, Serialize};

use super::item::InternalItem;

#[derive(Debug, Deserialize, Serialize)]
#[deprecated(note = "replace with external implementation")]
pub struct InternalResponse {
    pub items: Vec<InternalItem>,
}

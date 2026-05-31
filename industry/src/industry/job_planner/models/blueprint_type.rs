use serde::{Deserialize, Serialize};

#[derive
(
    Copy, Clone, Debug,
    Eq, PartialEq,
    Deserialize, Serialize,
)]
pub enum BlueprintTyp {
    Blueprint,
    Reaction,
    Material,
}

use serde::Serialize;
use starfoundry_libs_types::TypeId;

#[derive(Copy, Clone, Debug, Serialize)]
pub struct BlueprintBonus {
    pub ptype_id: TypeId,
    pub material: f32,
    pub time:     f32,
}

impl BlueprintBonus {
    pub fn no_bonus(ptype_id: TypeId) -> Self {
        Self {
            ptype_id,
            material: 0f32,
            time:     0f32,
        }
    }
}

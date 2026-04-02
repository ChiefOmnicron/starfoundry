use serde::Serialize;
use starfoundry_lib_types::TypeId;

#[derive(Copy, Clone, Debug, Serialize)]
pub struct BlueprintBonus {
    pub type_id:    TypeId,
    pub material:   f32,
    pub time:       f32,
}

impl BlueprintBonus {
    pub fn no_bonus(
        type_id: TypeId
    ) -> Self {
        Self {
            type_id:    type_id,
            material:   0f32,
            time:       0f32,
        }
    }

    pub fn new(
        type_id:    TypeId,
        material:   f32,
        time:       f32,
    ) -> Self {
        Self {
            type_id,
            material,
            time,
        }
    }
}

use serde::{Serialize, Serializer};
use starfoundry_libs_types::{SystemId, TypeId};
use utoipa::ToSchema;

use crate::{Security, StructureRig, StructureType, StructureUuid};
use serde::ser::SerializeSeq;

#[derive(Clone, Debug, Serialize, ToSchema)]
#[schema(
    example = json!({
        "id": "d187ee9c-3afe-4e72-a5b4-646020b0d646",
        "name": "1DQ1-A - 1-st Imperial Palace",
        "rigs": [
            37275
        ],
        "security": "NULLSEC",
        "services": [
            35894
        ],
        "structure_id": 1003520240,
        "structure_type_id": 35834,
        "system_id": 30004759
    })
)]
pub struct Structure {
    /// Internal id of the structure
    pub id:                StructureUuid,
    /// Name of the structure
    pub name:              String,
    /// Location of the strucutre
    pub system_id:         SystemId,
    /// Security of the location the structure is in
    pub security:          Security,

    /// Type of structure
    #[serde(deserialize_with = "StructureType::deserialize")]
    #[serde(serialize_with = "StructureType::serialize")]
    #[serde(rename = "structure_type_id")]
    #[schema(
        value_type = TypeId,
        example = "565692"
    )]
    pub structure_type:    StructureType,
    /// List of all rigs that are in the structure
    #[serde(serialize_with = "serialize_rigs")]
    pub rigs:              Vec<StructureRig>,
    /// Id of the structure in-game
    pub services:          Vec<TypeId>,

    /// EVE Id of the structure
    pub structure_id:      i64,
}

impl Structure {
    pub fn category_groups(
        &self,
    ) -> Vec<i32> {
        let mut base_bonus = self.structure_type.category_groups();

        let rig_bonus = self
            .rigs
            .iter()
            .map(|x| x.category_groups.clone())
            .flatten()
            .collect::<Vec<_>>();

        base_bonus.extend(&rig_bonus);
        base_bonus
    }

    pub fn rig_bonus_by_security(
        &self
    ) -> f32 {
        match (&self.security, &self.structure_type) {
            // Refinery
            (Security::Highsec, StructureType::Athanor) |
            (Security::Highsec, StructureType::Tatara)  => 0f32,
            (Security::Lowsec,  StructureType::Athanor) |
            (Security::Lowsec,  StructureType::Tatara)  => 1.0f32,
            (Security::Nullsec, StructureType::Athanor) |
            (Security::Nullsec, StructureType::Tatara)  => 1.1f32,

            // Engineering
            (Security::Highsec, StructureType::Raitaru) |
            (Security::Highsec, StructureType::Azbel)   |
            (Security::Highsec, StructureType::Sotiyo)  => 1.0f32,
            (Security::Lowsec,  StructureType::Raitaru) |
            (Security::Lowsec,  StructureType::Azbel)   |
            (Security::Lowsec,  StructureType::Sotiyo)  => 1.9f32,
            (Security::Nullsec, StructureType::Raitaru) |
            (Security::Nullsec, StructureType::Azbel)   |
            (Security::Nullsec, StructureType::Sotiyo)  => 2.1f32,

            // Invalid
            _                                           => 0f32,
        }
    }

    pub fn rigs(
        &self,
    ) -> Vec<StructureRig> {
        self
            .rigs
            .iter()
            .map(|x| {
                x.material.map(|y| y * self.rig_bonus_by_security());
                x.time.map(|y| y * self.rig_bonus_by_security());

                StructureRig {
                    type_id:         x.type_id,
                    material:        x.material.map(|y| y * self.rig_bonus_by_security()),
                    time:            x.time.map(|y| y * self.rig_bonus_by_security()),
                    name:            x.name.clone(),
                    category_groups: x.category_groups.clone(),
                }
            })
            .collect::<Vec<_>>()
    }
}

fn serialize_rigs<S>(
    value: &Vec<StructureRig>,
    serializer: S,
) -> Result<S::Ok, S::Error>
    where
        S: Serializer {

    let mut seq = serializer.serialize_seq(Some(value.len()))?;

    for entry in value {
        seq.serialize_element(&entry.type_id)?;
    }

    seq.end()
}

use starfoundry_lib_types::{AllianceId, CharacterId, CorporationId, TypeId};
use serde::{Deserialize, Serialize};
use crate::product::error::{ProductError, Result};
use serde_json::{json, Value};

/// Returns true if the character/corporation/alliance is on the blacklist
/// Returns false if they are not on the blacklist
pub fn check_blacklist(
    character_id:   CharacterId,
    corporation_id: CorporationId,
    alliance_id:    Option<AllianceId>,
    blacklist:      Vec<u64>,
) -> bool {
    let character_id = *character_id as u64;
    let corporation_id = *corporation_id as u64;

    let blacklist_character_id = blacklist.contains(&character_id);
    if blacklist_character_id {
        return true;
    }

    let blacklist_corporation_id = blacklist.contains(&corporation_id);
    if blacklist_corporation_id {
        return true
    }

    if let Some(x) = alliance_id {
        if blacklist.contains(&(*x as u64)) {
            return true;
        }
    };

    false
}

/// Returns true if the character/corporation/alliance is on the whitelist
/// Returns false if they are not on the whitelist
pub fn check_whitelist(
    character_id:   CharacterId,
    corporation_id: CorporationId,
    alliance_id:    Option<AllianceId>,
    whitelist:      Vec<u64>,
) -> bool {
    let character_id = *character_id as u64;
    let corporation_id = *corporation_id as u64;

    let whitelist_character_id = whitelist.contains(&character_id);
    let whitelist_corporation_id = whitelist.contains(&corporation_id);
    let whitelist_alliance_id = if let Some(x) = alliance_id {
        if whitelist.contains(&(*x as u64)) {
            true
        } else {
            false
        }
    } else {
        false
    };

    whitelist_character_id || whitelist_corporation_id || whitelist_alliance_id
}

pub async fn resolve_items(content: Value) -> Result<Value> {
    #[derive(Deserialize)]
    struct AppraisalResult {
        items: Vec<AppraisalItemResult>,
    }

    #[derive(Deserialize)]
    struct AppraisalItemResult {
        meta: AppraisalMetaResult,
        quantity: f32,
    }

    #[derive(Deserialize)]
    struct AppraisalMetaResult {
        name: String,
        type_id: TypeId,
    }

    #[derive(Serialize)]
    struct Item {
        pub name: String,
        pub type_id: TypeId,
        pub quantity: f32,
    }

    let appraisal: AppraisalResult = reqwest::Client::new()
        .post("https://api.appraisal.starfoundry.space/appraisals")
        .json(&json!({
            "appraisal": content,
            "market": 60003760,
            "persist": "NonPersist",
        }))
        .send()
        .await
        .map_err(ProductError::GeneralReqwestError)?
        .json()
        .await
        .map_err(ProductError::GeneralReqwestError)?;
    let appraisal = appraisal
        .items
        .into_iter()
        .map(|x| {
            Item {
                name: x.meta.name,
                quantity: x.quantity,
                type_id: x.meta.type_id,
            }
        })
        .collect::<Vec<_>>();
    serde_json::to_value(&appraisal).map_err(ProductError::GeneralSerdeError)
}


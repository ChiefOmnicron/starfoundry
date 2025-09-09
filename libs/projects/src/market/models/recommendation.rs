use serde::Serialize;
use starfoundry_lib_structures::StructureUuid;
use utoipa::ToSchema;
use uuid::Uuid;

#[derive(Clone, Debug, Serialize, ToSchema)]
#[schema(
    example = json!({
        "item_name": "Fullerite-C28",
        "source": "Jita 4-4",
        "type_id": 30375,
        "quantity": 100,
        "remaining": 1000,
        "volume": 2,
    })
)]
pub struct MarketRecommendation {
    pub structure_id: StructureUuid,
    pub item_name:    String,
    pub source:       String,
    pub type_id:      i32,
    pub quantity:     u64,
    pub remaining:    u64,
    pub price:        f64,
    pub volume:       f32,
}

impl Default for MarketRecommendation {
    fn default() -> Self {
        Self {
            structure_id: StructureUuid::new(Uuid::new_v4()),
            item_name:    String::default(),
            source:       String::default(),
            type_id:      i32::default(),
            quantity:     u64::default(),
            remaining:    u64::default(),
            price:        f64::default(),
            volume:       f32::default(),
        }
    }
}

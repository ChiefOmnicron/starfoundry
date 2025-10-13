use serde::Deserialize;

#[derive(Clone, Debug, Deserialize)]
pub struct CharacterInfo {
    pub character_id:     i32,
    pub corporation_id:   i32,
    pub alliance_id:      Option<i32>,
}

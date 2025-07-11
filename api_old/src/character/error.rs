use std::fmt;

use crate::auth::AuthError;

#[derive(Debug)]
pub enum CharacterError {
    CreateEveClient(starfoundry_libs_eve_api::Error),
    FetchAlts(sqlx::Error),
    FetchCharacter(sqlx::Error),
    FetchCharacterAlliance(starfoundry_libs_eve_api::Error),
    FetchCharacterCoporation(starfoundry_libs_eve_api::Error),
    RemoveCharacter(sqlx::Error),
    RemoveCharacterLogin(sqlx::Error),
    SaveCharacter(sqlx::Error),

    GetCharacterId(AuthError),
    GetCorporationId(AuthError),
    GetEveAuthClient(AuthError),
    FetchCharacterBlueprints(starfoundry_libs_eve_api::Error),
    FetchCorporationBlueprints(starfoundry_libs_eve_api::Error),
}

impl warp::reject::Reject for CharacterError { }

impl fmt::Display for CharacterError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

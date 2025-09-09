use sqlx::PgPool;
use starfoundry_lib_eve_api::EveApiClient;
use starfoundry_lib_types::StructureId;

use crate::{Error, ResolvedStructure, Result, Security};

pub async fn resolve_player_structure(
    pool:         &PgPool,
    client:       EveApiClient,
    structure_id: StructureId,
) -> Result<ResolvedStructure> {
    if *structure_id <= 1_000_000_000_000 {
        return Err(Error::InvalidStructureId(structure_id))
    }

    let (structure_id, structure) = client
        .resolve_structure(structure_id.into())
        .await
        .map_err(|e| Error::FetchPlayerStructureFromEve(e, structure_id))?;

    let security = sqlx::query!("
                SELECT security
                FROM system
                WHERE system_id = $1
            ",
            *structure.system_id,
        )
        .fetch_one(pool)
        .await
        .map_err(|e| Error::FetchPlayerStructureSystemSecurity(e, structure_id))?
        .security;

    let security = if security <= 0.0 {
        Security::Nullsec
    } else if security > 0.0 && security <= 0.5 {
        Security::Lowsec
    } else {
        Security::Highsec
    };

    Ok(ResolvedStructure {
        structure_id: structure_id,
        security:     security,
        system_id:    structure.system_id,
        name:         structure.name,
        type_id:      structure.type_id,
    })
}

use good_lp::{constraint, default_solver, variables, Solution, SolverModel};
use std::collections::HashMap;

use crate::{Config, Gas};
use crate::{Error, Result};

pub fn calculate_gas(
    config: &Config,
) -> Result<HashMap<i32, f64>> {
    variables! {
        vars:
            amber_cytoserocin;
            amber_mykoserocin;
            compressed_amber_cytoserocin;
            compressed_amber_mykoserocin;

            azure_cytoserocin;
            azure_mykoserocin;
            compressed_azure_cytoserocin;
            compressed_azure_mykoserocin;

            celadon_cytoserocin;
            celadon_mykoserocin;
            compressed_celadon_cytoserocin;
            compressed_celadon_mykoserocin;

            golden_cytoserocin;
            golden_mykoserocin;
            compressed_golden_cytoserocin;
            compressed_golden_mykoserocin;

            lime_cytoserocin;
            lime_mykoserocin;
            compressed_lime_cytoserocin;
            compressed_lime_mykoserocin;

            malachite_cytoserocin;
            malachite_mykoserocin;
            compressed_malachite_cytoserocin;
            compressed_malachite_mykoserocin;

            vermillion_cytoserocin;
            vermillion_mykoserocin;
            compressed_vermillion_cytoserocin;
            compressed_vermillion_mykoserocin;

            viridian_cytoserocin;
            viridian_mykoserocin;
            compressed_viridian_cytoserocin;
            compressed_viridian_mykoserocin;

            fullerite_c28;
            compressed_fullerite_c28;

            fullerite_c32;
            compressed_fullerite_c32;

            fullerite_c320;
            compressed_fullerite_c320;

            fullerite_c50;
            compressed_fullerite_c50;

            fullerite_c540;
            compressed_fullerite_c540;

            fullerite_c60;
            compressed_fullerite_c60;

            fullerite_c70;
            compressed_fullerite_c70;

            fullerite_c72;
            compressed_fullerite_c72;

            fullerite_c84;
            compressed_fullerite_c84;
    }

    // create the problem, find the lowest possible price while still fulfulling
    // the wanted minerals
    let mut problem = vars.minimise(
            (amber_cytoserocin                  * config.gas_price(&Gas::AmberCytoserocin)) +
            (amber_mykoserocin                  * config.gas_price(&Gas::AmberMykoserocin)) +
            (compressed_amber_cytoserocin       * config.gas_price(&Gas::CompressedAmberCytoserocin)) +
            (compressed_amber_mykoserocin       * config.gas_price(&Gas::CompressedAmberMykoserocin)) +

            (azure_cytoserocin                  * config.gas_price(&Gas::AzureCytoserocin)) +
            (azure_mykoserocin                  * config.gas_price(&Gas::AzureMykoserocin)) +
            (compressed_azure_cytoserocin       * config.gas_price(&Gas::CompressedAzureCytoserocin)) +
            (compressed_azure_mykoserocin       * config.gas_price(&Gas::CompressedAzureMykoserocin)) +

            (celadon_cytoserocin                * config.gas_price(&Gas::CeladonCytoserocin)) +
            (celadon_mykoserocin                * config.gas_price(&Gas::CeladonMykoserocin)) +
            (compressed_celadon_cytoserocin     * config.gas_price(&Gas::CompressedCeladonCytoserocin)) +
            (compressed_celadon_mykoserocin     * config.gas_price(&Gas::CompressedCeladonMykoserocin)) +

            (golden_cytoserocin                 * config.gas_price(&Gas::GoldenCytoserocin)) +
            (golden_mykoserocin                 * config.gas_price(&Gas::GoldenMykoserocin)) +
            (compressed_golden_cytoserocin      * config.gas_price(&Gas::CompressedGoldenCytoserocin)) +
            (compressed_golden_mykoserocin      * config.gas_price(&Gas::CompressedGoldenMykoserocin)) +

            (lime_cytoserocin                   * config.gas_price(&Gas::LimeCytoserocin)) +
            (lime_mykoserocin                   * config.gas_price(&Gas::LimeMykoserocin)) +
            (compressed_lime_cytoserocin        * config.gas_price(&Gas::CompressedLimeCytoserocin)) +
            (compressed_lime_mykoserocin        * config.gas_price(&Gas::CompressedLimeMykoserocin)) +

            (malachite_cytoserocin              * config.gas_price(&Gas::MalachiteCytoserocin)) +
            (malachite_mykoserocin              * config.gas_price(&Gas::MalachiteMykoserocin)) +
            (compressed_malachite_cytoserocin   * config.gas_price(&Gas::CompressedMalachiteCytoserocin)) +
            (compressed_malachite_mykoserocin   * config.gas_price(&Gas::CompressedMalachiteMykoserocin)) +

            (vermillion_cytoserocin             * config.gas_price(&Gas::VermillionCytoserocin)) +
            (vermillion_mykoserocin             * config.gas_price(&Gas::VermillionMykoserocin)) +
            (compressed_vermillion_cytoserocin  * config.gas_price(&Gas::CompressedVermillionCytoserocin)) +
            (compressed_vermillion_mykoserocin  * config.gas_price(&Gas::CompressedVermillionMykoserocin)) +

            (viridian_cytoserocin               * config.gas_price(&Gas::ViridianCytoserocin)) +
            (viridian_mykoserocin               * config.gas_price(&Gas::ViridianMykoserocin)) +
            (compressed_viridian_cytoserocin    * config.gas_price(&Gas::CompressedViridianCytoserocin)) +
            (compressed_viridian_mykoserocin    * config.gas_price(&Gas::CompressedViridianMykoserocin)) +

            (fullerite_c28                      * config.gas_price(&Gas::FulleriteC28)) +
            (compressed_fullerite_c28           * config.gas_price(&Gas::CompressedFulleriteC28)) +

            (fullerite_c32                      * config.gas_price(&Gas::FulleriteC32)) +
            (compressed_fullerite_c32           * config.gas_price(&Gas::CompressedFulleriteC32)) +

            (fullerite_c320                     * config.gas_price(&Gas::FulleriteC320)) +
            (compressed_fullerite_c320          * config.gas_price(&Gas::CompressedFulleriteC320)) +

            (fullerite_c50                      * config.gas_price(&Gas::FulleriteC50)) +
            (compressed_fullerite_c50           * config.gas_price(&Gas::CompressedFulleriteC50)) +

            (fullerite_c540                     * config.gas_price(&Gas::FulleriteC540)) +
            (compressed_fullerite_c540          * config.gas_price(&Gas::CompressedFulleriteC540)) +

            (fullerite_c60                      * config.gas_price(&Gas::FulleriteC60)) +
            (compressed_fullerite_c60           * config.gas_price(&Gas::CompressedFulleriteC60)) +

            (fullerite_c70                      * config.gas_price(&Gas::FulleriteC70)) +
            (compressed_fullerite_c70           * config.gas_price(&Gas::CompressedFulleriteC70)) +

            (fullerite_c72                      * config.gas_price(&Gas::FulleriteC72)) +
            (compressed_fullerite_c72           * config.gas_price(&Gas::CompressedFulleriteC72)) +

            (fullerite_c84                      * config.gas_price(&Gas::FulleriteC84)) +
            (compressed_fullerite_c84           * config.gas_price(&Gas::CompressedFulleriteC84))
        )
        .using(default_solver);

    let entries = vec![
        (Gas::AmberCytoserocin, amber_cytoserocin),
        (Gas::AmberMykoserocin, amber_mykoserocin),
        (Gas::CompressedAmberCytoserocin, compressed_amber_cytoserocin),
        (Gas::CompressedAmberMykoserocin, compressed_amber_mykoserocin),
        (Gas::AzureCytoserocin, azure_cytoserocin),
        (Gas::AzureMykoserocin, azure_mykoserocin),
        (Gas::CompressedAzureCytoserocin, compressed_azure_cytoserocin),
        (Gas::CompressedAzureMykoserocin, compressed_azure_mykoserocin),
        (Gas::CeladonCytoserocin, celadon_cytoserocin),
        (Gas::CeladonMykoserocin, celadon_mykoserocin),
        (Gas::CompressedCeladonCytoserocin, compressed_celadon_cytoserocin),
        (Gas::CompressedCeladonMykoserocin, compressed_celadon_mykoserocin),
        (Gas::GoldenCytoserocin, golden_cytoserocin),
        (Gas::GoldenMykoserocin, golden_mykoserocin),
        (Gas::CompressedGoldenCytoserocin, compressed_golden_cytoserocin),
        (Gas::CompressedGoldenMykoserocin, compressed_golden_mykoserocin),
        (Gas::LimeCytoserocin, lime_cytoserocin),
        (Gas::LimeMykoserocin, lime_mykoserocin),
        (Gas::CompressedLimeCytoserocin, compressed_lime_cytoserocin),
        (Gas::CompressedLimeMykoserocin, compressed_lime_mykoserocin),
        (Gas::MalachiteCytoserocin, malachite_cytoserocin),
        (Gas::MalachiteMykoserocin, malachite_mykoserocin),
        (Gas::CompressedMalachiteCytoserocin, compressed_malachite_cytoserocin),
        (Gas::CompressedMalachiteMykoserocin, compressed_malachite_mykoserocin),
        (Gas::VermillionCytoserocin, vermillion_cytoserocin),
        (Gas::VermillionMykoserocin, vermillion_mykoserocin),
        (Gas::CompressedVermillionCytoserocin, compressed_vermillion_cytoserocin),
        (Gas::CompressedVermillionMykoserocin, compressed_vermillion_mykoserocin),
        (Gas::ViridianCytoserocin, viridian_cytoserocin),
        (Gas::ViridianMykoserocin, viridian_mykoserocin),
        (Gas::CompressedViridianCytoserocin, compressed_viridian_cytoserocin),
        (Gas::CompressedViridianMykoserocin, compressed_viridian_mykoserocin),
        (Gas::FulleriteC28, fullerite_c28),
        (Gas::CompressedFulleriteC28, compressed_fullerite_c28),
        (Gas::FulleriteC32, fullerite_c32),
        (Gas::CompressedFulleriteC32, compressed_fullerite_c32),
        (Gas::FulleriteC320, fullerite_c320),
        (Gas::CompressedFulleriteC320, compressed_fullerite_c320),
        (Gas::FulleriteC50, fullerite_c50),
        (Gas::CompressedFulleriteC50, compressed_fullerite_c50),
        (Gas::FulleriteC540, fullerite_c540),
        (Gas::CompressedFulleriteC540, compressed_fullerite_c540),
        (Gas::FulleriteC60, fullerite_c60),
        (Gas::CompressedFulleriteC60, compressed_fullerite_c60),
        (Gas::FulleriteC70, fullerite_c70),
        (Gas::CompressedFulleriteC70, compressed_fullerite_c70),
        (Gas::FulleriteC72, fullerite_c72),
        (Gas::CompressedFulleriteC72, compressed_fullerite_c72),
        (Gas::FulleriteC84, fullerite_c84),
        (Gas::CompressedFulleriteC84, compressed_fullerite_c84),
    ];

    for (gas, var) in entries.iter() {
        if config.allowed_gas(&gas) {
            if config.gas_price(&gas) >= 0.01 {
                problem = problem
                    .with(constraint!(*var >= 0))
                    .with(constraint!(*var <= config.gas_limit(&gas)))
            } else {
                problem = problem.with(constraint!(*var == 0))
            }
        } else {
            problem = problem.with(constraint!(*var == 0))
        }
    }

    let solution = problem
        .with(constraint!(
            amber_cytoserocin +
            compressed_amber_cytoserocin
            >= config.want_gas(Gas::AmberCytoserocin)
        ).set_name("AmberCytoserocin".into()))
        .with(constraint!(
            amber_mykoserocin +
            compressed_amber_mykoserocin
            >= config.want_gas(Gas::AmberMykoserocin)
        ).set_name("AmberMykoserocin".into()))
        .with(constraint!(
            azure_cytoserocin +
            compressed_azure_cytoserocin
            >= config.want_gas(Gas::AzureCytoserocin)
        ).set_name("AzureCytoserocin".into()))
        .with(constraint!(
            azure_mykoserocin +
            compressed_azure_mykoserocin
            >= config.want_gas(Gas::AzureMykoserocin)
        ).set_name("AzureMykoserocin".into()))
        .with(constraint!(
            celadon_cytoserocin +
            compressed_celadon_cytoserocin
            >= config.want_gas(Gas::CeladonCytoserocin)
        ).set_name("CeladonCytoserocin".into()))
        .with(constraint!(
            celadon_mykoserocin +
            compressed_celadon_mykoserocin
            >= config.want_gas(Gas::CeladonMykoserocin)
        ).set_name("CeladonMykoserocin".into()))
        .with(constraint!(
            golden_cytoserocin +
            compressed_golden_cytoserocin
            >= config.want_gas(Gas::GoldenCytoserocin)
        ).set_name("GoldenCytoserocin".into()))
        .with(constraint!(
            golden_mykoserocin +
            compressed_golden_mykoserocin
            >= config.want_gas(Gas::GoldenMykoserocin)
        ).set_name("GoldenMykoserocin".into()))
        .with(constraint!(
            lime_cytoserocin +
            compressed_lime_cytoserocin
            >= config.want_gas(Gas::LimeCytoserocin)
        ).set_name("LimeCytoserocin".into()))
        .with(constraint!(
            lime_mykoserocin +
            compressed_lime_mykoserocin
            >= config.want_gas(Gas::LimeMykoserocin)
        ).set_name("LimeMykoserocin".into()))
        .with(constraint!(
            malachite_cytoserocin +
            compressed_malachite_cytoserocin
            >= config.want_gas(Gas::MalachiteCytoserocin)
        ).set_name("MalachiteCytoserocin".into()))
        .with(constraint!(
            malachite_mykoserocin +
            compressed_malachite_mykoserocin
            >= config.want_gas(Gas::MalachiteMykoserocin)
        ).set_name("MalachiteMykoserocin".into()))
        .with(constraint!(
            vermillion_cytoserocin +
            compressed_vermillion_cytoserocin
            >= config.want_gas(Gas::VermillionCytoserocin)
        ).set_name("VermillionCytoserocin".into()))
        .with(constraint!(
            vermillion_mykoserocin +
            compressed_vermillion_mykoserocin
            >= config.want_gas(Gas::VermillionMykoserocin)
        ).set_name("VermillionMykoserocin".into()))
        .with(constraint!(
            viridian_cytoserocin +
            compressed_viridian_cytoserocin
            >= config.want_gas(Gas::ViridianCytoserocin)
        ).set_name("ViridianCytoserocin".into()))
        .with(constraint!(
            viridian_mykoserocin +
            compressed_viridian_mykoserocin
            >= config.want_gas(Gas::ViridianMykoserocin)
        ).set_name("ViridianMykoserocin".into()))

        .with(constraint!(
            fullerite_c28 +
            compressed_fullerite_c28
            >= config.want_gas(Gas::FulleriteC28)
        ).set_name("FulleriteC28".into()))
        .with(constraint!(
            fullerite_c32 +
            compressed_fullerite_c32
            >= config.want_gas(Gas::FulleriteC32)
        ).set_name("FulleriteC32".into()))
        .with(constraint!(
            fullerite_c320 +
            compressed_fullerite_c320
            >= config.want_gas(Gas::FulleriteC320)
        ).set_name("FulleriteC320".into()))
        .with(constraint!(
            fullerite_c50 +
            compressed_fullerite_c50
            >= config.want_gas(Gas::FulleriteC50)
        ).set_name("FulleriteC50".into()))
        .with(constraint!(
            fullerite_c540 +
            compressed_fullerite_c540
            >= config.want_gas(Gas::FulleriteC540)
        ).set_name("FulleriteC540".into()))
        .with(constraint!(
            fullerite_c60 +
            compressed_fullerite_c60
            >= config.want_gas(Gas::FulleriteC60)
        ).set_name("FulleriteC60".into()))
        .with(constraint!(
            fullerite_c70 +
            compressed_fullerite_c70
            >= config.want_gas(Gas::FulleriteC70)
        ).set_name("FulleriteC70".into()))
        .with(constraint!(
            fullerite_c72 +
            compressed_fullerite_c72
            >= config.want_gas(Gas::FulleriteC72)
        ).set_name("FulleriteC72".into()))
        .with(constraint!(
            fullerite_c84 +
            compressed_fullerite_c84
            >= config.want_gas(Gas::FulleriteC84)
        ).set_name("FulleriteC84".into()))
        .solve()
        .map_err(|_| Error::NoSolution)?;

    let mut result = HashMap::new();
    for (asteroid, var) in entries.iter() {
        let entry = solution.value(*var);
        if entry > 0f64 {
            result.insert(asteroid.to_type_id(), entry.ceil());
        }
    }

    Ok(result)
}

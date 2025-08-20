use std::collections::HashMap;
use starfoundry_lib_types::TypeId;

use crate::{Asteroid, Mineral, OreReprocessingEfficiency};

pub fn overage(
    efficiency: OreReprocessingEfficiency,
    need:       HashMap<TypeId, i64>,
    want:       HashMap<Mineral, f64>,
) -> HashMap<Mineral, f64> {
    let mut total = HashMap::new();

    need
        .into_iter()
        .for_each(|(type_id, quantity)| {
            Asteroid::from_type_id(*type_id)
                .minerals()
                .into_iter()
                .for_each(|(mineral, mineral_quantity)| {
                    // make sure that there is always at least 100
                    let quantity = ((mineral_quantity * quantity as f64) / 100f64).floor();

                    let amount = (
                        (
                            quantity
                        ) * efficiency.efficiency()
                    ).floor();

                    total
                        .entry(mineral)
                        .and_modify(|x: &mut f64| *x += amount)
                        .or_insert(amount);
                });
        });

    want
        .into_iter()
        .for_each(|(mineral, quantity)| {
            total
                .entry(mineral)
                .and_modify(|x: &mut f64| *x -= quantity);
        });

    total
}

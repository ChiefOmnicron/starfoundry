use std::collections::HashMap;
use starfoundry_libs_types::TypeId;

use crate::{Asteroid, Mineral, OreReprocessingEfficiency};

pub fn overage(
    //items:      HashMap<TypeId, GroupId>,
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
                    /*let type_id = dbg!(mineral.clone().to_type_id());
                    let quantity = match **items.get(&type_id).unwrap() {
                        465 => mineral_quantity * quantity as f64,
                        _   => ((mineral_quantity * quantity as f64) / 100f64).floor(),
                    };*/

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

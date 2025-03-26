#[macro_export]
macro_rules! sort_by_market_group {
    ($name:ident, $typ_in:ty, $typ_out:ident) => {
        pub fn $name(entries: Vec<$typ_in>) -> Vec<$typ_out> {
            let mut market_groups   = Vec::new();
            let mut grouped_entries = std::collections::HashMap::new();

            let mut insert_into_map = |id: i32, entry: $typ_in| {
                grouped_entries
                    .entry(id)
                    .and_modify(|x: &mut Vec<$typ_in>| x.push(entry.clone()))
                    .or_insert(vec![entry]);
            };

            // First go through all entries, and sort them into the map
            for entry in entries.into_iter() {
                match *entry.category_id {
                    7i32 => {
                        insert_into_map(7, entry);
                        continue;
                    },
                    8i32 => {
                        insert_into_map(8, entry);
                        continue;
                    },
                    25i32 => {
                        insert_into_map(25, entry);
                        continue;
                    },
                    _  => {}
                }

                match *entry.group_id {
                    18i32 => {
                        insert_into_map(18, entry);
                        continue;
                    },
                    303i32 => {
                        insert_into_map(303, entry);
                        continue;
                    },
                    334i32 => {
                        insert_into_map(334, entry);
                        continue;
                    },
                    423i32 => {
                        insert_into_map(423, entry);
                        continue;
                    },
                    428i32 => {
                        insert_into_map(428, entry);
                        continue;
                    },
                    429i32 => {
                        insert_into_map(429, entry);
                        continue;
                    },
                    427i32 => {
                        insert_into_map(427, entry);
                        continue;
                    },
                    526i32 => {
                        insert_into_map(526, entry);
                        continue;
                    },
                    711i32 => {
                        insert_into_map(711, entry);
                        continue;
                    },
                    712i32 => {
                        insert_into_map(712, entry);
                        continue;
                    },
                    754i32 => {
                        insert_into_map(754, entry);
                        continue;
                    },
                    974i32 => {
                        insert_into_map(974, entry);
                        continue;
                    },
                    1136i32 => {
                        insert_into_map(1136, entry);
                        continue;
                    },
                    1042i32 => {
                        insert_into_map(1042, entry);
                        continue;
                    },
                    1034i32 => {
                        insert_into_map(1034, entry);
                        continue;
                    },
                    1040i32 => {
                        insert_into_map(1040, entry);
                        continue;
                    },
                    1041i32 => {
                        insert_into_map(1041, entry);
                        continue;
                    },
                    1996i32 => {
                        insert_into_map(1996, entry);
                        continue;
                    },
                    4168i32 => {
                        insert_into_map(4168, entry);
                        continue;
                    },
                    _  => {
                        insert_into_map(0, entry);
                    }
                }
            }

            // Secondly give the groups a name, and sort the entries
            for (header, id) in vec![
                ("COMPRESSED_MINERALS",       25),
                ("MINERALS",                  18),
                ("MOON_MATERIALS",           427),
                ("COMPRESSED_GAS",          4168),
                ("GAS",                      711),
                ("FUEL_BLOCKS",             1136),
                ("INTERMEDIATE_COMPOSITE",   428),
                ("COMPOSITE",                429),
                ("HYBRID_POLYMERS",          974),
                ("PI_TIER_1",               1042),
                ("PI_TIER_2",               1034),
                ("PI_TIER_3",               1040),
                ("PI_TIER_4",               1041),
                ("COMMODITIES",              526),
                ("CONSTRUCTION_COMPONENTS",  334),
                ("SALVAGE",                  754),
                ("MODULES",                    7),
                ("CHARGES",                    8),
                ("BOOSTER",                  303),
                ("ICE",                      423),
                ("BIOCHEMICAL",              712),
                ("ABYSSAL_MATERIALS",       1996),
                ("UNGROUPED",                  0),
            ] {
                if let Some(x) = grouped_entries.get_mut(&id) {
                    x.sort_by_key(|x| x.item_name.clone());
                    market_groups.push(
                        $typ_out {
                            header:  header.into(),
                            entries: x.clone(),
                        }
                    );
                }
            }

            market_groups
        }
    }
}

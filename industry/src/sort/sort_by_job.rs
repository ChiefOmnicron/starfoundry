#[macro_export]
macro_rules! sort_by_job {
    ($name:ident, $typ_in:ty, $typ_out:ident) => {
        pub fn $name(entries: Vec<$typ_in>) -> Vec<$typ_out> {
            let mut job_lists       = Vec::new();
            let mut grouped_entries = std::collections::HashMap::new();

            let mut insert_into_map = |id: i32, entry: $typ_in| {
                grouped_entries
                    .entry(id)
                    .and_modify(|x: &mut Vec<$typ_in>| x.push(entry.clone()))
                    .or_insert(vec![entry]);
            };

            // First go through all entries, and sort them into the map
            for entry in entries.into_iter() {
                match *entry.item.category.category_id {
                    6i32 => {
                        insert_into_map(6, entry);
                        continue;
                    },
                    8i32 => {
                        insert_into_map(8, entry);
                        continue;
                    },
                    65i32 => {
                        insert_into_map(65, entry);
                        continue;
                    },
                    66i32 => {
                        insert_into_map(66, entry);
                        continue;
                    },
                    _  => {}
                }

                match *entry.item.group.group_id {
                    332i32 => {
                        insert_into_map(332, entry);
                        continue;
                    },
                    334i32 => {
                        insert_into_map(334, entry);
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
                    536i32 => {
                        insert_into_map(536, entry);
                        continue;
                    },
                    873i32 => {
                        insert_into_map(873, entry);
                        continue;
                    },
                    913i32 => {
                        insert_into_map(913, entry);
                        continue;
                    },
                    974i32 => {
                        insert_into_map(974, entry);
                        continue;
                    },
                    4096i32 => {
                        insert_into_map(4096, entry);
                        continue;
                    },
                    // Rigs
                    773i32 => {
                        insert_into_map(1308, entry);
                        continue;
                    },
                    774i32 => {
                        insert_into_map(1308, entry);
                        continue;
                    },
                    775i32 => {
                        insert_into_map(1308, entry);
                        continue;
                    },
                    776i32 => {
                        insert_into_map(1308, entry);
                        continue;
                    },
                    777i32 => {
                        insert_into_map(1308, entry);
                        continue;
                    },
                    778i32 => {
                        insert_into_map(1308, entry);
                        continue;
                    },
                    779i32 => {
                        insert_into_map(1308, entry);
                        continue;
                    },
                    781i32 => {
                        insert_into_map(1308, entry);
                        continue;
                    },
                    782i32 => {
                        insert_into_map(1308, entry);
                        continue;
                    },
                    786i32 => {
                        insert_into_map(1308, entry);
                        continue;
                    },
                    904i32 => {
                        insert_into_map(1308, entry);
                        continue;
                    },
                    1308i32 => {
                        insert_into_map(1308, entry);
                        continue;
                    },
                    1232i32 => {
                        insert_into_map(1308, entry);
                        continue;
                    },
                    1233i32 => {
                        insert_into_map(1308, entry);
                        continue;
                    },
                    1234i32 => {
                        insert_into_map(1308, entry);
                        continue;
                    },
                    _  => {}
                }

                if let Some(meta_group) = entry.item.meta_group {
                    match *meta_group {
                        1i32 => {
                            insert_into_map(1, entry);
                            continue;
                        },
                        2i32 => {
                            insert_into_map(2, entry);
                            continue;
                        },
                        8i32 => {
                            insert_into_map(8, entry);
                            continue;
                        },
                        _  => {}
                    }
                }

                insert_into_map(0, entry.clone());
            }

            for (header, id) in vec![
                ("INTERMEDIATE_REACTIONS",                    428),
                ("COMPOSITE_REACTIONS",                       429),
                ("BIOCHEM_REACTIONS",                        4096),
                ("HYBRID_REACTIONS",                          974),
                ("CONSTRUCTION_COMPONENTS",                   334),
                ("ADVANCED_CAPITAL_CONSTRUCTION_COMPONENTS",  913),
                ("CAPITAL_CONSTRUCTION_COMPONENTS",           873),
                ("STRUCTURE_COMPONENTS",                      536),
                ("TOOLS",                                     332),
                ("T1_STUFF",                                    1),
                ("T2_STUFF",                                    2),
                ("RIGS",                                     1308),
                ("STRUCTURE_RIGS",                             66),
                ("CHARGES",                                     8),
                ("SHIPS",                                       6),
                ("STRUCTURE",                                  65),
                ("UNKNOWN",                                     0),
            ] {
                if let Some(entries) = grouped_entries.get_mut(&id) {
                    entries.sort_by_key(|x| x.item.name.clone());
                    let mut entries = entries
                        .chunk_by(|a, b| a.item.name == b.item.name)
                        .map(|x| x.into())
                        .collect::<Vec<Vec<$typ_in>>>();
                    let entries = entries
                        .iter_mut()
                        .map(|x| {
                            x.sort_by_key(|y| (y.runs.clone(), y.id));
                            x.reverse();
                            x.clone()
                        })
                        .collect::<Vec<_>>()
                        .into_iter()
                        .flatten()
                        .collect::<Vec<_>>();
                    job_lists.push(
                        $typ_out {
                            header:  header.into(),
                            entries: entries.clone(),
                        }
                    );
                }
            }

            job_lists
        }
    }
}

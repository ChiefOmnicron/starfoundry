use once_cell::sync::{Lazy, OnceCell};
use regex::Regex;
use sqlx::PgPool;
use starfoundry_lib_eve_gateway::{Item, ParseResult, ParsedItem};
use std::collections::HashMap;

use crate::item::error::{ItemError, Result};
use crate::item::services::{fetch_category, fetch_group};

pub static ITEM_CACHE: OnceCell<HashMap<String, Item>> = OnceCell::new();

/// Parses the given content and tries to detect item names and their quantity
/// 
pub fn parse(
    cache:   &HashMap<String, Item>,
    content: &str,
) -> ParseResult {
    static FIT_HEADER: Lazy<Regex> = Lazy::new(|| Regex::new(r"\[([a-zA-Z ]*)(,.*)?\]").unwrap());

    let mut items   = Vec::new();
    let mut invalid = Vec::new();

    let mut is_fit = false;

    for line in content.lines() {
        let mut entry = None;
        tracing::info!("Item parser {}", &line);

        let line = sanitize_name(line.to_lowercase())
            .trim()
            .replace("\t", " ");

        // TODO: refactor
        if line.to_lowercase().contains("[empty high slot]") ||
            line.to_lowercase().contains("[empty med slot]") ||
            line.to_lowercase().contains("[empty low slot]") {
            continue;
        }

        if FIT_HEADER.is_match(&line) {
            is_fit = true;
            let ship = if let Some(x) = FIT_HEADER.captures(&line) {
                x.get(1).map_or("", |m| m.as_str())
            } else {
                continue;
            };

            if let Some(item) = cache.get(&ship.to_lowercase()) {
                entry = Some(
                    ParsedItem {
                        item_name:           item.name.clone(),
                        type_id:             item.type_id,
                        quantity:            1,
                        material_efficiency: None,

                        raw:                 item.clone(),
                    }
                );
            }
        }

        let line = if is_fit && line.contains(", ") {
            let (without_comma, _) = line.split_once(", ").unwrap_or_default();
            without_comma.to_string()
        } else {
            line
        };

        let mut splitted_line = line.split_whitespace().collect::<Vec<_>>();
        let mut leftovers = Vec::new();

        if line.is_empty() {
            continue;
        }

        while !splitted_line.is_empty() {
            let line = splitted_line.join(" ").to_lowercase();

            if let Some(item) = cache.get(&line) {
                entry = Some(
                    ParsedItem {
                        item_name:           item.name.clone(),
                        type_id:             item.type_id,
                        quantity:            1,
                        material_efficiency: None,

                        raw:                 item.clone(),
                    }
                );
                break;
            } else {
                leftovers.push(splitted_line.pop());
            }
        }

        leftovers.reverse();
        if let Some(mut x) = entry {
            // quantity of items
            if let Some(Some(quantity)) = leftovers.first() {
                let quantity = *quantity;

                if quantity.starts_with('x') &&
                   quantity.chars().skip(1).all(|x| x.is_numeric()) {

                    x.quantity = quantity
                        .chars()
                        .skip(1)
                        .collect::<String>()
                        .parse()
                        .unwrap_or_default();
                } else {
                    // if the number is a float, this will be messed up big time
                    let quantity = quantity
                        .replace(",", "")
                        .replace(".", "");
                    x.quantity = quantity.parse::<i64>().unwrap_or(1);
                }
            }

            // Material Efficiency
            if let Some(Some(material_efficiency)) = leftovers.get(1) {
                if material_efficiency.chars().all(|x| x.is_numeric()) {
                    let material_efficiency = *material_efficiency;
                    x.material_efficiency = Some(material_efficiency.parse::<usize>().unwrap_or(0));
                }
            }

            items.push(x);
        } else {
            invalid.push(line);
        }
    }

    ParseResult {
        invalid,
        items,
    }
}

pub async fn load_items<'a>(
    pool: &PgPool,
) -> Result<&'a HashMap<String, Item>> {
    if let Some(x) = ITEM_CACHE.get() {
        return Ok(x);
    }

    let all_items_db = sqlx::query!("
            SELECT
                name,
                type_id,
                volume,
                category_id,
                group_id,
                meta_group_id,
                repackaged
            FROM item
        ")
        .fetch_all(pool)
        .await
        .map_err(ItemError::List)?;

    let mut all_items = HashMap::new();
    for item in all_items_db {
        let category = if let Ok(Some(x)) = fetch_category(
            pool,
            item.category_id.into(),
        ).await {
            x
        } else {
            continue;
        };
        let group = if let Ok(Some(x)) = fetch_group(
            pool,
            item.group_id.into(),
        ).await {
            x
        } else {
            continue;
        };

        let item = Item {
            name:       sanitize_name(item.name.clone()),
            volume:     item.volume,
            repackaged: item.repackaged,
            type_id:    item.type_id.into(),

            category:   category,
            group:      group,
            meta_group: item.meta_group_id.map(|x| x.into()),
        };

        let name = sanitize_name(item.name.to_lowercase());
        all_items.insert(name, item);
    }

    let data = ITEM_CACHE.get_or_init(|| all_items);
    Ok(data)
}

// some names have too many spaces
fn sanitize_name(
    name: String,
) -> String {
    name
        .replace("  blueprint", " blueprint")
        .replace("  Blueprint", " Blueprint")
        .replace("  hauler", " hauler")
        .replace("  Hauler", " Hauler")
        .replace("  skin", " skin")
        .replace("  SKIN", " SKIN")
        .replace(" ", "") // \u{a0}
        .trim_start()
        .trim_end()
        .to_string()
}

#[cfg(test)]
mod item_parser_tests {
    use super::parse;

    use std::collections::HashMap;
    use starfoundry_lib_eve_gateway::Item;
    use sqlx::PgPool;

    async fn load_items<'a>(
        pool: &PgPool,
    ) -> &'a HashMap<String, Item> {
        dotenvy::dotenv().ok();
        super::load_items(&pool).await.unwrap()
    }

    #[sqlx::test(
        fixtures(
            path = "../fixtures",
            scripts("category", "groups", "items_min")
        ),
    )]
    async fn one_line_no_tabs(
        pool: PgPool,
    ) {
        let all_items = load_items(&pool).await;
        let content = "Revelation Navy Issue 1".into();
        let result = parse(&all_items, content);

        assert_eq!(result.items.len(), 1);
        assert_eq!(result.items[0].item_name, "Revelation Navy Issue".to_string());
        assert_eq!(result.items[0].quantity, 1);
    }

    #[sqlx::test(
        fixtures(
            path = "../fixtures",
            scripts("category", "groups", "items_min")
        ),
    )]
    async fn one_line_tabs(
        pool: PgPool,
    ) {
        let all_items = load_items(&pool).await;
        let content = "Revelation Navy Issue\t1".into();
        let result = parse(&all_items, content);

        assert_eq!(result.items.len(), 1);
        assert_eq!(result.items[0].item_name, "Revelation Navy Issue".to_string());
        assert_eq!(result.items[0].quantity, 1);
    }

        #[sqlx::test(
        fixtures(
            path = "../fixtures",
            scripts("category", "groups", "items_min")
        ),
    )]
    async fn one_line_with_numbers_in_name(
        pool: PgPool,
    ) {
        let all_items = load_items(&pool).await;
        let content = "1200mm Artillery Cannon II\t10".into();
        let result = parse(&all_items, content);

        assert_eq!(result.items.len(), 1);
        assert_eq!(result.items[0].item_name, "1200mm Artillery Cannon II".to_string());
        assert_eq!(result.items[0].quantity, 10);
    }

        #[sqlx::test(
        fixtures(
            path = "../fixtures",
            scripts("category", "groups", "items_min")
        ),
    )]
    async fn multi_line(
        pool: PgPool,
    ) {
        let all_items = load_items(&pool).await;
        let content = "Revelation Navy Issue\t2
Phoenix Navy Issue\t1
Moros Navy Issue\t1
Naglfar Fleet Issue\t1".into();
        let result = parse(&all_items, content);

        assert_eq!(result.items.len(), 4);
        assert_eq!(result.items[0].item_name, "Revelation Navy Issue".to_string());
        assert_eq!(result.items[1].item_name, "Phoenix Navy Issue".to_string());
        assert_eq!(result.items[2].item_name, "Moros Navy Issue".to_string());
        assert_eq!(result.items[3].item_name, "Naglfar Fleet Issue".to_string());
        assert_eq!(result.items[0].quantity, 2);
        assert_eq!(result.items[1].quantity, 1);
        assert_eq!(result.items[2].quantity, 1);
        assert_eq!(result.items[3].quantity, 1);
    }

        #[sqlx::test(
        fixtures(
            path = "../fixtures",
            scripts("category", "groups", "items_min")
        ),
    )]
    async fn one_line_manufacturing_default_0(
        pool: PgPool,
    ) {
        let all_items = load_items(&pool).await;
        let content = "Revelation Navy Issue 1".into();
        let result = parse(&all_items, content);

        assert_eq!(result.items.len(), 1);
        assert_eq!(result.items[0].item_name, "Revelation Navy Issue".to_string());
        assert_eq!(result.items[0].quantity, 1);
        assert_eq!(result.items[0].material_efficiency, None);
    }

        #[sqlx::test(
        fixtures(
            path = "../fixtures",
            scripts("category", "groups", "items_min")
        ),
    )]
    async fn one_line_manufacturing_me_10(
        pool: PgPool,
    ) {
        let all_items = load_items(&pool).await;
        let content = "Revelation 1 10".into();
        let result = parse(&all_items, content);

        assert_eq!(result.items.len(), 1);
        assert_eq!(result.items[0].item_name, "Revelation".to_string());
        assert_eq!(result.items[0].quantity, 1);
        assert_eq!(result.items[0].material_efficiency, Some(10));
    }

    #[sqlx::test(
        fixtures(
            path = "../fixtures",
            scripts("category", "groups", "items_min")
        ),
    )]
    async fn mutli_line_multi_builds(
        pool: PgPool,
    ) {
        let all_items = load_items(&pool).await;
        let content = "Revelation Navy Issue\t2
1200mm Artillery Cannon II\t100 10
25000mm Steel Plates II 2 2".into();
        let result = parse(&all_items, content);

        assert_eq!(result.items.len(), 3);
        assert_eq!(result.items[0].item_name, "Revelation Navy Issue".to_string());
        assert_eq!(result.items[1].item_name, "1200mm Artillery Cannon II".to_string());
        assert_eq!(result.items[2].item_name, "25000mm Steel Plates II".to_string());
        assert_eq!(result.items[0].quantity, 2);
        assert_eq!(result.items[1].quantity, 100);
        assert_eq!(result.items[2].quantity, 2);
        assert_eq!(result.items[0].material_efficiency, None);
        assert_eq!(result.items[1].material_efficiency, Some(10));
        assert_eq!(result.items[2].material_efficiency, Some(2));
    }

    #[sqlx::test(
        fixtures(
            path = "../fixtures",
            scripts("category", "groups", "items_min")
        ),
    )]
    async fn blueprint_builds_with_x_as_quantity(
        pool: PgPool,
    ) {
        let all_items = load_items(&pool).await;
        let content = "Revelation\tx2\t10".into();
        let result = parse(&all_items, content);

        assert_eq!(result.items.len(), 1);
        assert_eq!(result.items[0].item_name, "Revelation".to_string());
        assert_eq!(result.items[0].quantity, 2);
        assert_eq!(result.items[0].material_efficiency, Some(10));
    }

    #[sqlx::test(
        fixtures(
            path = "../fixtures",
            scripts("category", "groups", "items_min")
        ),
    )]
    async fn one_line_builds_no_runs(
        pool: PgPool,
    ) {
        let all_items = load_items(&pool).await;
        let content = "Revelation Navy Issue".into();
        let result = parse(&all_items, content);

        assert_eq!(result.items.len(), 1);
        assert_eq!(result.items[0].item_name, "Revelation Navy Issue".to_string());
        assert_eq!(result.items[0].quantity, 1);
        assert_eq!(result.items[0].material_efficiency, None);
    }

    #[sqlx::test(
        fixtures(
            path = "../fixtures",
            scripts("category", "groups", "items_min")
        ),
    )]
    async fn line_with_x_as_quantity(
        pool: PgPool,
    ) {
        let all_items = load_items(&pool).await;
        let content = "Helium Isotopes x400000".into();
        let result = parse(&all_items, content);

        assert_eq!(result.items.len(), 1);
        assert_eq!(result.items[0].item_name, "Helium Isotopes".to_string());
        assert_eq!(result.items[0].quantity, 400_000);
    }

    #[sqlx::test(
        fixtures(
            path = "../fixtures",
            scripts("category", "groups", "items_min")
        ),
    )]
    async fn line_with_additional_number(
        pool: PgPool,
    ) {
        let all_items = load_items(&pool).await;
        let content = "Cap Booster 3200 5 10".into();
        let result = parse(&all_items, content);

        assert_eq!(result.items.len(), 1);
        assert_eq!(result.items[0].item_name, "Cap Booster 3200".to_string());
        assert_eq!(result.items[0].quantity, 5);
        assert_eq!(result.items[0].material_efficiency, Some(10));
    }

    #[sqlx::test(
        fixtures(
            path = "../fixtures",
            scripts("category", "groups", "items_min")
        ),
    )]
    async fn multi_line_with_empty_last_line(
        pool: PgPool,
    ) {
        let all_items = load_items(&pool).await;
        let content = "Cap Booster 3200 5 10
".into();
        let result = parse(&all_items, content);

        assert_eq!(result.items.len(), 1);
        assert_eq!(result.items[0].item_name, "Cap Booster 3200".to_string());
        assert_eq!(result.items[0].quantity, 5);
        assert_eq!(result.items[0].material_efficiency, Some(10));
    }

    #[sqlx::test(
        fixtures(
            path = "../fixtures",
            scripts("category", "groups", "items_min")
        ),
    )]
    async fn single_line_with_dash(
        pool: PgPool,
    ) {
        let all_items = load_items(&pool).await;
        let content = "Fullerite-C32 100".into();
        let result = parse(&all_items, content);

        assert_eq!(result.items.len(), 1);
        assert_eq!(result.items[0].item_name, "Fullerite-C32".to_string());
        assert_eq!(result.items[0].quantity, 100);
    }

    #[sqlx::test(
        fixtures(
            path = "../fixtures",
            scripts("category", "groups", "items_min")
        ),
    )]
    async fn fit_header_no_description(
        pool: PgPool,
    ) {
        let all_items = load_items(&pool).await;
        let content = "[Nightmare]".into();
        let result = parse(&all_items, content);

        assert_eq!(result.items.len(), 1);
        assert_eq!(result.items[0].item_name, "Nightmare".to_string());
        assert_eq!(result.items[0].quantity, 1);
    }

    #[sqlx::test(
        fixtures(
            path = "../fixtures",
            scripts("category", "groups", "items_min")
        ),
    )]
    async fn fit_header_description(
        pool: PgPool,
    ) {
        let all_items = load_items(&pool).await;
        let content = "[Nightmare, Test]".into();
        let result = parse(&all_items, content);

        assert_eq!(result.items.len(), 1);
        assert_eq!(result.items[0].item_name, "Nightmare".to_string());
        assert_eq!(result.items[0].quantity, 1);
    }

    #[sqlx::test(
        fixtures(
            path = "../fixtures",
            scripts("category", "groups", "items_min")
        ),
    )]
    async fn item_quantity_stuff(
        pool: PgPool,
    ) {
        let all_items = load_items(&pool).await;
        let content = "Legion 4 Strategic Cruiser".into();
        let result = parse(&all_items, content);

        assert_eq!(result.items.len(), 1);
        assert_eq!(result.items[0].item_name, "Legion".to_string());
        assert_eq!(result.items[0].quantity, 4);
    }

    #[sqlx::test(
        fixtures(
            path = "../fixtures",
            scripts("category", "groups", "items_min")
        ),
    )]
    async fn item_quantity_stuff_with_comma(
        pool: PgPool,
    ) {
        let all_items = load_items(&pool).await;
        let content = "Megacyte    1,040    Mineral".into();
        let result = parse(&all_items, content);

        assert_eq!(result.items.len(), 1);
        assert_eq!(result.items[0].item_name, "Megacyte".to_string());
        assert_eq!(result.items[0].quantity, 1040);
    }

        #[sqlx::test(
        fixtures(
            path = "../fixtures",
            scripts("category", "groups", "items_min")
        ),
    )]
    async fn ignore_total(
        pool: PgPool,
    ) {
        let all_items = load_items(&pool).await;
        let content = "Total:			0".into();
        let result = parse(&all_items, content);

        assert_eq!(result.items.len(), 0);
    }

    #[sqlx::test(
        fixtures(
            path = "../fixtures",
            scripts("category", "groups", "items_min")
        ),
    )]
    async fn ignore_empty_market(
        pool: PgPool,
    ) {
        let all_items = load_items(&pool).await;
        let content = "Capital Shield Emitter	130	-	-".into();
        let result = parse(&all_items, content);

        assert_eq!(result.items.len(), 1);
        assert_eq!(result.items[0].item_name, "Capital Shield Emitter".to_string());
        assert_eq!(result.items[0].quantity, 130);
    }

    #[sqlx::test(
        fixtures(
            path = "../fixtures",
            scripts("category", "groups", "items_min")
        ),
    )]
    async fn fit_with_loaded_module(
        pool: PgPool,
    ) {
        let all_items = load_items(&pool).await;
        let content = "[Karura, Killmail 126026825]
Triple Neutron Blaster Cannon II, Void XL
Triple Neutron Blaster Cannon II, Void XL
Triple Neutron Blaster Cannon II, Void XL".into();
        let result = parse(&all_items, content);

        assert_eq!(result.items.len(), 4);
        assert_eq!(result.items[0].item_name, "Karura".to_string());
        assert_eq!(result.items[0].quantity, 1);
        assert_eq!(result.items[1].item_name, "Triple Neutron Blaster Cannon II".to_string());
        assert_eq!(result.items[1].quantity, 1);
        assert_eq!(result.items[2].item_name, "Triple Neutron Blaster Cannon II".to_string());
        assert_eq!(result.items[2].quantity, 1);
        assert_eq!(result.items[3].item_name, "Triple Neutron Blaster Cannon II".to_string());
        assert_eq!(result.items[3].quantity, 1);
    }

    #[sqlx::test(
        fixtures(
            path = "../fixtures",
            scripts("category", "groups", "items_min")
        ),
    )]
    async fn survey_scanner_1(
        pool: PgPool,
    ) {
        let all_items = load_items(&pool).await;
        let content = "Monazite\t8\u{a0}476\t84\u{a0}760 m3\t92 km".into();
        let result = parse(&all_items, content);

        assert_eq!(result.items.len(), 1);
        assert_eq!(result.items[0].item_name, "Monazite".to_string());
        assert_eq!(result.items[0].quantity, 8476);
    }

    #[sqlx::test(
        fixtures(
            path = "../fixtures",
            scripts("category", "groups", "items_min")
        ),
    )]
    async fn survey_scanner_2(
        pool: PgPool,
    ) {
        let all_items = load_items(&pool).await;
        let content = "Shining Monazite\t8\u{a0}476\t84\u{a0}760 m3\t92 km".into();
        let result = parse(&all_items, content);

        assert_eq!(result.items.len(), 1);
        assert_eq!(result.items[0].item_name, "Shining Monazite".to_string());
        assert_eq!(result.items[0].quantity, 8476);
    }

    #[sqlx::test(
        fixtures(
            path = "../fixtures",
            scripts("category", "groups", "items_full")
        ),
    )]
    async fn all_items(
        pool: PgPool,
    ) {
        let all_items = load_items(&pool).await;

        for (name_lower, item) in all_items.iter() {
            // Splitted with <space>
            let result = parse(
                &all_items,
                &format!("{name_lower} 1"),
            );
            dbg!(name_lower, &result);
            assert_eq!(result.items.len(), 1);
            assert_eq!(result.items[0].item_name, item.name.clone());
            assert_eq!(result.items[0].type_id, item.type_id);
            assert_eq!(result.items[0].quantity, 1);
            assert_eq!(result.items[0].material_efficiency, None);

            // Splitted with <tab>
            let result = parse(
                &all_items,
                &format!("{name_lower}\t1"),
            );
            assert_eq!(result.items.len(), 1);
            assert_eq!(result.items[0].item_name, item.name.clone());
            assert_eq!(result.items[0].type_id, item.type_id);
            assert_eq!(result.items[0].quantity, 1);
            assert_eq!(result.items[0].material_efficiency, None);

            // Higher quantity
            let result = parse(
                &all_items,
                &format!("{name_lower}\t100"),
            );
            assert_eq!(result.items.len(), 1);
            assert_eq!(result.items[0].item_name, item.name.clone());
            assert_eq!(result.items[0].type_id, item.type_id);
            assert_eq!(result.items[0].quantity, 100);
            assert_eq!(result.items[0].material_efficiency, None);

            // Blueprint Manufacturing Efficiency
            let result = parse(
                &all_items,
                &format!("{name_lower}\t1\t1"),
            );
            assert_eq!(result.items.len(), 1);
            assert_eq!(result.items[0].item_name, item.name.clone());
            assert_eq!(result.items[0].type_id, item.type_id);
            assert_eq!(result.items[0].quantity, 1);
            assert_eq!(result.items[0].material_efficiency, Some(1));

            // Lowercase
            let result = parse(
                &all_items,
                &format!("{}\t1", name_lower.to_lowercase()),
            );
            assert_eq!(result.items.len(), 1);
            assert_eq!(result.items[0].item_name, item.name.clone());
            assert_eq!(result.items[0].type_id, item.type_id);
            assert_eq!(result.items[0].quantity, 1);
            assert_eq!(result.items[0].material_efficiency, None);

            // Uppercase
            let result = parse(
                &all_items,
                &format!("{}\t1", name_lower.to_uppercase()),
            );
            assert_eq!(result.items.len(), 1);
            assert_eq!(result.items[0].item_name, item.name.clone());
            assert_eq!(result.items[0].type_id, item.type_id);
            assert_eq!(result.items[0].quantity, 1);
            assert_eq!(result.items[0].material_efficiency, None);

            // Item Stuff
            let result = parse(
                &all_items,
                &format!("{} Battlecruiser", name_lower.to_uppercase()),
            );
            assert_eq!(result.items.len(), 1);
            assert_eq!(result.items[0].item_name, item.name.clone());
            assert_eq!(result.items[0].type_id, item.type_id);
            assert_eq!(result.items[0].quantity, 1);
            assert_eq!(result.items[0].material_efficiency, None);

            // Item Quantity Stuff
            let result = parse(
                &all_items,
                &format!("{} 1 Battlecruiser", name_lower.to_uppercase()),
            );
            assert_eq!(result.items.len(), 1);
            assert_eq!(result.items[0].item_name, item.name.clone());
            assert_eq!(result.items[0].type_id, item.type_id);
            assert_eq!(result.items[0].quantity, 1);
            assert_eq!(result.items[0].material_efficiency, None);

            // Item Quantity Stuff with comma
            let result = parse(
                &all_items,
                &format!("{} 1,040 Battlecruiser", name_lower.to_uppercase()),
            );
            assert_eq!(result.items.len(), 1);
            assert_eq!(result.items[0].item_name, item.name.clone());
            assert_eq!(result.items[0].type_id, item.type_id);
            assert_eq!(result.items[0].quantity, 1040);
            assert_eq!(result.items[0].material_efficiency, None);

            // Item Quantity Stuff with more spaces
            let result = parse(
                &all_items,
                &format!("{}    1,040    Mineral", name_lower.to_uppercase()),
            );
            assert_eq!(result.items.len(), 1);
            assert_eq!(result.items[0].item_name, item.name.clone());
            assert_eq!(result.items[0].type_id, item.type_id);
            assert_eq!(result.items[0].quantity, 1040);
            assert_eq!(result.items[0].material_efficiency, None);
        }
    }
}

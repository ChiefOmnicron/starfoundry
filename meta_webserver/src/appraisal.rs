use num_format::{Locale, ToFormattedString};
use serde_json::json;
use sqlx::PgPool;
use starfoundry_libs_appraisal::internal::fetch;

use crate::WithTemplate;

const TEMPLATE: &str = r#"<!DOCTYPE html>
<html>
    <head>
        <title>Appraisal {{code}}: Buy {{ buy }} / Sell {{ sell }}</title>

        <meta name="description" content="{{ description }}">
    </head>
</html>"#;

pub fn template() -> String {
    TEMPLATE.into()
}

pub async fn appraisal(
    pool: PgPool,
    code: String,
) -> WithTemplate<serde_json::Value> {
    let data = fetch(
            &pool,
            code.clone(),
        )
        .await
        .unwrap()
        .unwrap();
    let items = data
        .items
        .iter()
        .map(|x| {
            format!("{} - {}", x.meta.name, x.quantity.to_formatted_string(&Locale::en))
        })
        .collect::<Vec<_>>()
        .join("\n");

    let buy= data
        .items
        .iter()
        .map(|x| x.buy.max)
        .sum::<f64>()
        .round();
    let buy = if buy >= 1_000_000_000f64 {
        format!("{:.2} Billion", buy / 1_000_000_000f64)
    } else if buy >= 1_000_000f64 {
        format!("{:.2} Million", buy / 1_000_000f64)
    } else if buy >= 1_000_000f64 {
        format!("{:.2} Thousand", buy / 1_000f64)
    } else {
        format!("{:.2}", buy)
    };

    let sell = data
        .items
        .iter()
        .map(|x| x.sell.min)
        .sum::<f64>()
        .round();
    let sell = if sell >= 1_000_000_000f64 {
        format!("{:.2} Billion", sell / 1_000_000_000f64)
    } else if sell >= 1_000_000f64 {
        format!("{:.2} Million", sell / 1_000_000f64)
    } else if sell >= 1_000_000f64 {
        format!("{:.2} Thousand", sell / 1_000f64)
    } else {
        format!("{:.2}", sell)
    };

    WithTemplate {
        name: "appraisal.html",
        value: json!({
            "code": code,
            "description" : items,
            "buy": buy,
            "sell": sell,
        }),
    }
}

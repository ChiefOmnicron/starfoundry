use num_format::{Locale, ToFormattedString};
use sqlx::PgPool;
use sqlx::postgres::PgPoolOptions;
use starfoundry_libs_compression::*;
use std::collections::HashMap;
use term_table::row::Row;
use term_table::table_cell::{Alignment, TableCell};
use term_table::TableStyle;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenvy::dotenv().ok();

    let pg_addr = std::env::var("DATABASE_URL").expect("Expected that a DATABASE_URL ENV is set");
    let pool = PgPoolOptions::new()
        .connect(&pg_addr)
        .await?;

    let mut config = Config::default();
    //config.want_mineral.insert(Mineral::Tritanium, 5_000_000f64);
    //config.want_mineral.insert(Mineral::Mexallon, 5_000_000f64);
    //config.want_mineral.insert(Mineral::Pyerite, 5_000_000f64);
    //config.want_mineral.insert(Mineral::Isogen, 5_000_000f64);
    //config.want_mineral.insert(Mineral::Nocxium, 5_000_000f64);
    //config.want_mineral.insert(Mineral::Zydrine, 5_000_000f64);
    //config.want_mineral.insert(Mineral::Megacyte, 5_000_000f64);
    //config.want_mineral.insert(Mineral::Morphite, 1_000_000f64);
    //config.want_mineral.insert(Mineral::AtmosphericGases, 500_000f64);
    //config.want_mineral.insert(Mineral::EvaporiteDeposits, 500_000f64);
    //config.want_mineral.insert(Mineral::Hydrocarbons, 500_000f64);
    //config.want_mineral.insert(Mineral::Silicates, 500_000f64);
    //config.want_mineral.insert(Mineral::Cobalt, 100_000f64);
    //config.want_mineral.insert(Mineral::Scandium, 100_000f64);
    //config.want_mineral.insert(Mineral::Titanium, 100_000f64);
    //config.want_mineral.insert(Mineral::Tungsten, 100_000f64);
    //config.want_mineral.insert(Mineral::Chromium, 100_000f64);
    //config.want_mineral.insert(Mineral::Cadmium, 100_000f64);
    //config.want_mineral.insert(Mineral::Platinum, 100_000f64);
    //config.want_mineral.insert(Mineral::Vanadium, 100_000f64);
    //config.want_mineral.insert(Mineral::Caesium, 1_000f64);
    //config.want_mineral.insert(Mineral::Hafnium, 1_000f64);
    //config.want_mineral.insert(Mineral::Mercury, 1_000f64);
    //config.want_mineral.insert(Mineral::Technetium, 1_000f64);
    //config.want_mineral.insert(Mineral::Promethium, 1_000f64);
    //config.want_mineral.insert(Mineral::Neodymium, 1_000f64);
    //config.want_mineral.insert(Mineral::Dysprosium, 1_000f64);
    //config.want_mineral.insert(Mineral::Thulium, 1_000f64);
    //config.want_gas.insert(Gas::FulleriteC28, 5_000f64);

    config.want_mineral.insert(Mineral::StrontiumClathrates, 876f64);
    config.want_mineral.insert(Mineral::OxygenIsotopes, 6172f64);
    config.want_mineral.insert(Mineral::NitrogenIsotopes, 3472f64);
    config.want_mineral.insert(Mineral::LiquidOzone, 15300f64);
    config.want_mineral.insert(Mineral::HydrogenIsotopes, 5015f64);
    config.want_mineral.insert(Mineral::HeliumIsotopes, 5015f64);
    config.want_mineral.insert(Mineral::HeavyWater, 7434f64);

    config.prices_asteroid = fetch_asteroid_prices(&pool).await;
    config.prices_gas = fetch_gas_prices(&pool).await;
    config.limit_asteroid = fetch_astoid_limits(&pool).await;
    config.limit_gas = fetch_gas_limits(&pool).await;

    let result = calculate_ore(&config).unwrap();
    //let result = calculate_gas(&config).unwrap();

    debug_result_asteroid(&pool, config, result).await;
    //debug_result_gas(&pool, config, result).await;

    Ok(())
}

#[allow(dead_code)]
async fn debug_result_asteroid(
    pool:   &PgPool,
    config: Config,
    result: HashMap<i32, f64>,
) {
    let mut compressed_ore_copy = Vec::new();
    let mut compressed_ore_table = term_table::Table::new();
    compressed_ore_table.style = TableStyle::thin();
    compressed_ore_table.add_row(Row::new(vec![
        TableCell::builder("Ore").alignment(Alignment::Center).col_span(3)
    ]));
    compressed_ore_table.add_row(Row::new(vec![
        TableCell::new("Ore"),
        TableCell::new("Needed"),
        TableCell::new("Price"),
    ]));

    let mut total_price = 0u64;
    for (asteroid, amount) in result {
        let name = name(pool, asteroid).await;

        let price = (config.asteroid_price(&Asteroid::from_type_id(asteroid)) * amount).round() as u64;
        total_price += price;

        let price_format = price.to_formatted_string(&Locale::en);
        let amount = (amount as usize).to_formatted_string(&Locale::en);

        compressed_ore_copy.push(format!("{name}\t{amount}"));
        compressed_ore_table.add_row(Row::new(vec![
            TableCell::builder(format!("{name}")).alignment(Alignment::Left).col_span(1),
            TableCell::builder(amount).alignment(Alignment::Right).col_span(1),
            TableCell::builder(price_format).alignment(Alignment::Right).col_span(1),
        ]));
    }

    let total_price_format = total_price.to_formatted_string(&Locale::en);
    compressed_ore_table.add_row(Row::new(vec![
        TableCell::builder(format!("Total")).alignment(Alignment::Left).col_span(2),
        TableCell::builder(total_price_format).alignment(Alignment::Right).col_span(1),
    ]));

    println!("{}", compressed_ore_table.render());
    println!("{}", compressed_ore_copy.join("\n"));
}

async fn debug_result_gas(
    pool:   &PgPool,
    config: Config,
    result: HashMap<i32, f64>,
) {
    let mut compressed_ore_copy = Vec::new();
    let mut compressed_ore_table = term_table::Table::new();
    compressed_ore_table.style = TableStyle::thin();
    compressed_ore_table.add_row(Row::new(vec![
        TableCell::builder("Gas").alignment(Alignment::Center).col_span(3)
    ]));
    compressed_ore_table.add_row(Row::new(vec![
        TableCell::new("Gas"),
        TableCell::new("Needed"),
        TableCell::new("Price"),
    ]));

    let mut total_price = 0u64;
    for (asteroid, amount) in result {
        let name = name(pool, asteroid).await;

        let price = (config.gas_price(&Gas::from_type_id(asteroid)) * amount).round() as u64;
        total_price += price;

        let price_format = price.to_formatted_string(&Locale::en);
        let amount = (amount as usize).to_formatted_string(&Locale::en);

        compressed_ore_copy.push(format!("{name}\t{amount}"));
        compressed_ore_table.add_row(Row::new(vec![
            TableCell::builder(format!("{name}")).alignment(Alignment::Left).col_span(1),
            TableCell::builder(amount).alignment(Alignment::Right).col_span(1),
            TableCell::builder(price_format).alignment(Alignment::Right).col_span(1),
        ]));
    }

    let total_price_format = total_price.to_formatted_string(&Locale::en);
    compressed_ore_table.add_row(Row::new(vec![
        TableCell::builder(format!("Total")).alignment(Alignment::Left).col_span(2),
        TableCell::builder(total_price_format).alignment(Alignment::Right).col_span(1),
    ]));

    println!("{}", compressed_ore_table.render());
    println!("{}", compressed_ore_copy.join("\n"));
}

async fn fetch_asteroid_prices(
    pool: &PgPool,
) -> HashMap<Asteroid, f64> {
    let type_ids = Asteroid::type_ids();
    let mut prices = HashMap::new();

    sqlx::query!("
            SELECT
                average_price,
                type_id
            FROM market_price
            WHERE type_id = ANY($1)
        ",
            &type_ids,
        )
        .fetch_all(pool)
        .await
        .unwrap()
        .into_iter()
        .for_each(|x| {
            prices.insert(
                Asteroid::from_type_id(x.type_id),
                x.average_price,
            );
        });

    prices
}

async fn fetch_gas_prices(
    pool: &PgPool,
) -> HashMap<Gas, f64> {
    let type_ids = Gas::type_ids();
    let mut prices = HashMap::new();

    sqlx::query!("
            SELECT
                average_price,
                type_id
            FROM market_price
            WHERE type_id = ANY($1)
        ",
            &type_ids,
        )
        .fetch_all(pool)
        .await
        .unwrap()
        .into_iter()
        .for_each(|x| {
            prices.insert(
                Gas::from_type_id(x.type_id),
                x.average_price,
            );
        });

    prices
}

async fn fetch_astoid_limits(
    pool: &PgPool,
) -> HashMap<Asteroid, f64> {
    let type_ids = Asteroid::type_ids();
    let mut limits = HashMap::new();

    sqlx::query!("
            SELECT
                SUM(remaining) AS total,
                type_id
            FROM market_order_latest
            WHERE type_id = ANY($1)
            AND structure_id = 60003760
            AND is_buy = false
            GROUP BY type_id
        ",
            &type_ids,
        )
        .fetch_all(pool)
        .await
        .unwrap()
        .into_iter()
        .for_each(|x| {
            limits.insert(
                Asteroid::from_type_id(x.type_id),
                x.total.map(|x| x as f64).unwrap_or(0f64),
            );
        });

    limits
}

async fn fetch_gas_limits(
    pool: &PgPool,
) -> HashMap<Gas, f64> {
    let type_ids = Gas::type_ids();
    let mut limits = HashMap::new();

    sqlx::query!("
            SELECT
                SUM(remaining) AS total,
                type_id
            FROM market_order_latest
            WHERE type_id = ANY($1)
            AND structure_id = 60003760
            AND is_buy = false
            GROUP BY type_id
        ",
            &type_ids,
        )
        .fetch_all(pool)
        .await
        .unwrap()
        .into_iter()
        .for_each(|x| {
            limits.insert(
                Gas::from_type_id(x.type_id),
                x.total.map(|x| x as f64).unwrap_or(0f64),
            );
        });

    limits
}

async fn name(
    pool:    &PgPool,
    type_id: i32,
) -> String {
    sqlx::query!("
            SELECT name
            FROM item
            WHERE type_id = $1
        ",
            type_id,
        )
        .fetch_one(pool)
        .await
        .unwrap()
        .name
}

pub mod error;
pub mod config;

pub mod tasks;
mod utils;

use chrono::{Datelike, DateTime, Utc, Timelike, TimeZone};
use sqlx::PgPool;
use starfoundry_libs_eve_api::CredentialCache;
use std::sync::{Arc, Mutex};

/// This function needs to be started in a seperate thread as it will loop
/// forever
/// 
pub async fn execute(
    //config:           Config,
    pool:             PgPool,
    credential_cache: Arc<Mutex<CredentialCache>>,
) -> Result<(), Box<dyn std::error::Error>> {
    loop {
        // test if the EVE API-Server is available
        if !reqwest::get("https://esi.evetech.net/latest/status").await.unwrap().status().is_success() {
            tracing::error!("EVE-API-Server not available");
            std::thread::sleep(std::time::Duration::from_secs(60));
            continue;
        }

        let start = std::time::Instant::now();

        let (minute, hour, _) = current_time();
        // Skipp the first 30 mintes after server restart
        // Thanks to type limitations we can omit minute >= 0
        if hour == 11 && minute <= 30 {
            continue;
        }

        let mut handles = Vec::with_capacity(16);

        handles.push(tokio::spawn({
            let pool              = pool.clone();
            let credential_cache  = credential_cache.clone();

            async move {
                let task_duration = std::time::Instant::now();

                match tasks::wallet_character::task(
                    &pool,
                    credential_cache,
                ).await {

                    Err(e) => {
                        tracing::error!("Error in [WalletCharacter] {e}");
                    },
                    Ok(_)  => ()
                };

                tracing::info!("task [WalletCharacter] took {:.2}s", task_duration.elapsed().as_secs());
            }
        }));

        handles.push(tokio::spawn({
            let pool              = pool.clone();
            let credential_cache  = credential_cache.clone();

            async move {
                let task_duration = std::time::Instant::now();

                match tasks::wallet_corporation::task(
                    &pool,
                    credential_cache,
                ).await {

                    Err(e) => {
                        tracing::error!("Error in [WalletCorporation] {e}");
                    },
                    Ok(_)  => ()
                };

                tracing::info!("task [WalletCorporation] took {:.2}s", task_duration.elapsed().as_secs());
            }
        }));

        for handler in handles {
            // Only soft handle the error
            // We accept that the task may fail, in the next interval it will
            // start again
            if let Err(e) = handler.await {
                tracing::error!("Waiting for a handler failed {}", e);
            }
        }

        tracing::info!("Total time: {}s", start.elapsed().as_secs());

        std::thread::sleep(std::time::Duration::from_secs(30));
    }
}

/// Returns the current time rounded down to the current minute.
/// 
/// # Returns
/// 
/// Tuple of the current minute, hour and the complete date
/// * 0 > current minute
/// * 1 > current hour
/// * 2 > complete date as timestamp
/// 
fn current_time() -> (u32, u32, i64) {
    let ts = Utc::now();
    let date_time: DateTime<Utc> = Utc.with_ymd_and_hms(
        ts.year(), ts.month(), ts.day(),
        ts.hour(), ts.minute(), 0
    ).unwrap();

    (ts.minute(), ts.hour(), date_time.timestamp())
}

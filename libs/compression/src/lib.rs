#![recursion_limit = "256"]

mod asteroid_lp;
mod asteroid;
mod config;
mod error;
mod gas_lp;
mod gas;
mod reprocessing_efficiency;
mod reprocessing;

pub use self::asteroid_lp::*;
pub use self::asteroid::*;
pub use self::config::*;
pub use self::error::*;
pub use self::gas_lp::*;
pub use self::gas::*;
pub use self::reprocessing::*;
pub use self::reprocessing_efficiency::*;

use std::collections::HashMap;
use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct CompressionResult {
    pub want:    HashMap<i32, f64>,
    pub overage: HashMap<i32, f64>,
}

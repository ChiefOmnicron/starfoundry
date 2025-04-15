use std::collections::{HashMap, HashSet};
use std::f64;

use crate::asteroid::Asteroid;
use crate::{GasReprocessingEfficiency, OreReprocessingEfficiency};
use crate::gas::Gas;
use starfoundry_libs_types::TypeId;

#[derive(Debug)]
pub struct Config {
    pub want_mineral:      HashMap<Mineral, f64>,
    pub want_gas:          HashMap<Gas, f64>,
    pub ore_reprocessing:  OreReprocessingEfficiency,
    pub gas_decompression: GasReprocessingEfficiency,

    pub blacklist:         HashSet<Asteroid>,

    pub limit_asteroid:    HashMap<Asteroid, f64>,
    pub prices_asteroid:   HashMap<Asteroid, f64>,

    pub limit_gas:         HashMap<Gas, f64>,
    pub prices_gas:        HashMap<Gas, f64>,

    pub allow_minerals:          bool,
    pub allow_uncompressed_gas:  bool,
    pub allow_uncompressed_moon: bool,
    pub allow_uncompressed_ore:  bool,
}

impl Config {
    pub fn want_mineral(
        &self,
        mineral: Mineral,
    ) -> f64 {
        self.want_mineral
            .get(&mineral)
            .map(|x| *x)
            .unwrap_or_default()
    }

    pub fn want_gas(
        &self,
        gas: Gas,
    ) -> f64 {
        let mut percent_increase = self.gas_decompression.efficiency() - 100f64;
        if percent_increase.is_sign_negative() {
            percent_increase *= -1f64;
        }
        percent_increase /= self.gas_decompression.efficiency();
        percent_increase += 1f64;

        self.want_gas
            .get(&gas)
            .map(|x| *x * percent_increase)
            .unwrap_or_default()
    }

    pub fn asteroid_limit(
        &self,
        asteroid: &Asteroid,
    ) -> f64 {
        match asteroid {
            Asteroid::Tritanium         |
            Asteroid::Pyerite           |
            Asteroid::Mexallon          |
            Asteroid::Isogen            |
            Asteroid::Nocxium           |
            Asteroid::Zydrine           |
            Asteroid::Megacyte          |
            Asteroid::Morphite          |

            Asteroid::AtmosphericGases  |
            Asteroid::EvaporiteDeposits |
            Asteroid::Hydrocarbons      |
            Asteroid::Silicates         |
            Asteroid::Cobalt            |
            Asteroid::Scandium          |
            Asteroid::Titanium          |
            Asteroid::Tungsten          |
            Asteroid::Chromium          |
            Asteroid::Cadmium           |
            Asteroid::Platinum          |
            Asteroid::Vanadium          |
            Asteroid::Caesium           |
            Asteroid::Hafnium           |
            Asteroid::Mercury           |
            Asteroid::Technetium        |
            Asteroid::Promethium        |
            Asteroid::Neodymium         |
            Asteroid::Dysprosium        |
            Asteroid::Thulium => {
                self.limit_asteroid
                    .get(&asteroid)
                    // everything is calculated only on one ore, but for reprocessing
                    // you need 100
                    .map(|x| *x)
                    .unwrap_or_default()
            },
            _ => self.limit_asteroid
                    .get(&asteroid)
                    // everything is calculated only on one ore, but for reprocessing
                    // you need 100
                    .map(|x| (*x / 100f64).floor())
                    .unwrap_or_default()
        }
    }

    pub fn gas_limit(
        &self,
        gas: &Gas,
    ) -> f64 {
        self.limit_gas
            .get(&gas)
            // everything is calculated only on one ore, but for reprocessing
            // you need 100
            .map(|x| x.floor())
            .unwrap_or_default()
    }

    pub fn allowed_asteroid(
        &self,
        asteroid: &Asteroid,
    ) -> bool {
        if self.allow_minerals == false && asteroid.is_raw() {
            false
        } else if self.allow_uncompressed_ore == false && asteroid.is_ore() {
            false
        } else if self.allow_uncompressed_moon == false && asteroid.is_uncompressed_moon() {
            false
        } else {
            !self.blacklist.contains(&asteroid)
        }
    }

    pub fn allowed_gas(
        &self,
        gas: &Gas,
    ) -> bool {
        if self.allow_uncompressed_gas == false && gas.is_uncompressed() {
            false
        } else {
            true
        }
    }

    pub fn asteroid_price(
        &self,
        asteroid: &Asteroid,
    ) -> f64 {
        self.prices_asteroid
            .get(&asteroid)
            .map(|x| *x)
            .unwrap_or(1f64)
    }

    pub fn gas_price(
        &self,
        gas: &Gas,
    ) -> f64 {
        self.prices_gas
            .get(&gas)
            .map(|x| *x)
            .unwrap_or(1f64)
    }

    pub fn reprocessing_asteroid(
        &self,
    ) -> f64 {
        self.ore_reprocessing.efficiency()
    }
 }

impl Default for Config {
    fn default() -> Self {
        Self {
            want_mineral:      HashMap::new(),
            want_gas:          HashMap::new(),
            ore_reprocessing:      OreReprocessingEfficiency::default(),
            gas_decompression: GasReprocessingEfficiency::default(),

            blacklist:       HashSet::new(),

            limit_asteroid:  HashMap::new(),
            prices_asteroid: HashMap::new(),

            limit_gas:       HashMap::new(),
            prices_gas:      HashMap::new(),

            allow_minerals:          false,
            allow_uncompressed_gas:  false,
            allow_uncompressed_ore:  false,
            allow_uncompressed_moon: false,
        }
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Mineral {
    Tritanium,
    Pyerite,
    Mexallon,
    Isogen,
    Nocxium,
    Zydrine,
    Megacyte,
    Morphite,

    AtmosphericGases,
    EvaporiteDeposits,
    Hydrocarbons,
    Silicates,
    Cobalt,
    Scandium,
    Titanium,
    Tungsten,
    Chromium,
    Cadmium,
    Platinum,
    Vanadium,
    Caesium,
    Hafnium,
    Mercury,
    Technetium,
    Promethium,
    Neodymium,
    Dysprosium,
    Thulium,

    HeavyWater,
    LiquidOzone,
    StrontiumClathrates,
    HeliumIsotopes,
    NitrogenIsotopes,
    HydrogenIsotopes,
    OxygenIsotopes,
}

impl Mineral {
    pub fn to_type_id(self) -> TypeId {
        match self {
            Self::Tritanium             => TypeId(34),
            Self::Pyerite               => TypeId(35),
            Self::Mexallon              => TypeId(36),
            Self::Isogen                => TypeId(37),
            Self::Nocxium               => TypeId(38),
            Self::Zydrine               => TypeId(39),
            Self::Megacyte              => TypeId(40),
            Self::Morphite              => TypeId(11399),

            Self::AtmosphericGases      => TypeId(16634),
            Self::EvaporiteDeposits     => TypeId(16635),
            Self::Hydrocarbons          => TypeId(16633),
            Self::Silicates             => TypeId(16636),

            Self::Cobalt                => TypeId(16640),
            Self::Scandium              => TypeId(16639),
            Self::Titanium              => TypeId(16638),
            Self::Tungsten              => TypeId(16637),

            Self::Chromium              => TypeId(16641),
            Self::Cadmium               => TypeId(16643),
            Self::Platinum              => TypeId(16644),
            Self::Vanadium              => TypeId(16642),

            Self::Caesium               => TypeId(16647),
            Self::Hafnium               => TypeId(16648),
            Self::Mercury               => TypeId(16646),
            Self::Technetium            => TypeId(16649),
            Self::Promethium            => TypeId(16652),
            Self::Neodymium             => TypeId(16651),
            Self::Dysprosium            => TypeId(16650),
            Self::Thulium               => TypeId(16653),

            Self::HeavyWater            => TypeId(16272),
            Self::LiquidOzone           => TypeId(16273),
            Self::StrontiumClathrates   => TypeId(16275),
            Self::HeliumIsotopes        => TypeId(16274),
            Self::NitrogenIsotopes      => TypeId(17888),
            Self::HydrogenIsotopes      => TypeId(17889),
            Self::OxygenIsotopes        => TypeId(17887),
        }
    }
}

impl From<TypeId> for Mineral {
    fn from(value: TypeId) -> Self {
        match value {
            TypeId(34)    => Self::Tritanium,
            TypeId(35)    => Self::Pyerite,
            TypeId(36)    => Self::Mexallon,
            TypeId(37)    => Self::Isogen,
            TypeId(38)    => Self::Nocxium,
            TypeId(39)    => Self::Zydrine,
            TypeId(40)    => Self::Megacyte,
            TypeId(11399) => Self::Morphite,

            TypeId(16634) => Self::AtmosphericGases,
            TypeId(16635) => Self::EvaporiteDeposits,
            TypeId(16633) => Self::Hydrocarbons,
            TypeId(16636) => Self::Silicates,

            TypeId(16640) => Self::Cobalt,
            TypeId(16639) => Self::Scandium,
            TypeId(16638) => Self::Titanium,
            TypeId(16637) => Self::Tungsten,

            TypeId(16641) => Self::Chromium,
            TypeId(16643) => Self::Cadmium,
            TypeId(16644) => Self::Platinum,
            TypeId(16642) => Self::Vanadium,

            TypeId(16647) => Self::Caesium,
            TypeId(16648) => Self::Hafnium,
            TypeId(16646) => Self::Mercury,
            TypeId(16649) => Self::Technetium,
            TypeId(16652) => Self::Promethium,
            TypeId(16651) => Self::Neodymium,
            TypeId(16650) => Self::Dysprosium,
            TypeId(16653) => Self::Thulium,

            TypeId(16272) => Self::HeavyWater,
            TypeId(16273) => Self::LiquidOzone,
            TypeId(16275) => Self::StrontiumClathrates,
            TypeId(16274) => Self::HeliumIsotopes,
            TypeId(17888) => Self::NitrogenIsotopes,
            TypeId(17889) => Self::HydrogenIsotopes,
            TypeId(17887) => Self::OxygenIsotopes,

            _             => unimplemented!(),
        }
    }
}

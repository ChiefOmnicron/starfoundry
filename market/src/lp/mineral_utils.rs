use starfoundry_lib_types::TypeId;

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
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

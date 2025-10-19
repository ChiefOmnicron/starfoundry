use serde::{Deserialize, Serialize, Serializer, Deserializer};
use starfoundry_lib_types::TypeId;

#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub enum BonusVariations {
    Time(f32),
    Material(f32),
    Isk(f32),
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
pub enum StructureType {
    /// https://everef.net/type/35835
    Athanor,
    /// https://everef.net/type/35836
    Tatara,

    /// https://everef.net/type/35825
    Raitaru,
    /// https://everef.net/type/35826
    Azbel,
    /// https://everef.net/type/35827
    Sotiyo,

    /// https://everef.net/type/35832
    Astrahus,
    /// https://everef.net/type/35833
    Fortizar,
    /// https://everef.net/type/35834
    Keepstar,

    /// https://everef.net/type/47512
    MoreauFortizar,
    /// https://everef.net/type/47513
    DraccousFortizar,
    /// https://everef.net/type/47514
    HorizonFortizar,
    /// https://everef.net/type/47515
    MarginisFortizar,
    /// https://everef.net/type/47516
    PrometheusFortizar,
    /// https://everef.net/type/40340
    PalatineKeepstar,

    NpcAmarr,
    NpcJita,

    Invalid,
}

impl StructureType {
    /// Bonuses applied by the structure itself
    pub fn bonus(&self) -> Vec<BonusVariations> {
        match self {
            // Refinery
            Self::Athanor => Vec::new(),
            Self::Tatara  => vec![
                BonusVariations::Time(25.00f32),
            ],

            // Engineering
            Self::Raitaru => vec![
                BonusVariations::Time(15.00f32),
                BonusVariations::Material(1.00f32),
                BonusVariations::Isk(3.00f32),
            ],
            Self::Azbel   => vec![
                BonusVariations::Time(20.00f32),
                BonusVariations::Material(1.00f32),
                BonusVariations::Isk(4.00f32),
            ],
            Self::Sotiyo  => vec![
                BonusVariations::Time(30.00f32),
                BonusVariations::Material(1.00f32),
                BonusVariations::Isk(5.00f32),
            ],
            _             => Vec::new()
        }
    }

    pub fn deserialize<'de, D>(
        deserializer: D
    ) -> Result<Self, D::Error>
        where
            D: Deserializer<'de> {

        let v: i32 = Deserialize::deserialize(deserializer)?;
        Ok(StructureType::from(v))
    }

    pub fn serialize<S>(
        value: &Self,
        serializer: S
    ) -> Result<S::Ok, S::Error>
        where
            S: Serializer {

        serializer.serialize_i32(value.into())
    }

    pub fn into_i32(&self) -> i32 {
        self.into()
    }
}

impl From<i32> for StructureType {
    fn from(x: i32) -> Self {
        match x {
            35835 => Self::Athanor,
            35836 => Self::Tatara,

            35825 => Self::Raitaru,
            35826 => Self::Azbel,
            35827 => Self::Sotiyo,

            35832 => Self::Astrahus,
            35833 => Self::Fortizar,
            35834 => Self::Keepstar,

            47512 => Self::MoreauFortizar,
            47513 => Self::DraccousFortizar,
            47514 => Self::HorizonFortizar,
            47515 => Self::MarginisFortizar,
            47516 => Self::PrometheusFortizar,
            40340 => Self::PalatineKeepstar,

            1932  => Self::NpcAmarr,
            52678 => Self::NpcJita,

            _     => Self::Invalid,
        }
    }
}

impl From<StructureType> for i32 {
    fn from(x: StructureType) -> Self {
        Self::from(&x)
    }
}

impl From<StructureType> for TypeId {
    fn from(x: StructureType) -> Self {
        Self::from(&x)
    }
}

impl From<&StructureType> for i32 {
    fn from(x: &StructureType) -> Self {
        match *x {
            StructureType::Athanor            => 35835,
            StructureType::Tatara             => 35836,

            StructureType::Raitaru            => 35825,
            StructureType::Azbel              => 35826,
            StructureType::Sotiyo             => 35827,

            StructureType::Astrahus           => 35832,
            StructureType::Fortizar           => 35833,
            StructureType::Keepstar           => 35834,

            StructureType::MoreauFortizar     => 47512,
            StructureType::DraccousFortizar   => 47513,
            StructureType::HorizonFortizar    => 47514,
            StructureType::MarginisFortizar   => 47515,
            StructureType::PrometheusFortizar => 47516,
            StructureType::PalatineKeepstar   => 40340,

            StructureType::NpcAmarr           => 1932,
            StructureType::NpcJita            => 52678,

            StructureType::Invalid            => 0,
        }
    }
}

impl From<&StructureType> for TypeId {
    fn from(x: &StructureType) -> Self {
        match *x {
            StructureType::Athanor            => 35835,
            StructureType::Tatara             => 35836,

            StructureType::Raitaru            => 35825,
            StructureType::Azbel              => 35826,
            StructureType::Sotiyo             => 35827,

            StructureType::Astrahus           => 35832,
            StructureType::Fortizar           => 35833,
            StructureType::Keepstar           => 35834,

            StructureType::MoreauFortizar     => 47512,
            StructureType::DraccousFortizar   => 47513,
            StructureType::HorizonFortizar    => 47514,
            StructureType::MarginisFortizar   => 47515,
            StructureType::PrometheusFortizar => 47516,
            StructureType::PalatineKeepstar   => 40340,

            StructureType::NpcAmarr           => 1932,
            StructureType::NpcJita            => 52678,

            StructureType::Invalid            => 0,
        }.into()
    }
}

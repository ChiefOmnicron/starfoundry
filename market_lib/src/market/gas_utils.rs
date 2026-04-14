use starfoundry_lib_types::TypeId;

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Gas {
    AmberCytoserocin,
    AmberMykoserocin,
    CompressedAmberCytoserocin,
    CompressedAmberMykoserocin,

    AzureCytoserocin,
    AzureMykoserocin,
    CompressedAzureCytoserocin,
    CompressedAzureMykoserocin,

    CeladonCytoserocin,
    CeladonMykoserocin,
    CompressedCeladonCytoserocin,
    CompressedCeladonMykoserocin,

    GoldenCytoserocin,
    GoldenMykoserocin,
    CompressedGoldenCytoserocin,
    CompressedGoldenMykoserocin,

    LimeCytoserocin,
    LimeMykoserocin,
    CompressedLimeCytoserocin,
    CompressedLimeMykoserocin,

    MalachiteCytoserocin,
    MalachiteMykoserocin,
    CompressedMalachiteCytoserocin,
    CompressedMalachiteMykoserocin,

    VermillionCytoserocin,
    VermillionMykoserocin,
    CompressedVermillionCytoserocin,
    CompressedVermillionMykoserocin,

    ViridianCytoserocin,
    ViridianMykoserocin,
    CompressedViridianCytoserocin,
    CompressedViridianMykoserocin,

    FulleriteC28,
    CompressedFulleriteC28,

    FulleriteC32,
    CompressedFulleriteC32,

    FulleriteC320,
    CompressedFulleriteC320,

    FulleriteC50,
    CompressedFulleriteC50,

    FulleriteC540,
    CompressedFulleriteC540,

    FulleriteC60,
    CompressedFulleriteC60,

    FulleriteC70,
    CompressedFulleriteC70,

    FulleriteC72,
    CompressedFulleriteC72,

    FulleriteC84,
    CompressedFulleriteC84,
}

impl Gas {
    pub fn type_ids() -> Vec<TypeId> {
        vec![
            Self::AmberCytoserocin.to_type_id(),
            Self::AmberMykoserocin.to_type_id(),
            Self::CompressedAmberCytoserocin.to_type_id(),
            Self::CompressedAmberMykoserocin.to_type_id(),
            Self::AzureCytoserocin.to_type_id(),
            Self::AzureMykoserocin.to_type_id(),
            Self::CompressedAzureCytoserocin.to_type_id(),
            Self::CompressedAzureMykoserocin.to_type_id(),
            Self::CeladonCytoserocin.to_type_id(),
            Self::CeladonMykoserocin.to_type_id(),
            Self::CompressedCeladonCytoserocin.to_type_id(),
            Self::CompressedCeladonMykoserocin.to_type_id(),
            Self::GoldenCytoserocin.to_type_id(),
            Self::GoldenMykoserocin.to_type_id(),
            Self::CompressedGoldenCytoserocin.to_type_id(),
            Self::CompressedGoldenMykoserocin.to_type_id(),
            Self::LimeCytoserocin.to_type_id(),
            Self::LimeMykoserocin.to_type_id(),
            Self::CompressedLimeCytoserocin.to_type_id(),
            Self::CompressedLimeMykoserocin.to_type_id(),
            Self::MalachiteCytoserocin.to_type_id(),
            Self::MalachiteMykoserocin.to_type_id(),
            Self::CompressedMalachiteCytoserocin.to_type_id(),
            Self::CompressedMalachiteMykoserocin.to_type_id(),
            Self::VermillionCytoserocin.to_type_id(),
            Self::VermillionMykoserocin.to_type_id(),
            Self::CompressedVermillionCytoserocin.to_type_id(),
            Self::CompressedVermillionMykoserocin.to_type_id(),
            Self::ViridianCytoserocin.to_type_id(),
            Self::ViridianMykoserocin.to_type_id(),
            Self::CompressedViridianCytoserocin.to_type_id(),
            Self::CompressedViridianMykoserocin.to_type_id(),
            Self::FulleriteC28.to_type_id(),
            Self::CompressedFulleriteC28.to_type_id(),
            Self::FulleriteC32.to_type_id(),
            Self::CompressedFulleriteC32.to_type_id(),
            Self::FulleriteC320.to_type_id(),
            Self::CompressedFulleriteC320.to_type_id(),
            Self::FulleriteC50.to_type_id(),
            Self::CompressedFulleriteC50.to_type_id(),
            Self::FulleriteC540.to_type_id(),
            Self::CompressedFulleriteC540.to_type_id(),
            Self::FulleriteC60.to_type_id(),
            Self::CompressedFulleriteC60.to_type_id(),
            Self::FulleriteC70.to_type_id(),
            Self::CompressedFulleriteC70.to_type_id(),
            Self::FulleriteC72.to_type_id(),
            Self::CompressedFulleriteC72.to_type_id(),
            Self::FulleriteC84.to_type_id(),
            Self::CompressedFulleriteC84.to_type_id(),
        ]
    }

    pub fn to_type_id(&self) -> TypeId {
        match self {
            Self::AmberCytoserocin                  => 25268,
            Self::AmberMykoserocin                  => 28694,
            Self::CompressedAmberCytoserocin        => 62396,
            Self::CompressedAmberMykoserocin        => 62377,
            Self::AzureCytoserocin                  => 25279,
            Self::AzureMykoserocin                  => 28695,
            Self::CompressedAzureCytoserocin        => 62386,
            Self::CompressedAzureMykoserocin        => 62379,
            Self::CeladonCytoserocin                => 25275,
            Self::CeladonMykoserocin                => 28696,
            Self::CompressedCeladonCytoserocin      => 62387,
            Self::CompressedCeladonMykoserocin      => 62380,
            Self::GoldenCytoserocin                 => 25273,
            Self::GoldenMykoserocin                 => 28697,
            Self::CompressedGoldenCytoserocin       => 62390,
            Self::CompressedGoldenMykoserocin       => 62381,
            Self::LimeCytoserocin                   => 25277,
            Self::LimeMykoserocin                   => 28698,
            Self::CompressedLimeCytoserocin         => 62391,
            Self::CompressedLimeMykoserocin         => 62382,
            Self::MalachiteCytoserocin              => 25276,
            Self::MalachiteMykoserocin              => 28699,
            Self::CompressedMalachiteCytoserocin    => 62392,
            Self::CompressedMalachiteMykoserocin    => 62383,
            Self::VermillionCytoserocin             => 25278,
            Self::VermillionMykoserocin             => 28700,
            Self::CompressedVermillionCytoserocin   => 62393,
            Self::CompressedVermillionMykoserocin   => 62384,
            Self::ViridianCytoserocin               => 25274,
            Self::ViridianMykoserocin               => 28701,
            Self::CompressedViridianCytoserocin     => 62394,
            Self::CompressedViridianMykoserocin     => 62385,
            Self::FulleriteC28                      => 30375,
            Self::CompressedFulleriteC28            => 62402,
            Self::FulleriteC32                      => 30376,
            Self::CompressedFulleriteC32            => 62404,
            Self::FulleriteC320                     => 30377,
            Self::CompressedFulleriteC320           => 62406,
            Self::FulleriteC50                      => 30370,
            Self::CompressedFulleriteC50            => 62399,
            Self::FulleriteC540                     => 30378,
            Self::CompressedFulleriteC540           => 62405,
            Self::FulleriteC60                      => 30371,
            Self::CompressedFulleriteC60            => 62397,
            Self::FulleriteC70                      => 30372,
            Self::CompressedFulleriteC70            => 62398,
            Self::FulleriteC72                      => 30373,
            Self::CompressedFulleriteC72            => 62403,
            Self::FulleriteC84                      => 30374,
            Self::CompressedFulleriteC84            => 62400,
        }.into()
    }

    pub fn from_type_id(value: TypeId) -> Gas {
        match *value {
            25268 => Self::AmberCytoserocin,
            28694 => Self::AmberMykoserocin,
            62396 => Self::CompressedAmberCytoserocin,
            62377 => Self::CompressedAmberMykoserocin,
            25279 => Self::AzureCytoserocin,
            28695 => Self::AzureMykoserocin,
            62386 => Self::CompressedAzureCytoserocin,
            62379 => Self::CompressedAzureMykoserocin,
            25275 => Self::CeladonCytoserocin,
            28696 => Self::CeladonMykoserocin,
            62387 => Self::CompressedCeladonCytoserocin,
            62380 => Self::CompressedCeladonMykoserocin,
            25273 => Self::GoldenCytoserocin,
            28697 => Self::GoldenMykoserocin,
            62390 => Self::CompressedGoldenCytoserocin,
            62381 => Self::CompressedGoldenMykoserocin,
            25277 => Self::LimeCytoserocin,
            28698 => Self::LimeMykoserocin,
            62391 => Self::CompressedLimeCytoserocin,
            62382 => Self::CompressedLimeMykoserocin,
            25276 => Self::MalachiteCytoserocin,
            28699 => Self::MalachiteMykoserocin,
            62392 => Self::CompressedMalachiteCytoserocin,
            62383 => Self::CompressedMalachiteMykoserocin,
            25278 => Self::VermillionCytoserocin,
            28700 => Self::VermillionMykoserocin,
            62393 => Self::CompressedVermillionCytoserocin,
            62384 => Self::CompressedVermillionMykoserocin,
            25274 => Self::ViridianCytoserocin,
            28701 => Self::ViridianMykoserocin,
            62394 => Self::CompressedViridianCytoserocin,
            62385 => Self::CompressedViridianMykoserocin,
            30375 => Self::FulleriteC28,
            62402 => Self::CompressedFulleriteC28,
            30376 => Self::FulleriteC32,
            62404 => Self::CompressedFulleriteC32,
            30377 => Self::FulleriteC320,
            62406 => Self::CompressedFulleriteC320,
            30370 => Self::FulleriteC50,
            62399 => Self::CompressedFulleriteC50,
            30378 => Self::FulleriteC540,
            62405 => Self::CompressedFulleriteC540,
            30371 => Self::FulleriteC60,
            62397 => Self::CompressedFulleriteC60,
            30372 => Self::FulleriteC70,
            62398 => Self::CompressedFulleriteC70,
            30373 => Self::FulleriteC72,
            62403 => Self::CompressedFulleriteC72,
            30374 => Self::FulleriteC84,
            62400 => Self::CompressedFulleriteC84,

            _ => unimplemented!()
        }
    }

    pub fn is_uncompressed(&self) -> bool {
        match self {
            Self::AmberCytoserocin                  |
            Self::AmberMykoserocin                  |
            Self::AzureCytoserocin                  |
            Self::AzureMykoserocin                  |
            Self::CeladonCytoserocin                |
            Self::CeladonMykoserocin                |
            Self::GoldenCytoserocin                 |
            Self::GoldenMykoserocin                 |
            Self::LimeCytoserocin                   |
            Self::LimeMykoserocin                   |
            Self::MalachiteCytoserocin              |
            Self::MalachiteMykoserocin              |
            Self::VermillionCytoserocin             |
            Self::VermillionMykoserocin             |
            Self::ViridianCytoserocin               |
            Self::ViridianMykoserocin               |
            Self::FulleriteC28                      |
            Self::FulleriteC32                      |
            Self::FulleriteC320                     |
            Self::FulleriteC50                      |
            Self::FulleriteC540                     |
            Self::FulleriteC60                      |
            Self::FulleriteC70                      |
            Self::FulleriteC72                      |
            Self::FulleriteC84                      => true,
            _                                       => false,
        }
    }

    pub fn is_compressed(&self) -> bool {
        !self.is_uncompressed()
    }

    pub fn to_uncompressed_type_id(
        &self
    ) -> TypeId {
        match self {
            Self::CompressedAmberCytoserocin        => Self::AmberCytoserocin.to_type_id(),
            Self::AmberCytoserocin                  => Self::AmberCytoserocin.to_type_id(),
            Self::CompressedAmberMykoserocin        => Self::AmberMykoserocin.to_type_id(),
            Self::AmberMykoserocin                  => Self::AmberMykoserocin.to_type_id(),
            Self::CompressedAzureCytoserocin        => Self::AzureCytoserocin.to_type_id(),
            Self::AzureCytoserocin                  => Self::AzureCytoserocin.to_type_id(),
            Self::CompressedAzureMykoserocin        => Self::AzureMykoserocin.to_type_id(),
            Self::AzureMykoserocin                  => Self::AzureMykoserocin.to_type_id(),
            Self::CompressedCeladonCytoserocin      => Self::CeladonCytoserocin.to_type_id(),
            Self::CeladonCytoserocin                => Self::CeladonCytoserocin.to_type_id(),
            Self::CompressedCeladonMykoserocin      => Self::CeladonMykoserocin.to_type_id(),
            Self::CeladonMykoserocin                => Self::CeladonMykoserocin.to_type_id(),
            Self::CompressedGoldenCytoserocin       => Self::GoldenCytoserocin.to_type_id(),
            Self::GoldenCytoserocin                 => Self::GoldenCytoserocin.to_type_id(),
            Self::CompressedGoldenMykoserocin       => Self::GoldenMykoserocin.to_type_id(),
            Self::GoldenMykoserocin                 => Self::GoldenMykoserocin.to_type_id(),
            Self::CompressedLimeCytoserocin         => Self::LimeCytoserocin.to_type_id(),
            Self::LimeCytoserocin                   => Self::LimeCytoserocin.to_type_id(),
            Self::CompressedLimeMykoserocin         => Self::LimeMykoserocin.to_type_id(),
            Self::LimeMykoserocin                   => Self::LimeMykoserocin.to_type_id(),
            Self::CompressedMalachiteCytoserocin    => Self::MalachiteCytoserocin.to_type_id(),
            Self::MalachiteCytoserocin              => Self::MalachiteCytoserocin.to_type_id(),
            Self::CompressedMalachiteMykoserocin    => Self::MalachiteMykoserocin.to_type_id(),
            Self::MalachiteMykoserocin              => Self::MalachiteMykoserocin.to_type_id(),
            Self::CompressedVermillionCytoserocin   => Self::VermillionCytoserocin.to_type_id(),
            Self::VermillionCytoserocin             => Self::VermillionCytoserocin.to_type_id(),
            Self::CompressedVermillionMykoserocin   => Self::VermillionMykoserocin.to_type_id(),
            Self::VermillionMykoserocin             => Self::VermillionMykoserocin.to_type_id(),
            Self::CompressedViridianCytoserocin     => Self::ViridianCytoserocin.to_type_id(),
            Self::ViridianCytoserocin               => Self::ViridianCytoserocin.to_type_id(),
            Self::CompressedViridianMykoserocin     => Self::ViridianMykoserocin.to_type_id(),
            Self::ViridianMykoserocin               => Self::ViridianMykoserocin.to_type_id(),
            Self::CompressedFulleriteC28            => Self::FulleriteC28.to_type_id(),
            Self::FulleriteC28                      => Self::FulleriteC28.to_type_id(),
            Self::CompressedFulleriteC32            => Self::FulleriteC32.to_type_id(),
            Self::FulleriteC32                      => Self::FulleriteC32.to_type_id(),
            Self::CompressedFulleriteC320           => Self::FulleriteC320.to_type_id(),
            Self::FulleriteC320                     => Self::FulleriteC320.to_type_id(),
            Self::CompressedFulleriteC50            => Self::FulleriteC50.to_type_id(),
            Self::FulleriteC50                      => Self::FulleriteC50.to_type_id(),
            Self::CompressedFulleriteC540           => Self::FulleriteC540.to_type_id(),
            Self::FulleriteC540                     => Self::FulleriteC540.to_type_id(),
            Self::CompressedFulleriteC60            => Self::FulleriteC60.to_type_id(),
            Self::FulleriteC60                      => Self::FulleriteC60.to_type_id(),
            Self::CompressedFulleriteC70            => Self::FulleriteC70.to_type_id(),
            Self::FulleriteC70                      => Self::FulleriteC70.to_type_id(),
            Self::CompressedFulleriteC72            => Self::FulleriteC72.to_type_id(),
            Self::FulleriteC72                      => Self::FulleriteC72.to_type_id(),
            Self::CompressedFulleriteC84            => Self::FulleriteC84.to_type_id(),
            Self::FulleriteC84                      => Self::FulleriteC84.to_type_id(),
        }
    }

    pub fn to_compressed_type_id(
        &self
    ) -> TypeId {
        match self {
            Self::CompressedAmberCytoserocin        => Self::CompressedAmberCytoserocin.to_type_id(),
            Self::AmberCytoserocin                  => Self::CompressedAmberCytoserocin.to_type_id(),
            Self::CompressedAmberMykoserocin        => Self::CompressedAmberMykoserocin.to_type_id(),
            Self::AmberMykoserocin                  => Self::CompressedAmberMykoserocin.to_type_id(),
            Self::CompressedAzureCytoserocin        => Self::CompressedAzureCytoserocin.to_type_id(),
            Self::AzureCytoserocin                  => Self::CompressedAzureCytoserocin.to_type_id(),
            Self::CompressedAzureMykoserocin        => Self::CompressedAzureMykoserocin.to_type_id(),
            Self::AzureMykoserocin                  => Self::CompressedAzureMykoserocin.to_type_id(),
            Self::CompressedCeladonCytoserocin      => Self::CompressedCeladonCytoserocin.to_type_id(),
            Self::CeladonCytoserocin                => Self::CompressedCeladonCytoserocin.to_type_id(),
            Self::CompressedCeladonMykoserocin      => Self::CompressedCeladonMykoserocin.to_type_id(),
            Self::CeladonMykoserocin                => Self::CompressedCeladonMykoserocin.to_type_id(),
            Self::CompressedGoldenCytoserocin       => Self::CompressedGoldenCytoserocin.to_type_id(),
            Self::GoldenCytoserocin                 => Self::CompressedGoldenCytoserocin.to_type_id(),
            Self::CompressedGoldenMykoserocin       => Self::CompressedGoldenMykoserocin.to_type_id(),
            Self::GoldenMykoserocin                 => Self::CompressedGoldenMykoserocin.to_type_id(),
            Self::CompressedLimeCytoserocin         => Self::CompressedLimeCytoserocin.to_type_id(),
            Self::LimeCytoserocin                   => Self::CompressedLimeCytoserocin.to_type_id(),
            Self::CompressedLimeMykoserocin         => Self::CompressedLimeMykoserocin.to_type_id(),
            Self::LimeMykoserocin                   => Self::CompressedLimeMykoserocin.to_type_id(),
            Self::CompressedMalachiteCytoserocin    => Self::CompressedMalachiteCytoserocin.to_type_id(),
            Self::MalachiteCytoserocin              => Self::CompressedMalachiteCytoserocin.to_type_id(),
            Self::CompressedMalachiteMykoserocin    => Self::CompressedMalachiteMykoserocin.to_type_id(),
            Self::MalachiteMykoserocin              => Self::CompressedMalachiteMykoserocin.to_type_id(),
            Self::CompressedVermillionCytoserocin   => Self::CompressedVermillionCytoserocin.to_type_id(),
            Self::VermillionCytoserocin             => Self::CompressedVermillionCytoserocin.to_type_id(),
            Self::CompressedVermillionMykoserocin   => Self::CompressedVermillionMykoserocin.to_type_id(),
            Self::VermillionMykoserocin             => Self::CompressedVermillionMykoserocin.to_type_id(),
            Self::CompressedViridianCytoserocin     => Self::CompressedViridianCytoserocin.to_type_id(),
            Self::ViridianCytoserocin               => Self::CompressedViridianCytoserocin.to_type_id(),
            Self::CompressedViridianMykoserocin     => Self::CompressedViridianMykoserocin.to_type_id(),
            Self::ViridianMykoserocin               => Self::CompressedViridianMykoserocin.to_type_id(),
            Self::CompressedFulleriteC28            => Self::CompressedFulleriteC28.to_type_id(),
            Self::FulleriteC28                      => Self::CompressedFulleriteC28.to_type_id(),
            Self::CompressedFulleriteC32            => Self::CompressedFulleriteC32.to_type_id(),
            Self::FulleriteC32                      => Self::CompressedFulleriteC32.to_type_id(),
            Self::CompressedFulleriteC320           => Self::CompressedFulleriteC320.to_type_id(),
            Self::FulleriteC320                     => Self::CompressedFulleriteC320.to_type_id(),
            Self::CompressedFulleriteC50            => Self::CompressedFulleriteC50.to_type_id(),
            Self::FulleriteC50                      => Self::CompressedFulleriteC50.to_type_id(),
            Self::CompressedFulleriteC540           => Self::CompressedFulleriteC540.to_type_id(),
            Self::FulleriteC540                     => Self::CompressedFulleriteC540.to_type_id(),
            Self::CompressedFulleriteC60            => Self::CompressedFulleriteC60.to_type_id(),
            Self::FulleriteC60                      => Self::CompressedFulleriteC60.to_type_id(),
            Self::CompressedFulleriteC70            => Self::CompressedFulleriteC70.to_type_id(),
            Self::FulleriteC70                      => Self::CompressedFulleriteC70.to_type_id(),
            Self::CompressedFulleriteC72            => Self::CompressedFulleriteC72.to_type_id(),
            Self::FulleriteC72                      => Self::CompressedFulleriteC72.to_type_id(),
            Self::CompressedFulleriteC84            => Self::CompressedFulleriteC84.to_type_id(),
            Self::FulleriteC84                      => Self::CompressedFulleriteC84.to_type_id(),
        }
    }

    pub fn compressed_type_ids() -> Vec<TypeId> {
        Gas::type_ids()
            .into_iter()
            .map(|x| Gas::from_type_id(x.into()))
            .filter(|x| !x.is_uncompressed())
            .map(|x| x.to_type_id())
            .map(Into::into)
            .collect::<Vec<_>>()
    }

    pub fn is_gas(
        type_id: TypeId,
    ) -> bool {
        if Gas::type_ids().contains(&type_id) {
            true
        } else {
            false
        }
    }
}

impl TryFrom<TypeId> for Gas {
    type Error = String;

    fn try_from(value: TypeId) -> Result<Self, Self::Error> {
        match *value {
            25268 => Ok(Self::AmberCytoserocin),
            28694 => Ok(Self::AmberMykoserocin),
            62396 => Ok(Self::CompressedAmberCytoserocin),
            62377 => Ok(Self::CompressedAmberMykoserocin),
            25279 => Ok(Self::AzureCytoserocin),
            28695 => Ok(Self::AzureMykoserocin),
            62386 => Ok(Self::CompressedAzureCytoserocin),
            62379 => Ok(Self::CompressedAzureMykoserocin),
            25275 => Ok(Self::CeladonCytoserocin),
            28696 => Ok(Self::CeladonMykoserocin),
            62387 => Ok(Self::CompressedCeladonCytoserocin),
            62380 => Ok(Self::CompressedCeladonMykoserocin),
            25273 => Ok(Self::GoldenCytoserocin),
            28697 => Ok(Self::GoldenMykoserocin),
            62390 => Ok(Self::CompressedGoldenCytoserocin),
            62381 => Ok(Self::CompressedGoldenMykoserocin),
            25277 => Ok(Self::LimeCytoserocin),
            28698 => Ok(Self::LimeMykoserocin),
            62391 => Ok(Self::CompressedLimeCytoserocin),
            62382 => Ok(Self::CompressedLimeMykoserocin),
            25276 => Ok(Self::MalachiteCytoserocin),
            28699 => Ok(Self::MalachiteMykoserocin),
            62392 => Ok(Self::CompressedMalachiteCytoserocin),
            62383 => Ok(Self::CompressedMalachiteMykoserocin),
            25278 => Ok(Self::VermillionCytoserocin),
            28700 => Ok(Self::VermillionMykoserocin),
            62393 => Ok(Self::CompressedVermillionCytoserocin),
            62384 => Ok(Self::CompressedVermillionMykoserocin),
            25274 => Ok(Self::ViridianCytoserocin),
            28701 => Ok(Self::ViridianMykoserocin),
            62394 => Ok(Self::CompressedViridianCytoserocin),
            62385 => Ok(Self::CompressedViridianMykoserocin),
            30375 => Ok(Self::FulleriteC28),
            62402 => Ok(Self::CompressedFulleriteC28),
            30376 => Ok(Self::FulleriteC32),
            62404 => Ok(Self::CompressedFulleriteC32),
            30377 => Ok(Self::FulleriteC320),
            62406 => Ok(Self::CompressedFulleriteC320),
            30370 => Ok(Self::FulleriteC50),
            62399 => Ok(Self::CompressedFulleriteC50),
            30378 => Ok(Self::FulleriteC540),
            62405 => Ok(Self::CompressedFulleriteC540),
            30371 => Ok(Self::FulleriteC60),
            62397 => Ok(Self::CompressedFulleriteC60),
            30372 => Ok(Self::FulleriteC70),
            62398 => Ok(Self::CompressedFulleriteC70),
            30373 => Ok(Self::FulleriteC72),
            62403 => Ok(Self::CompressedFulleriteC72),
            30374 => Ok(Self::FulleriteC84),
            62400 => Ok(Self::CompressedFulleriteC84),

            _ => Err(format!("invalid gas type_id: {}", value))
        }
    }
}

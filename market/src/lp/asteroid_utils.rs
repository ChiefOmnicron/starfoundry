use std::collections::HashMap;
use crate::lp::Mineral;
use starfoundry_lib_types::TypeId;

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Asteroid {
    ArkonorIGrade,
    ArkonorIIGrade,
    ArkonorIIIGrade,
    ArkonorIVGrade,
    CompressedArkonorIGrade,
    CompressedArkonorIIGrade,
    CompressedArkonorIIIGrade,
    CompressedArkonorIVGrade,
    BatchCompressedArkonorIGrade,
    BatchCompressedArkonorIIGrade,
    BatchCompressedArkonorIIIGrade,
    BatchCompressedArkonorIVGrade,

    BezdnacineIGrade,
    BezdnacineIIGrade,
    BezdnacineIIIGrade,
    CompressedBezdnacineIGrade,
    CompressedBezdnacineIIGrade,
    CompressedBezdnacineIIIGrade,

    BistotIGrade,
    BistotIIGrade,
    BistotIIIGrade,
    BistotIVGrade,
    CompressedBistotIGrade,
    CompressedBistotIIGrade,
    CompressedBistotIIIGrade,
    CompressedBistotIVGrade,
    BatchCompressedBistotIGrade,
    BatchCompressedBistotIIGrade,
    BatchCompressedBistotIIIGrade,
    BatchCompressedBistotIVGrade,

    CrokiteIGrade,
    CrokiteIIGrade,
    CrokiteIIIGrade,
    CrokiteIVGrade,
    CompressedCrokiteIGrade,
    CompressedCrokiteIIGrade,
    CompressedCrokiteIIIGrade,
    CompressedCrokiteIVGrade,
    BatchCompressedCrokiteIGrade,
    BatchCompressedCrokiteIIGrade,
    BatchCompressedCrokiteIIIGrade,
    BatchCompressedCrokiteIVGrade,

    DarkOchreIGrade,
    OchreIIGrade,
    OchreIIIGrade,
    OchreIVGrade,
    CompressedDarkOchreIGrade,
    CompressedOchreIIGrade,
    CompressedOchreIIIGrade,
    CompressedOchreIVGrade,
    BatchCompressedDarkOchreIGrade,
    BatchCompressedOchreIIGrade,
    BatchCompressedOchreIIIGrade,
    BatchCompressedOchreIVGrade,

    DuciniumIGrade,
    DuciniumIIGrade,
    DuciniumIIIGrade,
    DuciniumIVGrade,
    CompressedDuciniumIGrade,
    CompressedDuciniumIIGrade,
    CompressedDuciniumIIIGrade,
    CompressedDuciniumIVGrade,

    EifyriumIGrade,
    EifyriumIIGrade,
    EifyriumIIIGrade,
    EifyriumIVGrade,
    CompressedEifyriumIGrade,
    CompressedEifyriumIIGrade,
    CompressedEifyriumIIIGrade,
    CompressedEifyriumIVGrade,

    GneissIGrade,
    GneissIIGrade,
    GneissIIIGrade,
    GneissIVGrade,
    CompressedGneissIGrade,
    CompressedGneissIIGrade,
    CompressedGneissIIIGrade,
    CompressedGneissIVGrade,
    BatchCompressedGneissIGrade,
    BatchCompressedGneissIIGrade,
    BatchCompressedGneissIIIGrade,
    BatchCompressedGneissIVGrade,

    GriemeerIGrade,
    GriemeerIIGrade,
    GriemeerIIIGrade,
    GriemeerIVGrade,
    CompressedGriemeerIGrade,
    CompressedGriemeerIIGrade,
    CompressedGriemeerIIIGrade,
    CompressedGriemeerIVGrade,

    HedbergiteIGrade,
    HedbergiteIIGrade,
    HedbergiteIIIGrade,
    HedbergiteIVGrade,
    CompressedHedbergiteIGrade,
    CompressedHedbergiteIIGrade,
    CompressedHedbergiteIIIGrade,
    CompressedHedbergiteIVGrade,
    BatchCompressedHedbergiteIGrade,
    BatchCompressedHedbergiteIIGrade,
    BatchCompressedHedbergiteIIIGrade,
    BatchCompressedHedbergiteIVGrade,

    HemorphiteIGrade,
    HemorphiteIIGrade,
    HemorphiteIIIGrade,
    HemorphiteIVGrade,
    CompressedHemorphiteIGrade,
    CompressedHemorphiteIIGrade,
    CompressedHemorphiteIIIGrade,
    CompressedHemorphiteIVGrade,
    BatchCompressedHemorphiteIGrade,
    BatchCompressedHemorphiteIIGrade,
    BatchCompressedHemorphiteIIIGrade,
    BatchCompressedHemorphiteIVGrade,

    HezorimeIGrade,
    HezorimeIIGrade,
    HezorimeIIIGrade,
    HezorimeIVGrade,
    CompressedHezorimeIGrade,
    CompressedHezorimeIIGrade,
    CompressedHezorimeIIIGrade,
    CompressedHezorimeIVGrade,

    JaspetIGrade,
    JaspetIIGrade,
    JaspetIIIGrade,
    JaspetIVGrade,
    CompressedJaspetIGrade,
    CompressedJaspetIIGrade,
    CompressedJaspetIIIGrade,
    CompressedJaspetIVGrade,
    BatchCompressedJaspetIGrade,
    BatchCompressedJaspetIIGrade,
    BatchCompressedJaspetIIIGrade,
    BatchCompressedJaspetIVGrade,

    KerniteIGrade,
    KerniteIIGrade,
    KerniteIIIGrade,
    KerniteIVGrade,
    CompressedKerniteIGrade,
    CompressedKerniteIIGrade,
    CompressedKerniteIIIGrade,
    CompressedKerniteIVGrade,
    BatchCompressedKerniteIGrade,
    BatchCompressedKerniteIIGrade,
    BatchCompressedKerniteIIIGrade,
    BatchCompressedKerniteIVGrade,

    KylixiumIGrade,
    KylixiumIIGrade,
    KylixiumIIIGrade,
    KylixiumIVGrade,
    CompressedKylixiumIGrade,
    CompressedKylixiumIIGrade,
    CompressedKylixiumIIIGrade,
    CompressedKylixiumIVGrade,

    Mercoxit,
    MagmaMercoxit,
    VitreousMercoxit,
    CompressedMercoxit,
    CompressedMagmaMercoxit,
    CompressedVitreousMercoxit,
    BatchCompressedMercoxit,
    BatchCompressedMagmaMercoxit,
    BatchCompressedVitreousMercoxit,

    Mordunium,
    PlumMordunium,
    PrizeMordunium,
    PlunderMordunium,
    CompressedMordunium,
    CompressedPlumMordunium,
    CompressedPrizeMordunium,
    CompressedPlunderMordunium,

    Nocxite,
    FragrantNocxite,
    IntoxicatingNocxite,
    AmbrosialNocxite,
    CompressedNocxite,
    CompressedFragrantNocxite,
    CompressedIntoxicatingNocxite,
    CompressedAmbrosialNocxite,

    Omber,
    SilveryOmber,
    GoldenOmber,
    PlatinoidOmber,
    CompressedOmber,
    CompressedSilveryOmber,
    CompressedGoldenOmber,
    CompressedPlatinoidOmber,
    BatchCompressedOmber,
    BatchCompressedSilveryOmber,
    BatchCompressedGoldenOmber,
    BatchCompressedPlatinoidOmber,

    Plagioclase,
    AzurePlagioclase,
    RichPlagioclase,
    SparklingPlagioclase,
    CompressedPlagioclase,
    CompressedAzurePlagioclase,
    CompressedRichPlagioclase,
    CompressedSparklingPlagioclase,
    BatchCompressedPlagioclase,
    BatchCompressedAzurePlagioclase,
    BatchCompressedRichPlagioclase,
    BatchCompressedSparklingPlagioclase,

    Pyroxeres,
    SolidPyroxeres,
    ViscousPyroxeres,
    OpulentPyroxeres,
    CompressedPyroxeres,
    CompressedSolidPyroxeres,
    CompressedViscousPyroxeres,
    CompressedOpulentPyroxeres,
    BatchCompressedPyroxeres,
    BatchCompressedSolidPyroxeres,
    BatchCompressedViscousPyroxeres,
    BatchCompressedOpulentPyroxeres,

    Rakovene,
    RakoveneIIGrade,
    RakoveneIIIGrade,
    CompressedRakovene,
    CompressedRakoveneIIGrade,
    CompressedRakoveneIIIGrade,

    Scordite,
    CondensedScordite,
    MassiveScordite,
    GlossyScordite,
    CompressedScordite,
    CompressedCondensedScordite,
    CompressedMassiveScordite,
    CompressedGlossyScordite,
    BatchCompressedScordite,
    BatchCompressedCondensedScordite,
    BatchCompressedMassiveScordite,
    BatchCompressedGlossyScordite,

    Spodumain,
    BrightSpodumain,
    GleamingSpodumain,
    DazzlingSpodumain,
    CompressedSpodumain,
    CompressedBrightSpodumain,
    CompressedGleamingSpodumain,
    CompressedDazzlingSpodumain,
    BatchCompressedSpodumain,
    BatchCompressedBrightSpodumain,
    BatchCompressedGleamingSpodumain,
    BatchCompressedDazzlingSpodumain,

    Talassonite,
    TalassoniteIIGrade,
    TalassoniteIIIGrade,
    CompressedTalassonite,
    CompressedTalassoniteIIGrade,
    CompressedTalassoniteIIIGrade,

    Ueganite,
    FoggyUeganite,
    OvercastUeganite,
    StormyUeganite,
    CompressedUeganite,
    CompressedFoggyUeganite,
    CompressedOvercastUeganite,
    CompressedStormyUeganite,

    Veldspar,
    ConcentratedVeldspar,
    DenseVeldspar,
    StableVeldspar,
    CompressedVeldspar,
    CompressedConcentratedVeldspar,
    CompressedDenseVeldspar,
    CompressedStableVeldspar,
    BatchCompressedVeldspar,
    BatchCompressedConcentratedVeldspar,
    BatchCompressedDenseVeldspar,
    BatchCompressedStableVeldspar,

    Ytirium,
    BootlegYtirium,
    FirewaterYtirium,
    MoonshineYtirium,
    CompressedYtirium,
    CompressedBootlegYtirium,
    CompressedFirewaterYtirium,
    CompressedMoonshineYtirium,

    // R4
    Bitumens,
    BrimfulBitumens,
    GlisteningBitumens,
    CompressedBitumens,
    CompressedBrimfulBitumens,
    CompressedGlisteningBitumens,

    Coesite,
    BrimfulCoesite,
    GlisteningCoesite,
    CompressedCoesite,
    CompressedBrimfulCoesite,
    CompressedGlisteningCoesite,

    Sylvite,
    BrimfulSylvite,
    GlisteningSylvite,
    CompressedSylvite,
    CompressedBrimfulSylvite,
    CompressedGlisteningSylvite,

    Zeolites,
    BrimfulZeolites,
    GlisteningZeolites,
    CompressedZeolites,
    CompressedBrimfulZeolites,
    CompressedGlisteningZeolites,

    // R8
    Cobaltite,
    CopiousCobaltite,
    TwinklingCobaltite,
    CompressedCobaltite,
    CompressedCopiousCobaltite,
    CompressedTwinklingCobaltite,

    Euxenite,
    CopiousEuxenite,
    TwinklingEuxenite,
    CompressedEuxenite,
    CompressedCopiousEuxenite,
    CompressedTwinklingEuxenite,

    Scheelite,
    CopiousScheelite,
    TwinklingScheelite,
    CompressedScheelite,
    CompressedCopiousScheelite,
    CompressedTwinklingScheelite,

    Titanite,
    CopiousTitanite,
    TwinklingTitanite,
    CompressedTitanite,
    CompressedCopiousTitanite,
    CompressedTwinklingTitanite,

    // R16
    Chromite,
    LavishChromite,
    ShimmeringChromite,
    CompressedChromite,
    CompressedLavishChromite,
    CompressedShimmeringChromite,

    Otavite,
    LavishOtavite,
    ShimmeringOtavite,
    CompressedOtavite,
    CompressedLavishOtavite,
    CompressedShimmeringOtavite,

    Sperrylite,
    LavishSperrylite,
    ShimmeringSperrylite,
    CompressedSperrylite,
    CompressedLavishSperrylite,
    CompressedShimmeringSperrylite,

    Vanadinite,
    LavishVanadinite,
    ShimmeringVanadinite,
    CompressedVanadinite,
    CompressedLavishVanadinite,
    CompressedShimmeringVanadinite,

    // R32
    Carnotite,
    RepleteCarnotite,
    GlowingCarnotite,
    CompressedCarnotite,
    CompressedRepleteCarnotite,
    CompressedGlowingCarnotite,

    Cinnabar,
    RepleteCinnabar,
    GlowingCinnabar,
    CompressedCinnabar,
    CompressedRepleteCinnabar,
    CompressedGlowingCinnabar,

    Pollucite,
    RepletePollucite,
    GlowingPollucite,
    CompressedPollucite,
    CompressedRepletePollucite,
    CompressedGlowingPollucite,

    Zircon,
    RepleteZircon,
    GlowingZircon,
    CompressedZircon,
    CompressedRepleteZircon,
    CompressedGlowingZircon,

    // R64
    Loparite,
    BountifulLoparite,
    ShiningLoparite,
    CompressedLoparite,
    CompressedBountifulLoparite,
    CompressedShiningLoparite,

    Monazite,
    BountifulMonazite,
    ShiningMonazite,
    CompressedMonazite,
    CompressedBountifulMonazite,
    CompressedShiningMonazite,

    Xenotime,
    BountifulXenotime,
    ShiningXenotime,
    CompressedXenotime,
    CompressedBountifulXenotime,
    CompressedShiningXenotime,

    Ytterbite,
    BountifulYtterbite,
    ShiningYtterbite,
    CompressedYtterbite,
    CompressedBountifulYtterbite,
    CompressedShiningYtterbite,

    // technically not asteroids, but needed for the calculations
    Tritanium,
    Pyerite,
    Mexallon,
    Isogen,
    Nocxium,
    Zydrine,
    Megacyte,
    Morphite,

    // R4
    AtmosphericGases,
    EvaporiteDeposits,
    Hydrocarbons,
    Silicates,

    // R8
    Cobalt,
    Scandium,
    Titanium,
    Tungsten,

    // R16
    Chromium,
    Cadmium,
    Platinum,
    Vanadium,

    // R32
    Caesium,
    Hafnium,
    Mercury,
    Technetium,

    // R64
    Promethium,
    Neodymium,
    Dysprosium,
    Thulium,

    // Ice
    BlueIce,
    BlueIceIVGrade,
    CompressedBlueIce,
    CompressedBlueIceIVGrade,
    IcicleIIGrade,
    IcicleIVGradeIIGrade,
    CompressedIcicleIIGrade,
    CompressedIcicleIVGradeIIGrade,
    GlacialMass,
    GlacialMassIVGrade,
    CompressedGlacialMass,
    CompressedGlacialMassIVGrade,
    WhiteGlaze,
    WhiteGlazeIVGrade,
    CompressedWhiteGlaze,
    CompressedWhiteGlazeIVGrade,
    DarkGlitter,
    CompressedDarkGlitter,
    Gelidus,
    CompressedGelidus,
    GlareCrust,
    CompressedGlareCrust,
    Krystallos,
    CompressedKrystallos,
}

impl Asteroid {
    pub fn type_ids() -> Vec<i32> {
        vec![
            Self::ArkonorIGrade.to_type_id(),
            Self::ArkonorIIGrade.to_type_id(),
            Self::ArkonorIIIGrade.to_type_id(),
            Self::ArkonorIVGrade.to_type_id(),
            Self::CompressedArkonorIGrade.to_type_id(),
            Self::CompressedArkonorIIGrade.to_type_id(),
            Self::CompressedArkonorIIIGrade.to_type_id(),
            Self::CompressedArkonorIVGrade.to_type_id(),

            Self::BezdnacineIGrade.to_type_id(),
            Self::BezdnacineIIGrade.to_type_id(),
            Self::BezdnacineIIIGrade.to_type_id(),
            Self::CompressedBezdnacineIGrade.to_type_id(),
            Self::CompressedBezdnacineIIGrade.to_type_id(),
            Self::CompressedBezdnacineIIIGrade.to_type_id(),

            Self::BistotIGrade.to_type_id(),
            Self::BistotIIGrade.to_type_id(),
            Self::BistotIIIGrade.to_type_id(),
            Self::BistotIVGrade.to_type_id(),
            Self::CompressedBistotIGrade.to_type_id(),
            Self::CompressedBistotIIGrade.to_type_id(),
            Self::CompressedBistotIIIGrade.to_type_id(),
            Self::CompressedBistotIVGrade.to_type_id(),

            Self::CrokiteIGrade.to_type_id(),
            Self::CrokiteIIGrade.to_type_id(),
            Self::CrokiteIIIGrade.to_type_id(),
            Self::CrokiteIVGrade.to_type_id(),
            Self::CompressedCrokiteIGrade.to_type_id(),
            Self::CompressedCrokiteIIGrade.to_type_id(),
            Self::CompressedCrokiteIIIGrade.to_type_id(),
            Self::CompressedCrokiteIVGrade.to_type_id(),

            Self::DarkOchreIGrade.to_type_id(),
            Self::OchreIIGrade.to_type_id(),
            Self::OchreIIIGrade.to_type_id(),
            Self::OchreIVGrade.to_type_id(),
            Self::CompressedDarkOchreIGrade.to_type_id(),
            Self::CompressedOchreIIGrade.to_type_id(),
            Self::CompressedOchreIIIGrade.to_type_id(),
            Self::CompressedOchreIVGrade.to_type_id(),

            Self::DuciniumIGrade.to_type_id(),
            Self::DuciniumIIGrade.to_type_id(),
            Self::DuciniumIIIGrade.to_type_id(),
            Self::DuciniumIVGrade.to_type_id(),
            Self::CompressedDuciniumIGrade.to_type_id(),
            Self::CompressedDuciniumIIGrade.to_type_id(),
            Self::CompressedDuciniumIIIGrade.to_type_id(),
            Self::CompressedDuciniumIVGrade.to_type_id(),

            Self::EifyriumIGrade.to_type_id(),
            Self::EifyriumIIGrade.to_type_id(),
            Self::EifyriumIIIGrade.to_type_id(),
            Self::EifyriumIVGrade.to_type_id(),
            Self::CompressedEifyriumIGrade.to_type_id(),
            Self::CompressedEifyriumIIGrade.to_type_id(),
            Self::CompressedEifyriumIIIGrade.to_type_id(),
            Self::CompressedEifyriumIVGrade.to_type_id(),

            Self::GneissIGrade.to_type_id(),
            Self::GneissIIGrade.to_type_id(),
            Self::GneissIIIGrade.to_type_id(),
            Self::GneissIVGrade.to_type_id(),
            Self::CompressedGneissIGrade.to_type_id(),
            Self::CompressedGneissIIGrade.to_type_id(),
            Self::CompressedGneissIIIGrade.to_type_id(),
            Self::CompressedGneissIVGrade.to_type_id(),

            Self::GriemeerIGrade.to_type_id(),
            Self::GriemeerIIGrade.to_type_id(),
            Self::GriemeerIIIGrade.to_type_id(),
            Self::GriemeerIVGrade.to_type_id(),
            Self::CompressedGriemeerIGrade.to_type_id(),
            Self::CompressedGriemeerIIGrade.to_type_id(),
            Self::CompressedGriemeerIIIGrade.to_type_id(),
            Self::CompressedGriemeerIVGrade.to_type_id(),

            Self::HedbergiteIGrade.to_type_id(),
            Self::HedbergiteIIGrade.to_type_id(),
            Self::HedbergiteIIIGrade.to_type_id(),
            Self::HedbergiteIVGrade.to_type_id(),
            Self::CompressedHedbergiteIGrade.to_type_id(),
            Self::CompressedHedbergiteIIGrade.to_type_id(),
            Self::CompressedHedbergiteIIIGrade.to_type_id(),
            Self::CompressedHedbergiteIVGrade.to_type_id(),

            Self::HemorphiteIGrade.to_type_id(),
            Self::HemorphiteIIGrade.to_type_id(),
            Self::HemorphiteIIIGrade.to_type_id(),
            Self::HemorphiteIVGrade.to_type_id(),
            Self::CompressedHemorphiteIGrade.to_type_id(),
            Self::CompressedHemorphiteIIGrade.to_type_id(),
            Self::CompressedHemorphiteIIIGrade.to_type_id(),
            Self::CompressedHemorphiteIVGrade.to_type_id(),

            Self::HezorimeIGrade.to_type_id(),
            Self::HezorimeIIGrade.to_type_id(),
            Self::HezorimeIIIGrade.to_type_id(),
            Self::HezorimeIVGrade.to_type_id(),
            Self::CompressedHezorimeIGrade.to_type_id(),
            Self::CompressedHezorimeIIGrade.to_type_id(),
            Self::CompressedHezorimeIIIGrade.to_type_id(),
            Self::CompressedHezorimeIVGrade.to_type_id(),

            Self::JaspetIGrade.to_type_id(),
            Self::JaspetIIGrade.to_type_id(),
            Self::JaspetIIIGrade.to_type_id(),
            Self::JaspetIVGrade.to_type_id(),
            Self::CompressedJaspetIGrade.to_type_id(),
            Self::CompressedJaspetIIGrade.to_type_id(),
            Self::CompressedJaspetIIIGrade.to_type_id(),
            Self::CompressedJaspetIVGrade.to_type_id(),

            Self::KerniteIGrade.to_type_id(),
            Self::KerniteIIGrade.to_type_id(),
            Self::KerniteIIIGrade.to_type_id(),
            Self::KerniteIVGrade.to_type_id(),
            Self::CompressedKerniteIGrade.to_type_id(),
            Self::CompressedKerniteIIGrade.to_type_id(),
            Self::CompressedKerniteIIIGrade.to_type_id(),
            Self::CompressedKerniteIVGrade.to_type_id(),

            Self::KylixiumIGrade.to_type_id(),
            Self::KylixiumIIGrade.to_type_id(),
            Self::KylixiumIIIGrade.to_type_id(),
            Self::KylixiumIVGrade.to_type_id(),
            Self::CompressedKylixiumIGrade.to_type_id(),
            Self::KylixiumIIGrade.to_type_id(),
            Self::CompressedKylixiumIIIGrade.to_type_id(),
            Self::CompressedKylixiumIVGrade.to_type_id(),

            Self::Mercoxit.to_type_id(),
            Self::MagmaMercoxit.to_type_id(),
            Self::VitreousMercoxit.to_type_id(),
            Self::CompressedMercoxit.to_type_id(),
            Self::CompressedMagmaMercoxit.to_type_id(),
            Self::CompressedVitreousMercoxit.to_type_id(),

            Self::Mordunium.to_type_id(),
            Self::PlumMordunium.to_type_id(),
            Self::PrizeMordunium.to_type_id(),
            Self::PlunderMordunium.to_type_id(),
            Self::CompressedMordunium.to_type_id(),
            Self::CompressedPlumMordunium.to_type_id(),
            Self::CompressedPrizeMordunium.to_type_id(),
            Self::CompressedPlunderMordunium.to_type_id(),

            Self::Nocxite.to_type_id(),
            Self::FragrantNocxite.to_type_id(),
            Self::IntoxicatingNocxite.to_type_id(),
            Self::AmbrosialNocxite.to_type_id(),
            Self::CompressedNocxite.to_type_id(),
            Self::CompressedFragrantNocxite.to_type_id(),
            Self::CompressedIntoxicatingNocxite.to_type_id(),
            Self::CompressedAmbrosialNocxite.to_type_id(),

            Self::Omber.to_type_id(),
            Self::SilveryOmber.to_type_id(),
            Self::GoldenOmber.to_type_id(),
            Self::PlatinoidOmber.to_type_id(),
            Self::CompressedOmber.to_type_id(),
            Self::CompressedSilveryOmber.to_type_id(),
            Self::CompressedGoldenOmber.to_type_id(),
            Self::CompressedPlatinoidOmber.to_type_id(),

            Self::Plagioclase.to_type_id(),
            Self::AzurePlagioclase.to_type_id(),
            Self::RichPlagioclase.to_type_id(),
            Self::SparklingPlagioclase.to_type_id(),
            Self::CompressedPlagioclase.to_type_id(),
            Self::CompressedAzurePlagioclase.to_type_id(),
            Self::CompressedRichPlagioclase.to_type_id(),
            Self::CompressedSparklingPlagioclase.to_type_id(),

            Self::Pyroxeres.to_type_id(),
            Self::SolidPyroxeres.to_type_id(),
            Self::ViscousPyroxeres.to_type_id(),
            Self::OpulentPyroxeres.to_type_id(),
            Self::CompressedPyroxeres.to_type_id(),
            Self::CompressedSolidPyroxeres.to_type_id(),
            Self::CompressedViscousPyroxeres.to_type_id(),
            Self::CompressedOpulentPyroxeres.to_type_id(),

            Self::Rakovene.to_type_id(),
            Self::RakoveneIIGrade.to_type_id(),
            Self::RakoveneIIIGrade.to_type_id(),
            Self::CompressedRakovene.to_type_id(),
            Self::CompressedRakoveneIIGrade.to_type_id(),
            Self::CompressedRakoveneIIIGrade.to_type_id(),

            Self::Scordite.to_type_id(),
            Self::CondensedScordite.to_type_id(),
            Self::MassiveScordite.to_type_id(),
            Self::GlossyScordite.to_type_id(),
            Self::CompressedScordite.to_type_id(),
            Self::CompressedCondensedScordite.to_type_id(),
            Self::CompressedMassiveScordite.to_type_id(),
            Self::CompressedGlossyScordite.to_type_id(),

            Self::Spodumain.to_type_id(),
            Self::BrightSpodumain.to_type_id(),
            Self::GleamingSpodumain.to_type_id(),
            Self::DazzlingSpodumain.to_type_id(),
            Self::CompressedSpodumain.to_type_id(),
            Self::CompressedBrightSpodumain.to_type_id(),
            Self::CompressedGleamingSpodumain.to_type_id(),
            Self::CompressedDazzlingSpodumain.to_type_id(),

            Self::Talassonite.to_type_id(),
            Self::TalassoniteIIGrade.to_type_id(),
            Self::TalassoniteIIIGrade.to_type_id(),
            Self::CompressedTalassonite.to_type_id(),
            Self::CompressedTalassoniteIIGrade.to_type_id(),
            Self::CompressedTalassoniteIIIGrade.to_type_id(),

            Self::Ueganite.to_type_id(),
            Self::FoggyUeganite.to_type_id(),
            Self::OvercastUeganite.to_type_id(),
            Self::StormyUeganite.to_type_id(),
            Self::CompressedUeganite.to_type_id(),
            Self::CompressedFoggyUeganite.to_type_id(),
            Self::CompressedOvercastUeganite.to_type_id(),
            Self::CompressedStormyUeganite.to_type_id(),

            Self::Veldspar.to_type_id(),
            Self::ConcentratedVeldspar.to_type_id(),
            Self::DenseVeldspar.to_type_id(),
            Self::StableVeldspar.to_type_id(),
            Self::CompressedVeldspar.to_type_id(),
            Self::CompressedConcentratedVeldspar.to_type_id(),
            Self::CompressedDenseVeldspar.to_type_id(),
            Self::CompressedStableVeldspar.to_type_id(),

            Self::Ytirium.to_type_id(),
            Self::BootlegYtirium.to_type_id(),
            Self::FirewaterYtirium.to_type_id(),
            Self::MoonshineYtirium.to_type_id(),
            Self::CompressedYtirium.to_type_id(),
            Self::CompressedBootlegYtirium.to_type_id(),
            Self::CompressedFirewaterYtirium.to_type_id(),
            Self::CompressedMoonshineYtirium.to_type_id(),

            Self::Tritanium.to_type_id(),
            Self::Pyerite.to_type_id(),
            Self::Mexallon.to_type_id(),
            Self::Isogen.to_type_id(),
            Self::Nocxium.to_type_id(),
            Self::Zydrine.to_type_id(),
            Self::Megacyte.to_type_id(),
            Self::Morphite.to_type_id(),

            Self::Bitumens.to_type_id(),
            Self::BrimfulBitumens.to_type_id(),
            Self::GlisteningBitumens.to_type_id(),
            Self::CompressedBitumens.to_type_id(),
            Self::CompressedBrimfulBitumens.to_type_id(),
            Self::CompressedGlisteningBitumens.to_type_id(),

            Self::Coesite.to_type_id(),
            Self::BrimfulCoesite.to_type_id(),
            Self::GlisteningCoesite.to_type_id(),
            Self::CompressedCoesite.to_type_id(),
            Self::CompressedBrimfulCoesite.to_type_id(),
            Self::CompressedGlisteningCoesite.to_type_id(),

            Self::Sylvite.to_type_id(),
            Self::BrimfulSylvite.to_type_id(),
            Self::GlisteningSylvite.to_type_id(),
            Self::CompressedSylvite.to_type_id(),
            Self::CompressedBrimfulSylvite.to_type_id(),
            Self::CompressedGlisteningSylvite.to_type_id(),

            Self::Zeolites.to_type_id(),
            Self::BrimfulZeolites.to_type_id(),
            Self::GlisteningZeolites.to_type_id(),
            Self::CompressedZeolites.to_type_id(),
            Self::CompressedBrimfulZeolites.to_type_id(),
            Self::CompressedGlisteningZeolites.to_type_id(),

            Self::Cobaltite.to_type_id(),
            Self::CopiousCobaltite.to_type_id(),
            Self::TwinklingCobaltite.to_type_id(),
            Self::CompressedCobaltite.to_type_id(),
            Self::CompressedCopiousCobaltite.to_type_id(),
            Self::CompressedTwinklingCobaltite.to_type_id(),

            Self::Euxenite.to_type_id(),
            Self::CopiousEuxenite.to_type_id(),
            Self::TwinklingEuxenite.to_type_id(),
            Self::CompressedEuxenite.to_type_id(),
            Self::CompressedCopiousEuxenite.to_type_id(),
            Self::CompressedTwinklingEuxenite.to_type_id(),

            Self::Scheelite.to_type_id(),
            Self::CopiousScheelite.to_type_id(),
            Self::TwinklingScheelite.to_type_id(),
            Self::CompressedScheelite.to_type_id(),
            Self::CompressedCopiousScheelite.to_type_id(),
            Self::CompressedTwinklingScheelite.to_type_id(),

            Self::Titanite.to_type_id(),
            Self::CopiousTitanite.to_type_id(),
            Self::TwinklingTitanite.to_type_id(),
            Self::CompressedTitanite.to_type_id(),
            Self::CompressedCopiousTitanite.to_type_id(),
            Self::CompressedTwinklingTitanite.to_type_id(),

            Self::Chromite.to_type_id(),
            Self::LavishChromite.to_type_id(),
            Self::ShimmeringChromite.to_type_id(),
            Self::CompressedChromite.to_type_id(),
            Self::CompressedLavishChromite.to_type_id(),
            Self::CompressedShimmeringChromite.to_type_id(),

            Self::Otavite.to_type_id(),
            Self::LavishOtavite.to_type_id(),
            Self::ShimmeringOtavite.to_type_id(),
            Self::CompressedOtavite.to_type_id(),
            Self::CompressedLavishOtavite.to_type_id(),
            Self::CompressedShimmeringOtavite.to_type_id(),

            Self::Sperrylite.to_type_id(),
            Self::LavishSperrylite.to_type_id(),
            Self::ShimmeringSperrylite.to_type_id(),
            Self::CompressedSperrylite.to_type_id(),
            Self::CompressedLavishSperrylite.to_type_id(),
            Self::CompressedShimmeringSperrylite.to_type_id(),

            Self::Vanadinite.to_type_id(),
            Self::LavishVanadinite.to_type_id(),
            Self::ShimmeringVanadinite.to_type_id(),
            Self::CompressedVanadinite.to_type_id(),
            Self::CompressedLavishVanadinite.to_type_id(),
            Self::CompressedShimmeringVanadinite.to_type_id(),

            Self::Carnotite.to_type_id(),
            Self::RepleteCarnotite.to_type_id(),
            Self::GlowingCarnotite.to_type_id(),
            Self::CompressedCarnotite.to_type_id(),
            Self::CompressedRepleteCarnotite.to_type_id(),
            Self::CompressedGlowingCarnotite.to_type_id(),

            Self::Cinnabar.to_type_id(),
            Self::RepleteCinnabar.to_type_id(),
            Self::GlowingCinnabar.to_type_id(),
            Self::CompressedCinnabar.to_type_id(),
            Self::CompressedRepleteCinnabar.to_type_id(),
            Self::CompressedGlowingCinnabar.to_type_id(),

            Self::Pollucite.to_type_id(),
            Self::RepletePollucite.to_type_id(),
            Self::GlowingPollucite.to_type_id(),
            Self::CompressedPollucite.to_type_id(),
            Self::CompressedRepletePollucite.to_type_id(),
            Self::CompressedGlowingPollucite.to_type_id(),

            Self::Zircon.to_type_id(),
            Self::RepleteZircon.to_type_id(),
            Self::GlowingZircon.to_type_id(),
            Self::CompressedZircon.to_type_id(),
            Self::CompressedRepleteZircon.to_type_id(),
            Self::CompressedGlowingZircon.to_type_id(),

            Self::Loparite.to_type_id(),
            Self::BountifulLoparite.to_type_id(),
            Self::ShiningLoparite.to_type_id(),
            Self::CompressedLoparite.to_type_id(),
            Self::CompressedBountifulLoparite.to_type_id(),
            Self::CompressedShiningLoparite.to_type_id(),

            Self::Monazite.to_type_id(),
            Self::BountifulMonazite.to_type_id(),
            Self::ShiningMonazite.to_type_id(),
            Self::CompressedMonazite.to_type_id(),
            Self::CompressedBountifulMonazite.to_type_id(),
            Self::CompressedShiningMonazite.to_type_id(),

            Self::Ytterbite.to_type_id(),
            Self::BountifulYtterbite.to_type_id(),
            Self::ShiningYtterbite.to_type_id(),
            Self::CompressedYtterbite.to_type_id(),
            Self::CompressedBountifulYtterbite.to_type_id(),
            Self::CompressedShiningYtterbite.to_type_id(),

            Self::Xenotime.to_type_id(),
            Self::BountifulXenotime.to_type_id(),
            Self::ShiningXenotime.to_type_id(),
            Self::CompressedXenotime.to_type_id(),
            Self::CompressedBountifulXenotime.to_type_id(),
            Self::CompressedShiningXenotime.to_type_id(),

            Self::AtmosphericGases.to_type_id(),
            Self::EvaporiteDeposits.to_type_id(),
            Self::Hydrocarbons.to_type_id(),
            Self::Silicates.to_type_id(),
            Self::Cobalt.to_type_id(),
            Self::Scandium.to_type_id(),
            Self::Titanium.to_type_id(),
            Self::Tungsten.to_type_id(),
            Self::Chromium.to_type_id(),
            Self::Cadmium.to_type_id(),
            Self::Platinum.to_type_id(),
            Self::Vanadium.to_type_id(),
            Self::Caesium.to_type_id(),
            Self::Hafnium.to_type_id(),
            Self::Mercury.to_type_id(),
            Self::Technetium.to_type_id(),
            Self::Promethium.to_type_id(),
            Self::Neodymium.to_type_id(),
            Self::Dysprosium.to_type_id(),
            Self::Thulium.to_type_id(),

            Self::BlueIce.to_type_id(),
            Self::BlueIceIVGrade.to_type_id(),
            Self::CompressedBlueIce.to_type_id(),
            Self::CompressedBlueIceIVGrade.to_type_id(),
            Self::IcicleIIGrade.to_type_id(),
            Self::IcicleIVGradeIIGrade.to_type_id(),
            Self::CompressedIcicleIIGrade.to_type_id(),
            Self::CompressedIcicleIVGradeIIGrade.to_type_id(),
            Self::GlacialMass.to_type_id(),
            Self::GlacialMassIVGrade.to_type_id(),
            Self::CompressedGlacialMass.to_type_id(),
            Self::CompressedGlacialMassIVGrade.to_type_id(),
            Self::WhiteGlaze.to_type_id(),
            Self::WhiteGlazeIVGrade.to_type_id(),
            Self::CompressedWhiteGlaze.to_type_id(),
            Self::CompressedWhiteGlazeIVGrade.to_type_id(),
            Self::DarkGlitter.to_type_id(),
            Self::CompressedDarkGlitter.to_type_id(),
            Self::Gelidus.to_type_id(),
            Self::CompressedGelidus.to_type_id(),
            Self::GlareCrust.to_type_id(),
            Self::CompressedGlareCrust.to_type_id(),
            Self::Krystallos.to_type_id(),
            Self::CompressedKrystallos.to_type_id(),
        ]
    }

    pub fn to_type_id(&self) -> i32 {
        match self {
            Self::ArkonorIGrade                                   => 22,
            Self::ArkonorIIGrade                            => 17425,
            Self::ArkonorIIIGrade                              => 17426,
            Self::ArkonorIVGrade                           => 46678,
            Self::CompressedArkonorIGrade                         => 62568,
            Self::CompressedArkonorIIGrade                  => 62569,
            Self::CompressedArkonorIIIGrade                    => 62570,
            Self::CompressedArkonorIVGrade                 => 62571,
            Self::BatchCompressedArkonorIGrade                    => 28367,
            Self::BatchCompressedArkonorIIGrade             => 28385,
            Self::BatchCompressedArkonorIIIGrade               => 28387,
            Self::BatchCompressedArkonorIVGrade            => 46691,

            Self::BezdnacineIGrade                                => 52316,
            Self::BezdnacineIIGrade                         => 56627,
            Self::BezdnacineIIIGrade                           => 56628,
            Self::CompressedBezdnacineIGrade                      => 62576,
            Self::CompressedBezdnacineIIGrade               => 62577,
            Self::CompressedBezdnacineIIIGrade                 => 62578,

            Self::BistotIGrade                                    => 1223,
            Self::BistotIIGrade                           => 17428,
            Self::BistotIIIGrade                          => 17429,
            Self::BistotIVGrade                               => 46676,
            Self::CompressedBistotIGrade                          => 62564,
            Self::CompressedBistotIIGrade                 => 62565,
            Self::CompressedBistotIIIGrade                => 62566,
            Self::CompressedBistotIVGrade                     => 62567,
            Self::BatchCompressedBistotIGrade                     => 28388,
            Self::BatchCompressedBistotIIGrade            => 28390,
            Self::BatchCompressedBistotIIIGrade           => 28389,
            Self::BatchCompressedBistotIVGrade                => 46692,

            Self::CrokiteIGrade                                   => 1225,
            Self::CrokiteIIGrade                              => 17432,
            Self::CrokiteIIIGrade                        => 17433,
            Self::CrokiteIVGrade                           => 46677,
            Self::CompressedCrokiteIGrade                         => 62560,
            Self::CompressedCrokiteIIGrade                    => 62561,
            Self::CompressedCrokiteIIIGrade              => 62562,
            Self::CompressedCrokiteIVGrade                 => 62563,
            Self::BatchCompressedCrokiteIGrade                    => 28391,
            Self::BatchCompressedCrokiteIIGrade               => 28392,
            Self::BatchCompressedCrokiteIIIGrade         => 28393,
            Self::BatchCompressedCrokiteIVGrade            => 46693,

            Self::DarkOchreIGrade                                 => 1232,
            Self::OchreIIGrade                                 => 17436,
            Self::OchreIIIGrade                             => 17437,
            Self::OchreIVGrade                                  => 46675,
            Self::CompressedDarkOchreIGrade                       => 62556,
            Self::CompressedOchreIIGrade                       => 62557,
            Self::CompressedOchreIIIGrade                   => 62558,
            Self::CompressedOchreIVGrade                        => 62559,
            Self::BatchCompressedDarkOchreIGrade                  => 28394,
            Self::BatchCompressedOchreIIGrade                  => 28396,
            Self::BatchCompressedOchreIIIGrade              => 28395,
            Self::BatchCompressedOchreIVGrade                   => 46694,

            Self::DuciniumIGrade                                  => 74533,
            Self::DuciniumIIGrade                             => 74534,
            Self::DuciniumIIIGrade                             => 74535,
            Self::DuciniumIVGrade                          => 74536,
            Self::CompressedDuciniumIGrade                        => 75287,
            Self::CompressedDuciniumIIGrade                   => 75288,
            Self::CompressedDuciniumIIIGrade                   => 75289,
            Self::CompressedDuciniumIVGrade                => 75290,

            Self::EifyriumIGrade                                  => 74529,
            Self::EifyriumIIGrade                             => 74530,
            Self::EifyriumIIIGrade                           => 74531,
            Self::EifyriumIVGrade                         => 74532,
            Self::CompressedEifyriumIGrade                        => 75283,
            Self::CompressedEifyriumIIGrade                   => 75284,
            Self::CompressedEifyriumIIIGrade                 => 75285,
            Self::CompressedEifyriumIVGrade               => 75286,

            Self::GneissIGrade                                    => 1229,
            Self::GneissIIGrade                          => 17865,
            Self::GneissIIIGrade                           => 17866,
            Self::GneissIVGrade                           => 46679,
            Self::CompressedGneissIGrade                          => 62552,
            Self::CompressedGneissIIGrade                => 62553,
            Self::CompressedGneissIIIGrade                 => 62554,
            Self::CompressedGneissIVGrade                 => 62555,
            Self::BatchCompressedGneissIGrade                     => 28397,
            Self::BatchCompressedGneissIIGrade           => 28398,
            Self::BatchCompressedGneissIIIGrade            => 28399,
            Self::BatchCompressedGneissIVGrade            => 46695,

            Self::GriemeerIGrade                                  => 81975,
            Self::GriemeerIIGrade                             => 81976,
            Self::GriemeerIIIGrade                              => 81977,
            Self::GriemeerIVGrade                            => 81978,
            Self::CompressedGriemeerIGrade                        => 82316,
            Self::CompressedGriemeerIIGrade                   => 82317,
            Self::CompressedGriemeerIIIGrade                    => 82318,
            Self::CompressedGriemeerIVGrade                  => 82319,

            Self::HedbergiteIGrade                                => 21,
            Self::HedbergiteIIGrade                          => 17440,
            Self::HedbergiteIIIGrade                          => 17441,
            Self::HedbergiteIVGrade                        => 46680,
            Self::CompressedHedbergiteIGrade                      => 62548,
            Self::CompressedHedbergiteIIGrade                => 62549,
            Self::CompressedHedbergiteIIIGrade                => 62550,
            Self::CompressedHedbergiteIVGrade              => 62551,
            Self::BatchCompressedHedbergiteIGrade                 => 28401,
            Self::BatchCompressedHedbergiteIIGrade           => 28402,
            Self::BatchCompressedHedbergiteIIIGrade           => 28400,
            Self::BatchCompressedHedbergiteIVGrade         => 46696,

            Self::HemorphiteIGrade                                => 1231,
            Self::HemorphiteIIGrade                           => 17444,
            Self::HemorphiteIIIGrade                         => 17445,
            Self::HemorphiteIVGrade                   => 46681,
            Self::CompressedHemorphiteIGrade                      => 62544,
            Self::CompressedHemorphiteIIGrade                 => 62545,
            Self::CompressedHemorphiteIIIGrade               => 62546,
            Self::CompressedHemorphiteIVGrade         => 62547,
            Self::BatchCompressedHemorphiteIGrade                 => 28403,
            Self::BatchCompressedHemorphiteIIGrade            => 28405,
            Self::BatchCompressedHemorphiteIIIGrade          => 28404,
            Self::BatchCompressedHemorphiteIVGrade    => 46697,

            Self::HezorimeIGrade                                  => 82163,
            Self::HezorimeIIGrade                              => 82164,
            Self::HezorimeIIIGrade                          => 82165,
            Self::HezorimeIVGrade                             => 82166,
            Self::CompressedHezorimeIGrade                        => 82312,
            Self::CompressedHezorimeIIGrade                    => 82313,
            Self::CompressedHezorimeIIIGrade                => 82314,
            Self::CompressedHezorimeIVGrade                   => 82315,

            Self::JaspetIGrade                                    => 1226,
            Self::JaspetIIGrade                                => 17448,
            Self::JaspetIIIGrade                            => 17449,
            Self::JaspetIVGrade                          => 46682,
            Self::CompressedJaspetIGrade                          => 62540,
            Self::CompressedJaspetIIGrade                      => 62541,
            Self::CompressedJaspetIIIGrade                  => 62542,
            Self::CompressedJaspetIVGrade                => 62543,
            Self::BatchCompressedJaspetIGrade                     => 28406,
            Self::BatchCompressedJaspetIIGrade                 => 28408,
            Self::BatchCompressedJaspetIIIGrade             => 28407,
            Self::BatchCompressedJaspetIVGrade           => 46698,

            Self::KerniteIGrade                                   => 20,
            Self::KerniteIIGrade                           => 17452,
            Self::KerniteIIIGrade                              => 17453,
            Self::KerniteIVGrade                        => 46683,
            Self::CompressedKerniteIGrade                         => 62536,
            Self::CompressedKerniteIIGrade                 => 62537,
            Self::CompressedKerniteIIIGrade                    => 62538,
            Self::CompressedKerniteIVGrade              => 62539,
            Self::BatchCompressedKerniteIGrade                    => 28410,
            Self::BatchCompressedKerniteIIGrade            => 28411,
            Self::BatchCompressedKerniteIIIGrade               => 28409,
            Self::BatchCompressedKerniteIVGrade         => 46699,

            Self::KylixiumIGrade                                  => 81900,
            Self::KylixiumIIGrade                            => 81901,
            Self::KylixiumIIIGrade                             => 81902,
            Self::KylixiumIVGrade                             => 81903,
            Self::CompressedKylixiumIGrade                        => 82300,
            Self::CompressedKylixiumIIGrade                  => 82301,
            Self::CompressedKylixiumIIIGrade                   => 82302,
            Self::CompressedKylixiumIVGrade                   => 82303,

            Self::Mercoxit                                  => 11396,
            Self::MagmaMercoxit                             => 17869,
            Self::VitreousMercoxit                          => 17870,
            Self::CompressedMercoxit                        => 62586,
            Self::CompressedMagmaMercoxit                   => 62587,
            Self::CompressedVitreousMercoxit                => 62588,
            Self::BatchCompressedMercoxit                   => 28413,
            Self::BatchCompressedMagmaMercoxit              => 28412,
            Self::BatchCompressedVitreousMercoxit           => 28414,

            Self::Mordunium                                 => 74521,
            Self::PlumMordunium                             => 74522,
            Self::PrizeMordunium                            => 74523,
            Self::PlunderMordunium                          => 74524,
            Self::CompressedMordunium                       => 75275,
            Self::CompressedPlumMordunium                   => 75276,
            Self::CompressedPrizeMordunium                  => 75277,
            Self::CompressedPlunderMordunium                => 75278,

            Self::Nocxite                                   => 82016,
            Self::FragrantNocxite                           => 82017,
            Self::IntoxicatingNocxite                       => 82018,
            Self::AmbrosialNocxite                          => 82019,
            Self::CompressedNocxite                         => 82304,
            Self::CompressedFragrantNocxite                 => 82305,
            Self::CompressedIntoxicatingNocxite             => 82306,
            Self::CompressedAmbrosialNocxite                => 82307,

            Self::Omber                                     => 1227,
            Self::SilveryOmber                              => 17867,
            Self::GoldenOmber                               => 17868,
            Self::PlatinoidOmber                            => 46684,
            Self::CompressedOmber                           => 62532,
            Self::CompressedSilveryOmber                    => 62533,
            Self::CompressedGoldenOmber                     => 62534,
            Self::CompressedPlatinoidOmber                  => 62535,
            Self::BatchCompressedOmber                      => 28416,
            Self::BatchCompressedSilveryOmber               => 28417,
            Self::BatchCompressedGoldenOmber                => 28415,
            Self::BatchCompressedPlatinoidOmber             => 46700,

            Self::Plagioclase                               => 18,
            Self::AzurePlagioclase                          => 17455,
            Self::RichPlagioclase                           => 17456,
            Self::SparklingPlagioclase                      => 46685,
            Self::CompressedPlagioclase                     => 62528,
            Self::CompressedAzurePlagioclase                => 62529,
            Self::CompressedRichPlagioclase                 => 62530,
            Self::CompressedSparklingPlagioclase            => 62531,
            Self::BatchCompressedPlagioclase                => 28422,
            Self::BatchCompressedAzurePlagioclase           => 28421,
            Self::BatchCompressedRichPlagioclase            => 28423,
            Self::BatchCompressedSparklingPlagioclase       => 46701,

            Self::Pyroxeres                                 => 1224,
            Self::SolidPyroxeres                            => 17459,
            Self::ViscousPyroxeres                          => 17460,
            Self::OpulentPyroxeres                          => 46686,
            Self::CompressedPyroxeres                       => 62524,
            Self::CompressedSolidPyroxeres                  => 62525,
            Self::CompressedViscousPyroxeres                => 62526,
            Self::CompressedOpulentPyroxeres                => 62527,
            Self::BatchCompressedPyroxeres                  => 28424,
            Self::BatchCompressedSolidPyroxeres             => 28425,
            Self::BatchCompressedViscousPyroxeres           => 28426,
            Self::BatchCompressedOpulentPyroxeres           => 46702,

            Self::Rakovene                                  => 52315,
            Self::RakoveneIIGrade                           => 56629,
            Self::RakoveneIIIGrade                             => 56630,
            Self::CompressedRakovene                        => 62579,
            Self::CompressedRakoveneIIGrade                 => 62580,
            Self::CompressedRakoveneIIIGrade                   => 62581,

            Self::Scordite                                  => 1228,
            Self::CondensedScordite                         => 17463,
            Self::MassiveScordite                           => 17464,
            Self::GlossyScordite                            => 46687,
            Self::CompressedScordite                        => 62520,
            Self::CompressedCondensedScordite               => 62521,
            Self::CompressedMassiveScordite                 => 62522,
            Self::CompressedGlossyScordite                  => 62523,
            Self::BatchCompressedScordite                   => 28429,
            Self::BatchCompressedCondensedScordite          => 28427,
            Self::BatchCompressedMassiveScordite            => 28428,
            Self::BatchCompressedGlossyScordite             => 46703,

            Self::Spodumain                                 => 19,
            Self::BrightSpodumain                           => 17466,
            Self::GleamingSpodumain                         => 17467,
            Self::DazzlingSpodumain                         => 46688,
            Self::CompressedSpodumain                       => 62572,
            Self::CompressedBrightSpodumain                 => 62573,
            Self::CompressedGleamingSpodumain               => 62574,
            Self::CompressedDazzlingSpodumain               => 62575,
            Self::BatchCompressedSpodumain                  => 28420,
            Self::BatchCompressedBrightSpodumain            => 28418,
            Self::BatchCompressedGleamingSpodumain          => 28419,
            Self::BatchCompressedDazzlingSpodumain          => 46704,

            Self::Talassonite                               => 52306,
            Self::TalassoniteIIGrade                        => 56625,
            Self::TalassoniteIIIGrade                          => 56626,
            Self::CompressedTalassonite                     => 62582,
            Self::CompressedTalassoniteIIGrade              => 62583,
            Self::CompressedTalassoniteIIIGrade                => 62584,

            Self::Ueganite                                  => 82205,
            Self::FoggyUeganite                             => 82206,
            Self::OvercastUeganite                          => 82207,
            Self::StormyUeganite                            => 82208,
            Self::CompressedUeganite                        => 82308,
            Self::CompressedFoggyUeganite                   => 82309,
            Self::CompressedOvercastUeganite                => 82310,
            Self::CompressedStormyUeganite                  => 82311,

            Self::Veldspar                                  => 1230,
            Self::ConcentratedVeldspar                      => 17470,
            Self::DenseVeldspar                             => 17471,
            Self::StableVeldspar                            => 46689,
            Self::CompressedVeldspar                        => 62516,
            Self::CompressedConcentratedVeldspar            => 62517,
            Self::CompressedDenseVeldspar                   => 62518,
            Self::CompressedStableVeldspar                  => 62519,
            Self::BatchCompressedVeldspar                   => 28432,
            Self::BatchCompressedConcentratedVeldspar       => 28430,
            Self::BatchCompressedDenseVeldspar              => 28431,
            Self::BatchCompressedStableVeldspar             => 46705,

            Self::Ytirium                                   => 74525,
            Self::BootlegYtirium                            => 74526,
            Self::FirewaterYtirium                          => 74527,
            Self::MoonshineYtirium                          => 74528,
            Self::CompressedYtirium                         => 75279,
            Self::CompressedBootlegYtirium                  => 75280,
            Self::CompressedFirewaterYtirium                => 75281,
            Self::CompressedMoonshineYtirium                => 75282,

            // MoonGoo
            Self::Bitumens                                  => 45492,
            Self::BrimfulBitumens                           => 46284,
            Self::GlisteningBitumens                        => 46285,
            Self::CompressedBitumens                        => 62454,
            Self::CompressedBrimfulBitumens                 => 62455,
            Self::CompressedGlisteningBitumens              => 62456,

            Self::Coesite                                   => 45493,
            Self::BrimfulCoesite                            => 46286,
            Self::GlisteningCoesite                         => 46287,
            Self::CompressedCoesite                         => 62457,
            Self::CompressedBrimfulCoesite                  => 62458,
            Self::CompressedGlisteningCoesite               => 62459,

            Self::Sylvite                                   => 45491,
            Self::BrimfulSylvite                            => 46282,
            Self::GlisteningSylvite                         => 46283,
            Self::CompressedSylvite                         => 62460,
            Self::CompressedBrimfulSylvite                  => 62461,
            Self::CompressedGlisteningSylvite               => 62466,

            Self::Zeolites                                  => 45490,
            Self::BrimfulZeolites                           => 46280,
            Self::GlisteningZeolites                        => 46281,
            Self::CompressedZeolites                        => 62463,
            Self::CompressedBrimfulZeolites                 => 62464,
            Self::CompressedGlisteningZeolites              => 62467,

            Self::Cobaltite                                 => 45494,
            Self::CopiousCobaltite                          => 46288,
            Self::TwinklingCobaltite                        => 46289,
            Self::CompressedCobaltite                       => 62474,
            Self::CompressedCopiousCobaltite                => 62475,
            Self::CompressedTwinklingCobaltite              => 62476,

            Self::Euxenite                                  => 45495,
            Self::CopiousEuxenite                           => 46290,
            Self::TwinklingEuxenite                         => 46291,
            Self::CompressedEuxenite                        => 62471,
            Self::CompressedCopiousEuxenite                 => 62472,
            Self::CompressedTwinklingEuxenite               => 62473,

            Self::Scheelite                                 => 45497,
            Self::CopiousScheelite                          => 46294,
            Self::TwinklingScheelite                        => 46295,
            Self::CompressedScheelite                       => 62468,
            Self::CompressedCopiousScheelite                => 62469,
            Self::CompressedTwinklingScheelite              => 62470,

            Self::Titanite                                  => 45496,
            Self::CopiousTitanite                           => 46292,
            Self::TwinklingTitanite                         => 46293,
            Self::CompressedTitanite                        => 62477,
            Self::CompressedCopiousTitanite                 => 62478,
            Self::CompressedTwinklingTitanite               => 62479,

            Self::Chromite                                  => 45501,
            Self::LavishChromite                            => 46302,
            Self::ShimmeringChromite                        => 46303,
            Self::CompressedChromite                        => 62480,
            Self::CompressedLavishChromite                  => 62481,
            Self::CompressedShimmeringChromite              => 62482,

            Self::Otavite                                   => 45498,
            Self::LavishOtavite                             => 46296,
            Self::ShimmeringOtavite                         => 46297,
            Self::CompressedOtavite                         => 62483,
            Self::CompressedLavishOtavite                   => 62484,
            Self::CompressedShimmeringOtavite               => 62485,

            Self::Sperrylite                                => 45499,
            Self::LavishSperrylite                          => 46298,
            Self::ShimmeringSperrylite                      => 46299,
            Self::CompressedSperrylite                      => 62486,
            Self::CompressedLavishSperrylite                => 62487,
            Self::CompressedShimmeringSperrylite            => 62488,

            Self::Vanadinite                                => 45500,
            Self::LavishVanadinite                          => 46300,
            Self::ShimmeringVanadinite                      => 46301,
            Self::CompressedVanadinite                      => 62489,
            Self::CompressedLavishVanadinite                => 62490,
            Self::CompressedShimmeringVanadinite            => 62491,

            Self::Carnotite                                 => 45502,
            Self::RepleteCarnotite                          => 46304,
            Self::GlowingCarnotite                          => 46305,
            Self::CompressedCarnotite                       => 62492,
            Self::CompressedRepleteCarnotite                => 62493,
            Self::CompressedGlowingCarnotite                => 62494,

            Self::Cinnabar                                  => 45506,
            Self::RepleteCinnabar                           => 46310,
            Self::GlowingCinnabar                           => 46311,
            Self::CompressedCinnabar                        => 62495,
            Self::CompressedRepleteCinnabar                 => 62496,
            Self::CompressedGlowingCinnabar                 => 62497,

            Self::Pollucite                                 => 45504,
            Self::RepletePollucite                          => 46308,
            Self::GlowingPollucite                          => 46309,
            Self::CompressedPollucite                       => 62498,
            Self::CompressedRepletePollucite                => 62499,
            Self::CompressedGlowingPollucite                => 62500,

            Self::Zircon                                    => 45503,
            Self::RepleteZircon                             => 46306,
            Self::GlowingZircon                             => 46307,
            Self::CompressedZircon                          => 62501,
            Self::CompressedRepleteZircon                   => 62502,
            Self::CompressedGlowingZircon                   => 62503,

            Self::Loparite                                  => 45512,
            Self::BountifulLoparite                         => 46316,
            Self::ShiningLoparite                           => 46317,
            Self::CompressedLoparite                        => 62504,
            Self::CompressedBountifulLoparite               => 62505,
            Self::CompressedShiningLoparite                 => 62506,

            Self::Monazite                                  => 45511,
            Self::BountifulMonazite                         => 46314,
            Self::ShiningMonazite                           => 46315,
            Self::CompressedMonazite                        => 62507,
            Self::CompressedBountifulMonazite               => 62508,
            Self::CompressedShiningMonazite                 => 62509,

            Self::Xenotime                                  => 45510,
            Self::BountifulXenotime                         => 46312,
            Self::ShiningXenotime                           => 46313,
            Self::CompressedXenotime                        => 62510,
            Self::CompressedBountifulXenotime               => 62511,
            Self::CompressedShiningXenotime                 => 62512,

            Self::Ytterbite                                 => 45513,
            Self::BountifulYtterbite                        => 46318,
            Self::ShiningYtterbite                          => 46319,
            Self::CompressedYtterbite                       => 62513,
            Self::CompressedBountifulYtterbite              => 62514,
            Self::CompressedShiningYtterbite                => 62515,

            Self::Tritanium                                 => 34,
            Self::Pyerite                                   => 35,
            Self::Mexallon                                  => 36,
            Self::Isogen                                    => 37,
            Self::Nocxium                                   => 38,
            Self::Zydrine                                   => 39,
            Self::Megacyte                                  => 40,
            Self::Morphite                                  => 11399,

            Self::AtmosphericGases                          => 16634,
            Self::EvaporiteDeposits                         => 16635,
            Self::Hydrocarbons                              => 16633,
            Self::Silicates                                 => 16636,
            Self::Cobalt                                    => 16640,
            Self::Scandium                                  => 16639,
            Self::Titanium                                  => 16638,
            Self::Tungsten                                  => 16637,
            Self::Chromium                                  => 16641,
            Self::Cadmium                                   => 16643,
            Self::Platinum                                  => 16644,
            Self::Vanadium                                  => 16642,
            Self::Caesium                                   => 16647,
            Self::Hafnium                                   => 16648,
            Self::Mercury                                   => 16646,
            Self::Technetium                                => 16649,
            Self::Promethium                                => 16652,
            Self::Neodymium                                 => 16651,
            Self::Dysprosium                                => 16650,
            Self::Thulium                                   => 16653,

            Self::BlueIce                                   => 16264,
            Self::BlueIceIVGrade                            => 17975,
            Self::CompressedBlueIce                         => 28433,
            Self::CompressedBlueIceIVGrade                  => 28443,
            Self::IcicleIIGrade                               => 16262,
            Self::IcicleIVGradeIIGrade                        => 17978,
            Self::CompressedIcicleIIGrade                     => 28434,
            Self::CompressedIcicleIVGradeIIGrade              => 28436,
            Self::GlacialMass                               => 16263,
            Self::GlacialMassIVGrade                        => 17977,
            Self::CompressedGlacialMass                     => 28438,
            Self::CompressedGlacialMassIVGrade              => 28442,
            Self::WhiteGlaze                                => 16265,
            Self::WhiteGlazeIVGrade                         => 17976,
            Self::CompressedWhiteGlaze                      => 28444,
            Self::CompressedWhiteGlazeIVGrade               => 28441,
            Self::DarkGlitter                               => 16267,
            Self::CompressedDarkGlitter                     => 28435,
            Self::Gelidus                                   => 16268,
            Self::CompressedGelidus                         => 28437,
            Self::GlareCrust                                => 16266,
            Self::CompressedGlareCrust                      => 28439,
            Self::Krystallos                                => 16269,
            Self::CompressedKrystallos                      => 28440,
        }
    }

    pub fn from_type_id(value: TypeId) -> Self {
        match *value {
            22    => Self::ArkonorIGrade,
            17425 => Self::ArkonorIIGrade,
            17426 => Self::ArkonorIIIGrade,
            46678 => Self::ArkonorIVGrade,
            62568 => Self::CompressedArkonorIGrade,
            62569 => Self::CompressedArkonorIIGrade,
            62570 => Self::CompressedArkonorIIIGrade,
            62571 => Self::CompressedArkonorIVGrade,
            28367 => Self::BatchCompressedArkonorIGrade,
            28385 => Self::BatchCompressedArkonorIIGrade,
            28387 => Self::BatchCompressedArkonorIIIGrade,
            46691 => Self::BatchCompressedArkonorIVGrade,

            52316 => Self::BezdnacineIGrade,
            56627 => Self::BezdnacineIIGrade,
            56628 => Self::BezdnacineIIIGrade,
            62576 => Self::CompressedBezdnacineIGrade,
            62577 => Self::CompressedBezdnacineIIGrade,
            62578 => Self::CompressedBezdnacineIIIGrade,

            1223  => Self::BistotIGrade,
            17428 => Self::BistotIIGrade,
            17429 => Self::BistotIIIGrade,
            46676 => Self::BistotIVGrade,
            62564 => Self::CompressedBistotIGrade,
            62565 => Self::CompressedBistotIIGrade,
            62566 => Self::CompressedBistotIIIGrade,
            62567 => Self::CompressedBistotIVGrade,
            28388 => Self::BatchCompressedBistotIGrade,
            28390 => Self::BatchCompressedBistotIIGrade,
            28389 => Self::BatchCompressedBistotIIIGrade,
            46692 => Self::BatchCompressedBistotIVGrade,

            1225  => Self::CrokiteIGrade,
            17432 => Self::CrokiteIIGrade,
            17433 => Self::CrokiteIIIGrade,
            46677 => Self::CrokiteIVGrade,
            62560 => Self::CompressedCrokiteIGrade,
            62561 => Self::CompressedCrokiteIIGrade,
            62562 => Self::CompressedCrokiteIIIGrade,
            62563 => Self::CompressedCrokiteIVGrade,
            28391 => Self::BatchCompressedCrokiteIGrade,
            28392 => Self::BatchCompressedCrokiteIIGrade,
            28393 => Self::BatchCompressedCrokiteIIIGrade,
            46693 => Self::BatchCompressedCrokiteIVGrade,

            1232  => Self::DarkOchreIGrade,
            17436 => Self::OchreIIGrade,
            17437 => Self::OchreIIIGrade,
            46675 => Self::OchreIVGrade,
            62556 => Self::CompressedDarkOchreIGrade,
            62557 => Self::CompressedOchreIIGrade,
            62558 => Self::CompressedOchreIIIGrade,
            62559 => Self::CompressedOchreIVGrade,
            28394 => Self::BatchCompressedDarkOchreIGrade,
            28396 => Self::BatchCompressedOchreIIGrade,
            28395 => Self::BatchCompressedOchreIIIGrade,
            46694 => Self::BatchCompressedOchreIVGrade,

            74533 => Self::DuciniumIGrade,
            74534 => Self::DuciniumIIGrade,
            74535 => Self::DuciniumIIIGrade,
            74536 => Self::DuciniumIVGrade,
            75287 => Self::CompressedDuciniumIGrade,
            75288 => Self::CompressedDuciniumIIGrade,
            75289 => Self::CompressedDuciniumIIIGrade,
            75290 => Self::CompressedDuciniumIVGrade,

            74529 => Self::EifyriumIGrade,
            74530 => Self::EifyriumIIGrade,
            74531 => Self::EifyriumIIIGrade,
            74532 => Self::EifyriumIVGrade,
            75283 => Self::CompressedEifyriumIGrade,
            75284 => Self::CompressedEifyriumIIGrade,
            75285 => Self::CompressedEifyriumIIIGrade,
            75286 => Self::CompressedEifyriumIVGrade,

            1229  => Self::GneissIGrade,
            17865 => Self::GneissIIGrade,
            17866 => Self::GneissIIIGrade,
            46679 => Self::GneissIVGrade,
            62552 => Self::CompressedGneissIGrade,
            62553 => Self::CompressedGneissIIGrade,
            62554 => Self::CompressedGneissIIIGrade,
            62555 => Self::CompressedGneissIVGrade,
            28397 => Self::BatchCompressedGneissIGrade,
            28398 => Self::BatchCompressedGneissIIGrade,
            28399 => Self::BatchCompressedGneissIIIGrade,
            46695 => Self::BatchCompressedGneissIVGrade,


            81975 => Self::GriemeerIGrade,
            81976 => Self::GriemeerIIGrade,
            81977 => Self::GriemeerIIIGrade,
            81978 => Self::GriemeerIVGrade,
            82316 => Self::CompressedGriemeerIGrade,
            82317 => Self::CompressedGriemeerIIGrade,
            82318 => Self::CompressedGriemeerIIIGrade,
            82319 => Self::CompressedGriemeerIVGrade,

            21    => Self::HedbergiteIGrade,
            17440 => Self::HedbergiteIIGrade,
            17441 => Self::HedbergiteIIIGrade,
            46680 => Self::HedbergiteIVGrade,
            62548 => Self::CompressedHedbergiteIGrade,
            62549 => Self::CompressedHedbergiteIIGrade,
            62550 => Self::CompressedHedbergiteIIIGrade,
            62551 => Self::CompressedHedbergiteIVGrade,
            28401 => Self::BatchCompressedHedbergiteIGrade,
            28402 => Self::BatchCompressedHedbergiteIIGrade,
            28400 => Self::BatchCompressedHedbergiteIIIGrade,
            46696 => Self::BatchCompressedHedbergiteIVGrade,

            1231  => Self::HemorphiteIGrade,
            17444 => Self::HemorphiteIIGrade,
            17445 => Self::HemorphiteIIIGrade,
            46681 => Self::HemorphiteIVGrade,
            62544 => Self::CompressedHemorphiteIGrade,
            62545 => Self::CompressedHemorphiteIIGrade,
            62546 => Self::CompressedHemorphiteIIIGrade,
            62547 => Self::CompressedHemorphiteIVGrade,
            28403 => Self::BatchCompressedHemorphiteIGrade,
            28405 => Self::BatchCompressedHemorphiteIIGrade,
            28404 => Self::BatchCompressedHemorphiteIIIGrade,
            46697 => Self::BatchCompressedHemorphiteIVGrade,

            82163 => Self::HezorimeIGrade,
            82164 => Self::HezorimeIIGrade,
            82165 => Self::HezorimeIIIGrade,
            82166 => Self::HezorimeIVGrade,
            82312 => Self::CompressedHezorimeIGrade,
            82313 => Self::CompressedHezorimeIIGrade,
            82314 => Self::CompressedHezorimeIIIGrade,
            82315 => Self::CompressedHezorimeIVGrade,

            1226  => Self::JaspetIGrade,
            17448 => Self::JaspetIIGrade,
            17449 => Self::JaspetIIIGrade,
            46682 => Self::JaspetIVGrade,
            62540 => Self::CompressedJaspetIGrade,
            62541 => Self::CompressedJaspetIIGrade,
            62542 => Self::CompressedJaspetIIIGrade,
            62543 => Self::CompressedJaspetIVGrade,
            28406 => Self::BatchCompressedJaspetIGrade,
            28408 => Self::BatchCompressedJaspetIIGrade,
            28407 => Self::BatchCompressedJaspetIIIGrade,
            46698 => Self::BatchCompressedJaspetIVGrade,


            20    => Self::KerniteIGrade,
            17452 => Self::KerniteIIGrade,
            17453 => Self::KerniteIIIGrade,
            46683 => Self::KerniteIVGrade,
            62536 => Self::CompressedKerniteIGrade,
            62537 => Self::CompressedKerniteIIGrade,
            62538 => Self::CompressedKerniteIIIGrade,
            62539 => Self::CompressedKerniteIVGrade,
            28410 => Self::BatchCompressedKerniteIGrade,
            28411 => Self::BatchCompressedKerniteIIGrade,
            28409 => Self::BatchCompressedKerniteIIIGrade,
            46699 => Self::BatchCompressedKerniteIVGrade,

            81900 => Self::KylixiumIGrade,
            81901 => Self::KylixiumIIGrade,
            81902 => Self::KylixiumIIIGrade,
            81903 => Self::KylixiumIVGrade,
            82300 => Self::CompressedKylixiumIGrade,
            82301 => Self::CompressedKylixiumIIGrade,
            82302 => Self::CompressedKylixiumIIIGrade,
            82303 => Self::CompressedKylixiumIVGrade,

            11396 => Self::Mercoxit,
            17869 => Self::MagmaMercoxit,
            17870 => Self::VitreousMercoxit,
            62586 => Self::CompressedMercoxit,
            62587 => Self::CompressedMagmaMercoxit,
            62588 => Self::CompressedVitreousMercoxit,
            28413 => Self::BatchCompressedMercoxit,
            28412 => Self::BatchCompressedMagmaMercoxit,
            28414 => Self::BatchCompressedVitreousMercoxit,

            74521 => Self::Mordunium,
            74522 => Self::PlumMordunium,
            74523 => Self::PrizeMordunium,
            74524 => Self::PlunderMordunium,
            75275 => Self::CompressedMordunium,
            75276 => Self::CompressedPlumMordunium,
            75277 => Self::CompressedPrizeMordunium,
            75278 => Self::CompressedPlunderMordunium,

            82016 => Self::Nocxite,
            82017 => Self::FragrantNocxite,
            82018 => Self::IntoxicatingNocxite,
            82019 => Self::AmbrosialNocxite,
            82304 => Self::CompressedNocxite,
            82305 => Self::CompressedFragrantNocxite,
            82306 => Self::CompressedIntoxicatingNocxite,
            82307 => Self::CompressedAmbrosialNocxite,

            1227  => Self::Omber,
            17867 => Self::SilveryOmber,
            17868 => Self::GoldenOmber,
            46684 => Self::PlatinoidOmber,
            62532 => Self::CompressedOmber,
            62533 => Self::CompressedSilveryOmber,
            62534 => Self::CompressedGoldenOmber,
            62535 => Self::CompressedPlatinoidOmber,
            28416 => Self::BatchCompressedOmber,
            28417 => Self::BatchCompressedSilveryOmber,
            28415 => Self::BatchCompressedGoldenOmber,
            46700 => Self::BatchCompressedPlatinoidOmber,

            18    => Self::Plagioclase,
            17455 => Self::AzurePlagioclase,
            17456 => Self::RichPlagioclase,
            46685 => Self::SparklingPlagioclase,
            62528 => Self::CompressedPlagioclase,
            62529 => Self::CompressedAzurePlagioclase,
            62530 => Self::CompressedRichPlagioclase,
            62531 => Self::CompressedSparklingPlagioclase,
            28422 => Self::BatchCompressedPlagioclase,
            28421 => Self::BatchCompressedAzurePlagioclase,
            28423 => Self::BatchCompressedRichPlagioclase,
            46701 => Self::BatchCompressedSparklingPlagioclase,

            1224  => Self::Pyroxeres,
            17459 => Self::SolidPyroxeres,
            17460 => Self::ViscousPyroxeres,
            46686 => Self::OpulentPyroxeres,
            62524 => Self::CompressedPyroxeres,
            62525 => Self::CompressedSolidPyroxeres,
            62526 => Self::CompressedViscousPyroxeres,
            62527 => Self::CompressedOpulentPyroxeres,
            28424 => Self::BatchCompressedPyroxeres,
            28425 => Self::BatchCompressedSolidPyroxeres,
            28426 => Self::BatchCompressedViscousPyroxeres,
            46702 => Self::BatchCompressedOpulentPyroxeres,

            52315 => Self::Rakovene,
            56629 => Self::RakoveneIIGrade,
            56630 => Self::RakoveneIIIGrade,
            62579 => Self::CompressedRakovene,
            62580 => Self::CompressedRakoveneIIGrade,
            62581 => Self::CompressedRakoveneIIIGrade,

            1228  => Self::Scordite,
            17463 => Self::CondensedScordite,
            17464 => Self::MassiveScordite,
            46687 => Self::GlossyScordite,
            62520 => Self::CompressedScordite,
            62521 => Self::CompressedCondensedScordite,
            62522 => Self::CompressedMassiveScordite,
            62523 => Self::CompressedGlossyScordite,
            28429 => Self::BatchCompressedScordite,
            28427 => Self::BatchCompressedCondensedScordite,
            28428 => Self::BatchCompressedMassiveScordite,
            46703 => Self::BatchCompressedGlossyScordite,

            19    => Self::Spodumain,
            17466 => Self::BrightSpodumain,
            17467 => Self::GleamingSpodumain,
            46688 => Self::DazzlingSpodumain,
            62572 => Self::CompressedSpodumain,
            62573 => Self::CompressedBrightSpodumain,
            62574 => Self::CompressedGleamingSpodumain,
            62575 => Self::CompressedDazzlingSpodumain,
            28420 => Self::BatchCompressedSpodumain,
            28418 => Self::BatchCompressedBrightSpodumain,
            28419 => Self::BatchCompressedGleamingSpodumain,
            46704 => Self::BatchCompressedDazzlingSpodumain,

            52306 => Self::Talassonite,
            56625 => Self::TalassoniteIIGrade,
            56626 => Self::TalassoniteIIIGrade,
            62582 => Self::CompressedTalassonite,
            62583 => Self::CompressedTalassoniteIIGrade,
            62584 => Self::CompressedTalassoniteIIIGrade,

            82205 => Self::Ueganite,
            82206 => Self::FoggyUeganite,
            82207 => Self::OvercastUeganite,
            82208 => Self::StormyUeganite,
            82308 => Self::CompressedUeganite,
            82309 => Self::CompressedFoggyUeganite,
            82310 => Self::CompressedOvercastUeganite,
            82311 => Self::CompressedStormyUeganite,

            1230  => Self::Veldspar,
            17470 => Self::ConcentratedVeldspar,
            17471 => Self::DenseVeldspar,
            46689 => Self::StableVeldspar,
            62516 => Self::CompressedVeldspar,
            62517 => Self::CompressedConcentratedVeldspar,
            62518 => Self::CompressedDenseVeldspar,
            62519 => Self::CompressedStableVeldspar,
            28432 => Self::BatchCompressedVeldspar,
            28430 => Self::BatchCompressedConcentratedVeldspar,
            28431 => Self::BatchCompressedDenseVeldspar,
            46705 => Self::BatchCompressedStableVeldspar,

            74525 => Self::Ytirium,
            74526 => Self::BootlegYtirium,
            74527 => Self::FirewaterYtirium,
            74528 => Self::MoonshineYtirium,
            75279 => Self::CompressedYtirium,
            75280 => Self::CompressedBootlegYtirium,
            75281 => Self::CompressedFirewaterYtirium,
            75282 => Self::CompressedMoonshineYtirium,

            // MoonGoo
            45492 => Self::Bitumens,
            46284 => Self::BrimfulBitumens,
            46285 => Self::GlisteningBitumens,
            62454 => Self::CompressedBitumens,
            62455 => Self::CompressedBrimfulBitumens,
            62456 => Self::CompressedGlisteningBitumens,

            45493 => Self::Coesite,
            46286 => Self::BrimfulCoesite,
            46287 => Self::GlisteningCoesite,
            62457 => Self::CompressedCoesite,
            62458 => Self::CompressedBrimfulCoesite,
            62459 => Self::CompressedGlisteningCoesite,

            45491 => Self::Sylvite,
            46282 => Self::BrimfulSylvite,
            46283 => Self::GlisteningSylvite,
            62460 => Self::CompressedSylvite,
            62461 => Self::CompressedBrimfulSylvite,
            62466 => Self::CompressedGlisteningSylvite,

            45490 => Self::Zeolites,
            46280 => Self::BrimfulZeolites,
            46281 => Self::GlisteningZeolites,
            62463 => Self::CompressedZeolites,
            62464 => Self::CompressedBrimfulZeolites,
            62467 => Self::CompressedGlisteningZeolites,

            45494 => Self::Cobaltite,
            46288 => Self::CopiousCobaltite,
            46289 => Self::TwinklingCobaltite,
            62474 => Self::CompressedCobaltite,
            62475 => Self::CompressedCopiousCobaltite,
            62476 => Self::CompressedTwinklingCobaltite,

            45495 => Self::Euxenite,
            46290 => Self::CopiousEuxenite,
            46291 => Self::TwinklingEuxenite,
            62471 => Self::CompressedEuxenite,
            62472 => Self::CompressedCopiousEuxenite,
            62473 => Self::CompressedTwinklingEuxenite,

            45497 => Self::Scheelite,
            46294 => Self::CopiousScheelite,
            46295 => Self::TwinklingScheelite,
            62468 => Self::CompressedScheelite,
            62469 => Self::CompressedCopiousScheelite,
            62470 => Self::CompressedTwinklingScheelite,

            45496 => Self::Titanite,
            46292 => Self::CopiousTitanite,
            46293 => Self::TwinklingTitanite,
            62477 => Self::CompressedTitanite,
            62478 => Self::CompressedCopiousTitanite,
            62479 => Self::CompressedTwinklingTitanite,

            45501 => Self::Chromite,
            46302 => Self::LavishChromite,
            46303 => Self::ShimmeringChromite,
            62480 => Self::CompressedChromite,
            62481 => Self::CompressedLavishChromite,
            62482 => Self::CompressedShimmeringChromite,

            45498 => Self::Otavite,
            46296 => Self::LavishOtavite,
            46297 => Self::ShimmeringOtavite,
            62483 => Self::CompressedOtavite,
            62484 => Self::CompressedLavishOtavite,
            62485 => Self::CompressedShimmeringOtavite,

            45499 => Self::Sperrylite,
            46298 => Self::LavishSperrylite,
            46299 => Self::ShimmeringSperrylite,
            62486 => Self::CompressedSperrylite,
            62487 => Self::CompressedLavishSperrylite,
            62488 => Self::CompressedShimmeringSperrylite,

            45500 => Self::Vanadinite,
            46300 => Self::LavishVanadinite,
            46301 => Self::ShimmeringVanadinite,
            62489 => Self::CompressedVanadinite,
            62490 => Self::CompressedLavishVanadinite,
            62491 => Self::CompressedShimmeringVanadinite,

            45502 => Self::Carnotite,
            46304 => Self::RepleteCarnotite,
            46305 => Self::GlowingCarnotite,
            62492 => Self::CompressedCarnotite,
            62493 => Self::CompressedRepleteCarnotite,
            62494 => Self::CompressedGlowingCarnotite,

            45506 => Self::Cinnabar,
            46310 => Self::RepleteCinnabar,
            46311 => Self::GlowingCinnabar,
            62495 => Self::CompressedCinnabar,
            62496 => Self::CompressedRepleteCinnabar,
            62497 => Self::CompressedGlowingCinnabar,

            45504 => Self::Pollucite,
            46308 => Self::RepletePollucite,
            46309 => Self::GlowingPollucite,
            62498 => Self::CompressedPollucite,
            62499 => Self::CompressedRepletePollucite,
            62500 => Self::CompressedGlowingPollucite,

            45503 => Self::Zircon,
            46306 => Self::RepleteZircon,
            46307 => Self::GlowingZircon,
            62501 => Self::CompressedZircon,
            62502 => Self::CompressedRepleteZircon,
            62503 => Self::CompressedGlowingZircon,

            45512 => Self::Loparite,
            46316 => Self::BountifulLoparite,
            46317 => Self::ShiningLoparite,
            62504 => Self::CompressedLoparite,
            62505 => Self::CompressedBountifulLoparite,
            62506 => Self::CompressedShiningLoparite,

            45511 => Self::Monazite,
            46314 => Self::BountifulMonazite,
            46315 => Self::ShiningMonazite,
            62507 => Self::CompressedMonazite,
            62508 => Self::CompressedBountifulMonazite,
            62509 => Self::CompressedShiningMonazite,

            45510 => Self::Xenotime,
            46312 => Self::BountifulXenotime,
            46313 => Self::ShiningXenotime,
            62510 => Self::CompressedXenotime,
            62511 => Self::CompressedBountifulXenotime,
            62512 => Self::CompressedShiningXenotime,

            45513 => Self::Ytterbite,
            46318 => Self::BountifulYtterbite,
            46319 => Self::ShiningYtterbite,
            62513 => Self::CompressedYtterbite,
            62514 => Self::CompressedBountifulYtterbite,
            62515 => Self::CompressedShiningYtterbite,

            // minerals
            34    => Self::Tritanium,
            35    => Self::Pyerite,
            36    => Self::Mexallon,
            37    => Self::Isogen,
            38    => Self::Nocxium,
            39    => Self::Zydrine,
            40    => Self::Megacyte,
            11399 => Self::Morphite,

            16634 => Self::AtmosphericGases,
            16635 => Self::EvaporiteDeposits,
            16633 => Self::Hydrocarbons,
            16636 => Self::Silicates,

            16640 => Self::Cobalt,
            16639 => Self::Scandium,
            16638 => Self::Titanium,
            16637 => Self::Tungsten,

            16641 => Self::Chromium,
            16643 => Self::Cadmium,
            16644 => Self::Platinum,
            16642 => Self::Vanadium,

            16647 => Self::Caesium,
            16648 => Self::Hafnium,
            16646 => Self::Mercury,
            16649 => Self::Technetium,
            16652 => Self::Promethium,
            16651 => Self::Neodymium,
            16650 => Self::Dysprosium,
            16653 => Self::Thulium,

            // ice
            16264 => Self::BlueIce,
            17975 => Self::BlueIceIVGrade,
            28433 => Self::CompressedBlueIce,
            28443 => Self::CompressedBlueIceIVGrade,
            16262 => Self::IcicleIIGrade,
            17978 => Self::IcicleIVGradeIIGrade,
            28434 => Self::CompressedIcicleIIGrade,
            28436 => Self::CompressedIcicleIVGradeIIGrade,
            16263 => Self::GlacialMass,
            17977 => Self::GlacialMassIVGrade,
            28438 => Self::CompressedGlacialMass,
            28442 => Self::CompressedGlacialMassIVGrade,
            16265 => Self::WhiteGlaze,
            17976 => Self::WhiteGlazeIVGrade,
            28444 => Self::CompressedWhiteGlaze,
            28441 => Self::CompressedWhiteGlazeIVGrade,
            16267 => Self::DarkGlitter,
            28435 => Self::CompressedDarkGlitter,
            16268 => Self::Gelidus,
            28437 => Self::CompressedGelidus,
            16266 => Self::GlareCrust,
            28439 => Self::CompressedGlareCrust,
            16269 => Self::Krystallos,
            28440 => Self::CompressedKrystallos,

            _ => unimplemented!()
        }
    }

    pub fn mineral(
        &self,
        mineral: Mineral
    ) -> f64 {
        self.minerals()
            .get(&mineral)
            .unwrap_or(&0f64)
            .clone()
    }

    pub fn minerals(&self) -> HashMap<Mineral, f64> {
        let mineral_init = |minerals: Vec<Mineral>, base: Vec<f64>| {
            minerals
                .into_iter()
                .zip(base.into_iter())
                .collect::<HashMap<_, _>>()
        };

        match self {
            Self::ArkonorIGrade                                   |
            Self::ArkonorIIGrade                            |
            Self::ArkonorIIIGrade                              |
            Self::ArkonorIVGrade                           |
            Self::CompressedArkonorIGrade                         |
            Self::CompressedArkonorIIGrade                  |
            Self::CompressedArkonorIIIGrade                    |
            Self::CompressedArkonorIVGrade                 |
            Self::BatchCompressedArkonorIGrade                    |
            Self::BatchCompressedArkonorIIGrade             |
            Self::BatchCompressedArkonorIIIGrade               |
            Self::BatchCompressedArkonorIVGrade            => {
                mineral_init(
                    vec![
                        Mineral::Pyerite,
                        Mineral::Mexallon,
                        Mineral::Megacyte,
                    ],
                    vec![
                        3200f64 * self.ore_modifier(),
                        1200f64 * self.ore_modifier(),
                        120f64 * self.ore_modifier(),
                    ],
                )
            },

            Self::BezdnacineIGrade                                |
            Self::BezdnacineIIGrade                         |
            Self::BezdnacineIIIGrade                           |
            Self::CompressedBezdnacineIGrade                      |
            Self::CompressedBezdnacineIIGrade               |
            Self::CompressedBezdnacineIIIGrade                 => {
                mineral_init(
                    vec![
                        Mineral::Tritanium,
                        Mineral::Isogen,
                        Mineral::Megacyte,
                    ],
                    vec![
                        40000f64 * self.ore_modifier(),
                        4800f64 * self.ore_modifier(),
                        128f64 * self.ore_modifier(),
                    ],
                )
            },

            Self::BistotIGrade                                    |
            Self::BistotIIGrade                           |
            Self::BistotIIIGrade                          |
            Self::BistotIVGrade                               |
            Self::CompressedBistotIGrade                          |
            Self::CompressedBistotIIGrade                 |
            Self::CompressedBistotIIIGrade                |
            Self::CompressedBistotIVGrade                     |
            Self::BatchCompressedBistotIGrade                     |
            Self::BatchCompressedBistotIIGrade            |
            Self::BatchCompressedBistotIIIGrade           |
            Self::BatchCompressedBistotIVGrade                => {
                mineral_init(
                    vec![
                        Mineral::Pyerite,
                        Mineral::Mexallon,
                        Mineral::Zydrine,
                    ],
                    vec![
                        3200f64 * self.ore_modifier(),
                        1200f64 * self.ore_modifier(),
                        160f64 * self.ore_modifier(),
                    ],
                )
            },

            Self::CrokiteIGrade                                   |
            Self::CrokiteIIGrade                              |
            Self::CrokiteIIIGrade                        |
            Self::CrokiteIVGrade                           |
            Self::CompressedCrokiteIGrade                         |
            Self::CompressedCrokiteIIGrade                    |
            Self::CompressedCrokiteIIIGrade              |
            Self::CompressedCrokiteIVGrade                 |
            Self::BatchCompressedCrokiteIGrade                    |
            Self::BatchCompressedCrokiteIIGrade               |
            Self::BatchCompressedCrokiteIIIGrade         |
            Self::BatchCompressedCrokiteIVGrade            => {
                mineral_init(
                    vec![
                        Mineral::Pyerite,
                        Mineral::Mexallon,
                        Mineral::Nocxium,
                    ],
                    vec![
                        800f64 * self.ore_modifier(),
                        2000f64 * self.ore_modifier(),
                        800f64 * self.ore_modifier(),
                    ],
                )
            },

            Self::DarkOchreIGrade                                 |
            Self::OchreIIGrade                                 |
            Self::OchreIIIGrade                             |
            Self::OchreIVGrade                                  |
            Self::CompressedDarkOchreIGrade                       |
            Self::CompressedOchreIIGrade                       |
            Self::CompressedOchreIIIGrade                   |
            Self::CompressedOchreIVGrade                        |
            Self::BatchCompressedDarkOchreIGrade                  |
            Self::BatchCompressedOchreIIGrade                  |
            Self::BatchCompressedOchreIIIGrade              |
            Self::BatchCompressedOchreIVGrade                   => {
                mineral_init(
                    vec![
                        Mineral::Mexallon,
                        Mineral::Isogen,
                        Mineral::Nocxium,
                    ],
                    vec![
                        1360f64 * self.ore_modifier(),
                        1200f64 * self.ore_modifier(),
                        320f64 * self.ore_modifier(),
                    ],
                )
            },

            Self::DuciniumIGrade                                  |
            Self::DuciniumIIGrade                             |
            Self::DuciniumIIIGrade                             |
            Self::DuciniumIVGrade                          |
            Self::CompressedDuciniumIGrade                        |
            Self::CompressedDuciniumIIGrade                   |
            Self::CompressedDuciniumIIIGrade                   |
            Self::CompressedDuciniumIVGrade                => {
                mineral_init(
                    vec![
                        Mineral::Megacyte,
                    ],
                    vec![
                        170f64 * self.ore_modifier(),
                    ],
                )
            },

            Self::EifyriumIGrade                                  |
            Self::EifyriumIIGrade                             |
            Self::EifyriumIIIGrade                           |
            Self::EifyriumIVGrade                         |
            Self::CompressedEifyriumIGrade                        |
            Self::CompressedEifyriumIIGrade                   |
            Self::CompressedEifyriumIIIGrade                 |
            Self::CompressedEifyriumIVGrade               => {
                mineral_init(
                    vec![
                        Mineral::Zydrine,
                    ],
                    vec![
                        266f64 * self.ore_modifier(),
                    ],
                )
            },

            Self::GneissIGrade                                    |
            Self::GneissIIGrade                          |
            Self::GneissIIIGrade                           |
            Self::GneissIVGrade                           |
            Self::CompressedGneissIGrade                          |
            Self::CompressedGneissIIGrade                |
            Self::CompressedGneissIIIGrade                 |
            Self::CompressedGneissIVGrade                 |
            Self::BatchCompressedGneissIGrade                     |
            Self::BatchCompressedGneissIIGrade           |
            Self::BatchCompressedGneissIIIGrade            |
            Self::BatchCompressedGneissIVGrade            => {
                mineral_init(
                    vec![
                        Mineral::Pyerite,
                        Mineral::Mexallon,
                        Mineral::Isogen,
                    ],
                    vec![
                        2000f64 * self.ore_modifier(),
                        1500f64 * self.ore_modifier(),
                        800f64 * self.ore_modifier(),
                    ],
                )
            },

            Self::GriemeerIGrade                                  |
            Self::GriemeerIIGrade                             |
            Self::GriemeerIIIGrade                              |
            Self::GriemeerIVGrade                            |
            Self::CompressedGriemeerIGrade                        |
            Self::CompressedGriemeerIIGrade                   |
            Self::CompressedGriemeerIIIGrade                    |
            Self::CompressedGriemeerIVGrade                  => {
                mineral_init(
                    vec![
                        Mineral::Tritanium,
                        Mineral::Isogen,
                    ],
                    vec![
                        250f64 * self.ore_modifier(),
                        80f64 * self.ore_modifier(),
                    ],
                )
            },

            Self::HedbergiteIGrade                                |
            Self::HedbergiteIIGrade                          |
            Self::HedbergiteIIIGrade                          |
            Self::HedbergiteIVGrade                        |
            Self::CompressedHedbergiteIGrade                      |
            Self::CompressedHedbergiteIIGrade                |
            Self::CompressedHedbergiteIIIGrade                |
            Self::CompressedHedbergiteIVGrade              |
            Self::BatchCompressedHedbergiteIGrade                 |
            Self::BatchCompressedHedbergiteIIGrade           |
            Self::BatchCompressedHedbergiteIIIGrade           |
            Self::BatchCompressedHedbergiteIVGrade         => {
                mineral_init(
                    vec![
                        Mineral::Pyerite,
                        Mineral::Nocxium,
                    ],
                    vec![
                        450f64 * self.ore_modifier(),
                        120f64 * self.ore_modifier(),
                    ],
                )
            },

            Self::HemorphiteIGrade                                |
            Self::HemorphiteIIGrade                           |
            Self::HemorphiteIIIGrade                         |
            Self::HemorphiteIVGrade                   |
            Self::CompressedHemorphiteIGrade                      |
            Self::CompressedHemorphiteIIGrade                 |
            Self::CompressedHemorphiteIIIGrade               |
            Self::CompressedHemorphiteIVGrade         |
            Self::BatchCompressedHemorphiteIGrade                 |
            Self::BatchCompressedHemorphiteIIGrade            |
            Self::BatchCompressedHemorphiteIIIGrade          |
            Self::BatchCompressedHemorphiteIVGrade    => {
                mineral_init(
                    vec![
                        Mineral::Tritanium,
                        Mineral::Isogen,
                        Mineral::Nocxium,
                    ],
                    vec![
                        2000f64 * self.ore_modifier(),
                        240f64 * self.ore_modifier(),
                        90f64 * self.ore_modifier(),
                    ],
                )
            },

            Self::HezorimeIGrade                                  |
            Self::HezorimeIIGrade                              |
            Self::HezorimeIIIGrade                          |
            Self::HezorimeIVGrade                             |
            Self::CompressedHezorimeIGrade                        |
            Self::CompressedHezorimeIIGrade                    |
            Self::CompressedHezorimeIIIGrade                |
            Self::CompressedHezorimeIVGrade                   => {
                mineral_init(
                    vec![
                        Mineral::Tritanium,
                        Mineral::Isogen,
                        Mineral::Zydrine,
                    ],
                    vec![
                        2000f64 * self.ore_modifier(),
                        120f64 * self.ore_modifier(),
                        60f64 * self.ore_modifier(),
                    ],
                )
            },

            Self::JaspetIGrade                                    |
            Self::JaspetIIGrade                                |
            Self::JaspetIIIGrade                            |
            Self::JaspetIVGrade                          |
            Self::CompressedJaspetIGrade                          |
            Self::CompressedJaspetIIGrade                      |
            Self::CompressedJaspetIIIGrade                  |
            Self::CompressedJaspetIVGrade                |
            Self::BatchCompressedJaspetIGrade                     |
            Self::BatchCompressedJaspetIIGrade                 |
            Self::BatchCompressedJaspetIIIGrade             |
            Self::BatchCompressedJaspetIVGrade           => {
                mineral_init(
                    vec![
                        Mineral::Mexallon,
                        Mineral::Nocxium,
                    ],
                    vec![
                        150f64 * self.ore_modifier(),
                        50f64 * self.ore_modifier(),
                    ],
                )
            },

            Self::KerniteIGrade                                   |
            Self::KerniteIIGrade                           |
            Self::KerniteIIIGrade                              |
            Self::KerniteIVGrade                        |
            Self::CompressedKerniteIGrade                         |
            Self::CompressedKerniteIIGrade                 |
            Self::CompressedKerniteIIIGrade                    |
            Self::CompressedKerniteIVGrade              |
            Self::BatchCompressedKerniteIGrade                    |
            Self::BatchCompressedKerniteIIGrade            |
            Self::BatchCompressedKerniteIIIGrade               |
            Self::BatchCompressedKerniteIVGrade         => {
                mineral_init(
                    vec![
                        Mineral::Mexallon,
                        Mineral::Isogen,
                    ],
                    vec![
                        60f64 * self.ore_modifier(),
                        120f64 * self.ore_modifier(),
                    ],
                )
            },

            Self::KylixiumIGrade                                  |
            Self::KylixiumIIGrade                            |
            Self::KylixiumIIIGrade                             |
            Self::KylixiumIVGrade                             |
            Self::CompressedKylixiumIGrade                        |
            Self::CompressedKylixiumIIGrade                  |
            Self::CompressedKylixiumIIIGrade                   |
            Self::CompressedKylixiumIVGrade                   => {
                mineral_init(
                    vec![
                        Mineral::Tritanium,
                        Mineral::Pyerite,
                        Mineral::Mexallon,
                    ],
                    vec![
                        300f64 * self.ore_modifier(),
                        200f64 * self.ore_modifier(),
                        550f64 * self.ore_modifier(),
                    ],
                )
            },

            Self::Mercoxit                                  |
            Self::MagmaMercoxit                             |
            Self::VitreousMercoxit                          |
            Self::CompressedMercoxit                        |
            Self::CompressedMagmaMercoxit                   |
            Self::CompressedVitreousMercoxit                |
            Self::BatchCompressedMercoxit                   |
            Self::BatchCompressedMagmaMercoxit              |
            Self::BatchCompressedVitreousMercoxit           => {
                mineral_init(
                    vec![
                        Mineral::Morphite,
                    ],
                    vec![
                        140f64 * self.ore_modifier(),
                    ],
                )
            },

            Self::Mordunium                                 |
            Self::PlumMordunium                             |
            Self::PrizeMordunium                            |
            Self::PlunderMordunium                          |
            Self::CompressedMordunium                       |
            Self::CompressedPlumMordunium                   |
            Self::CompressedPrizeMordunium                  |
            Self::CompressedPlunderMordunium                => {
                mineral_init(
                    vec![
                        Mineral::Pyerite,
                    ],
                    vec![
                        97f64 * self.ore_modifier(),
                    ],
                )
            },

            Self::Nocxite                                   |
            Self::FragrantNocxite                           |
            Self::IntoxicatingNocxite                       |
            Self::AmbrosialNocxite                          |
            Self::CompressedNocxite                         |
            Self::CompressedFragrantNocxite                 |
            Self::CompressedIntoxicatingNocxite             |
            Self::CompressedAmbrosialNocxite                => {
                mineral_init(
                    vec![
                        Mineral::Tritanium,
                        Mineral::Pyerite,
                        Mineral::Nocxium,
                    ],
                    vec![
                        900f64 * self.ore_modifier(),
                        150f64 * self.ore_modifier(),
                        105f64 * self.ore_modifier(),
                    ],
                )
            },

            Self::Omber                                     |
            Self::SilveryOmber                              |
            Self::GoldenOmber                               |
            Self::PlatinoidOmber                            |
            Self::CompressedOmber                           |
            Self::CompressedSilveryOmber                    |
            Self::CompressedGoldenOmber                     |
            Self::CompressedPlatinoidOmber                  |
            Self::BatchCompressedOmber                      |
            Self::BatchCompressedSilveryOmber               |
            Self::BatchCompressedGoldenOmber                |
            Self::BatchCompressedPlatinoidOmber             => {
                mineral_init(
                    vec![
                        Mineral::Pyerite,
                        Mineral::Isogen,
                    ],
                    vec![
                        90f64 * self.ore_modifier(),
                        75f64 * self.ore_modifier(),
                    ],
                )
            },

            Self::Plagioclase                               |
            Self::AzurePlagioclase                          |
            Self::RichPlagioclase                           |
            Self::SparklingPlagioclase                      |
            Self::CompressedPlagioclase                     |
            Self::CompressedAzurePlagioclase                |
            Self::CompressedRichPlagioclase                 |
            Self::CompressedSparklingPlagioclase            |
            Self::BatchCompressedPlagioclase                |
            Self::BatchCompressedAzurePlagioclase           |
            Self::BatchCompressedRichPlagioclase            |
            Self::BatchCompressedSparklingPlagioclase       => {
                mineral_init(
                    vec![
                        Mineral::Tritanium,
                        Mineral::Mexallon,
                    ],
                    vec![
                        175f64 * self.ore_modifier(),
                        70f64 * self.ore_modifier(),
                    ],
                )
            },

            Self::Pyroxeres                                 |
            Self::SolidPyroxeres                            |
            Self::ViscousPyroxeres                          |
            Self::OpulentPyroxeres                          |
            Self::CompressedPyroxeres                       |
            Self::CompressedSolidPyroxeres                  |
            Self::CompressedViscousPyroxeres                |
            Self::CompressedOpulentPyroxeres                |
            Self::BatchCompressedPyroxeres                  |
            Self::BatchCompressedSolidPyroxeres             |
            Self::BatchCompressedViscousPyroxeres           |
            Self::BatchCompressedOpulentPyroxeres           => {
                mineral_init(
                    vec![
                        Mineral::Pyerite,
                        Mineral::Mexallon,
                    ],
                    vec![
                        90f64 * self.ore_modifier(),
                        30f64 * self.ore_modifier(),
                    ],
                )
            },

            Self::Rakovene                                  |
            Self::RakoveneIIGrade                           |
            Self::RakoveneIIIGrade                             |
            Self::CompressedRakovene                        |
            Self::CompressedRakoveneIIGrade                 |
            Self::CompressedRakoveneIIIGrade                   => {
                mineral_init(
                    vec![
                        Mineral::Tritanium,
                        Mineral::Isogen,
                        Mineral::Zydrine,
                    ],
                    vec![
                        40000f64 * self.ore_modifier(),
                        3200f64 * self.ore_modifier(),
                        200f64 * self.ore_modifier(),
                    ],
                )
            },

            Self::Scordite                                  |
            Self::CondensedScordite                         |
            Self::MassiveScordite                           |
            Self::GlossyScordite                            |
            Self::CompressedScordite                        |
            Self::CompressedCondensedScordite               |
            Self::CompressedMassiveScordite                 |
            Self::CompressedGlossyScordite                  |
            Self::BatchCompressedScordite                   |
            Self::BatchCompressedCondensedScordite          |
            Self::BatchCompressedMassiveScordite            |
            Self::BatchCompressedGlossyScordite             => {
                mineral_init(
                    vec![
                        Mineral::Tritanium,
                        Mineral::Pyerite,
                    ],
                    vec![
                        150f64 * self.ore_modifier(),
                        110f64 * self.ore_modifier(),
                    ],
                )
            },

            Self::Spodumain                                 |
            Self::BrightSpodumain                           |
            Self::GleamingSpodumain                         |
            Self::DazzlingSpodumain                         |
            Self::CompressedSpodumain                       |
            Self::CompressedBrightSpodumain                 |
            Self::CompressedGleamingSpodumain               |
            Self::CompressedDazzlingSpodumain               |
            Self::BatchCompressedSpodumain                  |
            Self::BatchCompressedBrightSpodumain            |
            Self::BatchCompressedGleamingSpodumain          |
            Self::BatchCompressedDazzlingSpodumain          => {
                mineral_init(
                    vec![
                        Mineral::Tritanium,
                        Mineral::Isogen,
                        Mineral::Nocxium,
                        Mineral::Zydrine,
                        Mineral::Megacyte,
                    ],
                    vec![
                        48000f64 * self.ore_modifier(),
                        1000f64 * self.ore_modifier(),
                        160f64 * self.ore_modifier(),
                        80f64 * self.ore_modifier(),
                        40f64 * self.ore_modifier(),
                    ],
                )
            },

            Self::Talassonite                               |
            Self::TalassoniteIIGrade                        |
            Self::TalassoniteIIIGrade                          |
            Self::CompressedTalassonite                     |
            Self::CompressedTalassoniteIIGrade              |
            Self::CompressedTalassoniteIIIGrade                => {
                mineral_init(
                    vec![
                        Mineral::Tritanium,
                        Mineral::Nocxium,
                        Mineral::Megacyte,
                    ],
                    vec![
                        40000f64 * self.ore_modifier(),
                        960f64 * self.ore_modifier(),
                        32f64 * self.ore_modifier(),
                    ],
                )
            },

            Self::Ueganite                                  |
            Self::FoggyUeganite                             |
            Self::OvercastUeganite                          |
            Self::StormyUeganite                            |
            Self::CompressedUeganite                        |
            Self::CompressedFoggyUeganite                   |
            Self::CompressedOvercastUeganite                |
            Self::CompressedStormyUeganite                  => {
                mineral_init(
                    vec![
                        Mineral::Tritanium,
                        Mineral::Megacyte,
                    ],
                    vec![
                        800f64 * self.ore_modifier(),
                        40f64 * self.ore_modifier(),
                    ],
                )
            },

            Self::Veldspar                                  |
            Self::ConcentratedVeldspar                      |
            Self::DenseVeldspar                             |
            Self::StableVeldspar                            |
            Self::CompressedVeldspar                        |
            Self::CompressedConcentratedVeldspar            |
            Self::CompressedDenseVeldspar                   |
            Self::CompressedStableVeldspar                  |
            Self::BatchCompressedVeldspar                   |
            Self::BatchCompressedConcentratedVeldspar       |
            Self::BatchCompressedDenseVeldspar              |
            Self::BatchCompressedStableVeldspar             => {
                mineral_init(
                    vec![
                        Mineral::Tritanium,
                    ],
                    vec![
                        400f64 * self.ore_modifier(),
                    ],
                )
            },

            Self::Ytirium                                   |
            Self::BootlegYtirium                            |
            Self::FirewaterYtirium                          |
            Self::MoonshineYtirium                          |
            Self::CompressedYtirium                         |
            Self::CompressedBootlegYtirium                  |
            Self::CompressedFirewaterYtirium                |
            Self::CompressedMoonshineYtirium                => {
                mineral_init(
                    vec![
                        Mineral::Isogen,
                    ],
                    vec![
                        240f64 * self.ore_modifier(),
                    ],
                )
            },

            // MoonGoo
            Self::Bitumens                                  |
            Self::BrimfulBitumens                           |
            Self::GlisteningBitumens                        |
            Self::CompressedBitumens                        |
            Self::CompressedBrimfulBitumens                 |
            Self::CompressedGlisteningBitumens              => {
                mineral_init(
                    vec![
                        Mineral::Pyerite,
                        Mineral::Mexallon,
                        Mineral::Hydrocarbons,
                    ],
                    vec![
                        6000f64 * self.ore_modifier(),
                        400f64 * self.ore_modifier(),
                        65f64 * self.ore_modifier(),
                    ],
                )
            },

            Self::Coesite                                   |
            Self::BrimfulCoesite                            |
            Self::GlisteningCoesite                         |
            Self::CompressedCoesite                         |
            Self::CompressedBrimfulCoesite                  |
            Self::CompressedGlisteningCoesite               => {
                mineral_init(
                    vec![
                        Mineral::Pyerite,
                        Mineral::Mexallon,
                        Mineral::Silicates,
                    ],
                    vec![
                        2000f64 * self.ore_modifier(),
                        400f64 * self.ore_modifier(),
                        65f64 * self.ore_modifier(),
                    ],
                )
            },

            Self::Sylvite                                   |
            Self::BrimfulSylvite                            |
            Self::GlisteningSylvite                         |
            Self::CompressedSylvite                         |
            Self::CompressedBrimfulSylvite                  |
            Self::CompressedGlisteningSylvite               => {
                mineral_init(
                    vec![
                        Mineral::Pyerite,
                        Mineral::Mexallon,
                        Mineral::EvaporiteDeposits,
                    ],
                    vec![
                        4000f64 * self.ore_modifier(),
                        400f64 * self.ore_modifier(),
                        65f64 * self.ore_modifier(),
                    ],
                )
            },

            Self::Zeolites                                  |
            Self::BrimfulZeolites                           |
            Self::GlisteningZeolites                        |
            Self::CompressedZeolites                        |
            Self::CompressedBrimfulZeolites                 |
            Self::CompressedGlisteningZeolites              => {
                mineral_init(
                    vec![
                        Mineral::Pyerite,
                        Mineral::Mexallon,
                        Mineral::AtmosphericGases,
                    ],
                    vec![
                        8000f64 * self.ore_modifier(),
                        400f64 * self.ore_modifier(),
                        65f64 * self.ore_modifier(),
                    ],
                )
            },

            Self::Cobaltite                                 |
            Self::CopiousCobaltite                          |
            Self::TwinklingCobaltite                        |
            Self::CompressedCobaltite                       |
            Self::CompressedCopiousCobaltite                |
            Self::CompressedTwinklingCobaltite              => {
                mineral_init(
                    vec![
                        Mineral::Cobalt,
                    ],
                    vec![
                        40f64 * self.ore_modifier(),
                    ],
                )
            },

            Self::Euxenite                                  |
            Self::CopiousEuxenite                           |
            Self::TwinklingEuxenite                         |
            Self::CompressedEuxenite                        |
            Self::CompressedCopiousEuxenite                 |
            Self::CompressedTwinklingEuxenite               => {
                mineral_init(
                    vec![
                        Mineral::Scandium,
                    ],
                    vec![
                        40f64 * self.ore_modifier(),
                    ],
                )
            },

            Self::Scheelite                                 |
            Self::CopiousScheelite                          |
            Self::TwinklingScheelite                        |
            Self::CompressedScheelite                       |
            Self::CompressedCopiousScheelite                |
            Self::CompressedTwinklingScheelite              => {
                mineral_init(
                    vec![
                        Mineral::Tungsten,
                    ],
                    vec![
                        40f64 * self.ore_modifier(),
                    ],
                )
            },

            Self::Titanite                                  |
            Self::CopiousTitanite                           |
            Self::TwinklingTitanite                         |
            Self::CompressedTitanite                        |
            Self::CompressedCopiousTitanite                 |
            Self::CompressedTwinklingTitanite               => {
                mineral_init(
                    vec![
                        Mineral::Titanium,
                    ],
                    vec![
                        40f64 * self.ore_modifier(),
                    ],
                )
            },

            Self::Chromite                                  |
            Self::LavishChromite                            |
            Self::ShimmeringChromite                        |
            Self::CompressedChromite                        |
            Self::CompressedLavishChromite                  |
            Self::CompressedShimmeringChromite              => {
                mineral_init(
                    vec![
                        Mineral::Hydrocarbons,
                        Mineral::Chromium,
                    ],
                    vec![
                        10f64 * self.ore_modifier(),
                        40f64 * self.ore_modifier(),
                    ],
                )
            },

            Self::Otavite                                   |
            Self::LavishOtavite                             |
            Self::ShimmeringOtavite                         |
            Self::CompressedOtavite                         |
            Self::CompressedLavishOtavite                   |
            Self::CompressedShimmeringOtavite               => {
                mineral_init(
                    vec![
                        Mineral::AtmosphericGases,
                        Mineral::Cadmium,
                    ],
                    vec![
                        10f64 * self.ore_modifier(),
                        40f64 * self.ore_modifier(),
                    ],
                )
            },

            Self::Sperrylite                                |
            Self::LavishSperrylite                          |
            Self::ShimmeringSperrylite                      |
            Self::CompressedSperrylite                      |
            Self::CompressedLavishSperrylite                |
            Self::CompressedShimmeringSperrylite            => {
                mineral_init(
                    vec![
                        Mineral::EvaporiteDeposits,
                        Mineral::Platinum,
                    ],
                    vec![
                        10f64 * self.ore_modifier(),
                        40f64 * self.ore_modifier(),
                    ],
                )
            },

            Self::Vanadinite                                |
            Self::LavishVanadinite                          |
            Self::ShimmeringVanadinite                      |
            Self::CompressedVanadinite                      |
            Self::CompressedLavishVanadinite                |
            Self::CompressedShimmeringVanadinite            => {
                mineral_init(
                    vec![
                        Mineral::Silicates,
                        Mineral::Vanadium,
                    ],
                    vec![
                        10f64 * self.ore_modifier(),
                        40f64 * self.ore_modifier(),
                    ],
                )
            },

            Self::Carnotite                                 |
            Self::RepleteCarnotite                          |
            Self::GlowingCarnotite                          |
            Self::CompressedCarnotite                       |
            Self::CompressedRepleteCarnotite                |
            Self::CompressedGlowingCarnotite                => {
                mineral_init(
                    vec![
                        Mineral::AtmosphericGases,
                        Mineral::Cobalt,
                        Mineral::Technetium,
                    ],
                    vec![
                        15f64 * self.ore_modifier(),
                        10f64 * self.ore_modifier(),
                        50f64 * self.ore_modifier(),
                    ],
                )
            },

            Self::Cinnabar                                  |
            Self::RepleteCinnabar                           |
            Self::GlowingCinnabar                           |
            Self::CompressedCinnabar                        |
            Self::CompressedRepleteCinnabar                 |
            Self::CompressedGlowingCinnabar                 => {
                mineral_init(
                    vec![
                        Mineral::EvaporiteDeposits,
                        Mineral::Tungsten,
                        Mineral::Mercury,
                    ],
                    vec![
                        15f64 * self.ore_modifier(),
                        10f64 * self.ore_modifier(),
                        50f64 * self.ore_modifier(),
                    ],
                )
            },

            Self::Pollucite                                 |
            Self::RepletePollucite                          |
            Self::GlowingPollucite                          |
            Self::CompressedPollucite                       |
            Self::CompressedRepletePollucite                |
            Self::CompressedGlowingPollucite                => {
                mineral_init(
                    vec![
                        Mineral::Hydrocarbons,
                        Mineral::Scandium,
                        Mineral::Caesium,
                    ],
                    vec![
                        15f64 * self.ore_modifier(),
                        10f64 * self.ore_modifier(),
                        50f64 * self.ore_modifier(),
                    ],
                )
            },

            Self::Zircon                                    |
            Self::RepleteZircon                             |
            Self::GlowingZircon                             |
            Self::CompressedZircon                          |
            Self::CompressedRepleteZircon                   |
            Self::CompressedGlowingZircon                   => {
                mineral_init(
                    vec![
                        Mineral::Silicates,
                        Mineral::Titanium,
                        Mineral::Hafnium,
                    ],
                    vec![
                        15f64 * self.ore_modifier(),
                        10f64 * self.ore_modifier(),
                        50f64 * self.ore_modifier(),
                    ],
                )
            },

            Self::Loparite                                  |
            Self::BountifulLoparite                         |
            Self::ShiningLoparite                           |
            Self::CompressedLoparite                        |
            Self::CompressedBountifulLoparite               |
            Self::CompressedShiningLoparite                 => {
                mineral_init(
                    vec![
                        Mineral::Hydrocarbons,
                        Mineral::Scandium,
                        Mineral::Platinum,
                        Mineral::Promethium,
                    ],
                    vec![
                        20f64 * self.ore_modifier(),
                        20f64 * self.ore_modifier(),
                        10f64 * self.ore_modifier(),
                        22f64 * self.ore_modifier(),
                    ],
                )
            },

            Self::Monazite                                  |
            Self::BountifulMonazite                         |
            Self::ShiningMonazite                           |
            Self::CompressedMonazite                        |
            Self::CompressedBountifulMonazite               |
            Self::CompressedShiningMonazite                 => {
                mineral_init(
                    vec![
                        Mineral::EvaporiteDeposits,
                        Mineral::Tungsten,
                        Mineral::Chromium,
                        Mineral::Neodymium,
                    ],
                    vec![
                        20f64 * self.ore_modifier(),
                        20f64 * self.ore_modifier(),
                        10f64 * self.ore_modifier(),
                        22f64 * self.ore_modifier(),
                    ],
                )
            },

            Self::Xenotime                                  |
            Self::BountifulXenotime                         |
            Self::ShiningXenotime                           |
            Self::CompressedXenotime                        |
            Self::CompressedBountifulXenotime               |
            Self::CompressedShiningXenotime                 => {
                mineral_init(
                    vec![
                        Mineral::AtmosphericGases,
                        Mineral::Cobalt,
                        Mineral::Vanadium,
                        Mineral::Dysprosium,
                    ],
                    vec![
                        20f64 * self.ore_modifier(),
                        20f64 * self.ore_modifier(),
                        10f64 * self.ore_modifier(),
                        22f64 * self.ore_modifier(),
                    ],
                )
            },

            Self::Ytterbite                                 |
            Self::BountifulYtterbite                        |
            Self::ShiningYtterbite                          |
            Self::CompressedYtterbite                       |
            Self::CompressedBountifulYtterbite              |
            Self::CompressedShiningYtterbite                => {
                mineral_init(
                    vec![
                        Mineral::Silicates,
                        Mineral::Titanium,
                        Mineral::Cadmium,
                        Mineral::Thulium,
                    ],
                    vec![
                        20f64 * self.ore_modifier(),
                        20f64 * self.ore_modifier(),
                        10f64 * self.ore_modifier(),
                        22f64 * self.ore_modifier(),
                    ],
                )
            },

            Self::Tritanium                             => mineral_init(vec![Mineral::Tritanium], vec![1f64]),
            Self::Pyerite                               => mineral_init(vec![Mineral::Pyerite], vec![1f64]),
            Self::Mexallon                              => mineral_init(vec![Mineral::Mexallon], vec![1f64]),
            Self::Isogen                                => mineral_init(vec![Mineral::Isogen], vec![1f64]),
            Self::Nocxium                               => mineral_init(vec![Mineral::Nocxium], vec![1f64]),
            Self::Zydrine                               => mineral_init(vec![Mineral::Zydrine], vec![1f64]),
            Self::Megacyte                              => mineral_init(vec![Mineral::Megacyte], vec![1f64]),
            Self::Morphite                              => mineral_init(vec![Mineral::Morphite], vec![1f64]),

            Self::AtmosphericGases                      => mineral_init(vec![Mineral::AtmosphericGases], vec![1f64]),
            Self::EvaporiteDeposits                     => mineral_init(vec![Mineral::EvaporiteDeposits], vec![1f64]),
            Self::Hydrocarbons                          => mineral_init(vec![Mineral::Hydrocarbons], vec![1f64]),
            Self::Silicates                             => mineral_init(vec![Mineral::Silicates], vec![1f64]),
            Self::Cobalt                                => mineral_init(vec![Mineral::Cobalt], vec![1f64]),
            Self::Scandium                              => mineral_init(vec![Mineral::Scandium], vec![1f64]),
            Self::Titanium                              => mineral_init(vec![Mineral::Titanium], vec![1f64]),
            Self::Tungsten                              => mineral_init(vec![Mineral::Tungsten], vec![1f64]),
            Self::Chromium                              => mineral_init(vec![Mineral::Chromium], vec![1f64]),
            Self::Cadmium                               => mineral_init(vec![Mineral::Cadmium], vec![1f64]),
            Self::Platinum                              => mineral_init(vec![Mineral::Platinum], vec![1f64]),
            Self::Vanadium                              => mineral_init(vec![Mineral::Vanadium], vec![1f64]),
            Self::Caesium                               => mineral_init(vec![Mineral::Caesium], vec![1f64]),
            Self::Hafnium                               => mineral_init(vec![Mineral::Hafnium], vec![1f64]),
            Self::Mercury                               => mineral_init(vec![Mineral::Mercury], vec![1f64]),
            Self::Technetium                            => mineral_init(vec![Mineral::Technetium], vec![1f64]),
            Self::Promethium                            => mineral_init(vec![Mineral::Promethium], vec![1f64]),
            Self::Neodymium                             => mineral_init(vec![Mineral::Neodymium], vec![1f64]),
            Self::Dysprosium                            => mineral_init(vec![Mineral::Dysprosium], vec![1f64]),
            Self::Thulium                               => mineral_init(vec![Mineral::Thulium], vec![1f64]),

            Self::BlueIce                                   |
            Self::CompressedBlueIce                         => {
                mineral_init(
                    vec![
                        Mineral::HeavyWater,
                        Mineral::LiquidOzone,
                        Mineral::StrontiumClathrates,
                        Mineral::OxygenIsotopes,
                    ],
                    vec![
                        69f64,
                        35f64,
                        1f64,
                        414f64,
                    ],
                )
            },

            Self::BlueIceIVGrade                            |
            Self::CompressedBlueIceIVGrade                  => {
                mineral_init(
                    vec![
                        Mineral::HeavyWater,
                        Mineral::LiquidOzone,
                        Mineral::StrontiumClathrates,
                        Mineral::OxygenIsotopes,
                    ],
                    vec![
                        104f64,
                        55f64,
                        1f64,
                        483f64,
                    ],
                )
            },

            Self::IcicleIIGrade                               |
            Self::CompressedIcicleIIGrade                     => {
                mineral_init(
                    vec![
                        Mineral::HeavyWater,
                        Mineral::LiquidOzone,
                        Mineral::StrontiumClathrates,
                        Mineral::HeliumIsotopes,
                    ],
                    vec![
                        69f64,
                        35f64,
                        1f64,
                        414f64,
                    ],
                )
            },

            Self::IcicleIVGradeIIGrade                        |
            Self::CompressedIcicleIVGradeIIGrade              => {
                mineral_init(
                    vec![
                        Mineral::HeavyWater,
                        Mineral::LiquidOzone,
                        Mineral::StrontiumClathrates,
                        Mineral::HeliumIsotopes,
                    ],
                    vec![
                        104f64,
                        55f64,
                        1f64,
                        483f64,
                    ],
                )
            },

            Self::GlacialMass                               |
            Self::CompressedGlacialMass                     => {
                mineral_init(
                    vec![
                        Mineral::HeavyWater,
                        Mineral::LiquidOzone,
                        Mineral::StrontiumClathrates,
                        Mineral::HydrogenIsotopes,
                    ],
                    vec![
                        69f64,
                        35f64,
                        1f64,
                        414f64,
                    ],
                )
            },

            Self::GlacialMassIVGrade                        |
            Self::CompressedGlacialMassIVGrade              => {
                mineral_init(
                    vec![
                        Mineral::HeavyWater,
                        Mineral::LiquidOzone,
                        Mineral::StrontiumClathrates,
                        Mineral::HydrogenIsotopes,
                    ],
                    vec![
                        104f64,
                        55f64,
                        1f64,
                        483f64,
                    ],
                )
            },

            Self::WhiteGlaze                                |
            Self::CompressedWhiteGlaze                      => {
                mineral_init(
                    vec![
                        Mineral::HeavyWater,
                        Mineral::LiquidOzone,
                        Mineral::StrontiumClathrates,
                        Mineral::NitrogenIsotopes,
                    ],
                    vec![
                        69f64,
                        35f64,
                        1f64,
                        414f64,
                    ],
                )
            },

            Self::WhiteGlazeIVGrade                         |
            Self::CompressedWhiteGlazeIVGrade               => {
                mineral_init(
                    vec![
                        Mineral::HeavyWater,
                        Mineral::LiquidOzone,
                        Mineral::StrontiumClathrates,
                        Mineral::NitrogenIsotopes,
                    ],
                    vec![
                        104f64,
                        55f64,
                        1f64,
                        483f64,
                    ],
                )
            },

            Self::DarkGlitter                               |
            Self::CompressedDarkGlitter                     => {
                mineral_init(
                    vec![
                        Mineral::HeavyWater,
                        Mineral::LiquidOzone,
                        Mineral::StrontiumClathrates,
                    ],
                    vec![
                        691f64,
                        1381f64,
                        69f64,
                    ],
                )
            },

            Self::Gelidus                                   |
            Self::CompressedGelidus                         => {
                mineral_init(
                    vec![
                        Mineral::HeavyWater,
                        Mineral::LiquidOzone,
                        Mineral::StrontiumClathrates,
                    ],
                    vec![
                        345f64,
                        691f64,
                        104f64,
                    ],
                )
            },

            Self::GlareCrust                                |
            Self::CompressedGlareCrust                      => {
                mineral_init(
                    vec![
                        Mineral::HeavyWater,
                        Mineral::LiquidOzone,
                        Mineral::StrontiumClathrates,
                    ],
                    vec![
                        1381f64,
                        691f64,
                        35f64,
                    ],
                )
            },

            Self::Krystallos                                |
            Self::CompressedKrystallos                      => {
                mineral_init(
                    vec![
                        Mineral::HeavyWater,
                        Mineral::LiquidOzone,
                        Mineral::StrontiumClathrates,
                    ],
                    vec![
                        173f64,
                        691f64,
                        173f64,
                    ],
                )
            },
        }
    }

    pub fn ore_modifier(&self) -> f64 {
        match self {
            Self::ArkonorIGrade                                   |
            Self::CompressedArkonorIGrade                         |
            Self::BezdnacineIGrade                                |
            Self::CompressedBezdnacineIGrade                      |
            Self::BistotIGrade                                    |
            Self::CompressedBistotIGrade                          |
            Self::CrokiteIGrade                                   |
            Self::CompressedCrokiteIGrade                         |
            Self::DarkOchreIGrade                                 |
            Self::CompressedDarkOchreIGrade                       |
            Self::DuciniumIGrade                                  |
            Self::CompressedDuciniumIGrade                        |
            Self::EifyriumIGrade                                  |
            Self::CompressedEifyriumIGrade                        |
            Self::GneissIGrade                                    |
            Self::CompressedGneissIGrade                          |
            Self::GriemeerIGrade                                  |
            Self::CompressedGriemeerIGrade                        |
            Self::HedbergiteIGrade                                |
            Self::CompressedHedbergiteIGrade                      |
            Self::HemorphiteIGrade                                |
            Self::CompressedHemorphiteIGrade                      |
            Self::HezorimeIGrade                                  |
            Self::CompressedHezorimeIGrade                        |
            Self::JaspetIGrade                                    |
            Self::CompressedJaspetIGrade                          |
            Self::KerniteIGrade                                   |
            Self::CompressedKerniteIGrade                         |
            Self::KylixiumIGrade                                  |
            Self::CompressedKylixiumIGrade                        |
            Self::Mercoxit                                  |
            Self::CompressedMercoxit                        |
            Self::Mordunium                                 |
            Self::CompressedMordunium                       |
            Self::Nocxite                                   |
            Self::CompressedNocxite                         |
            Self::Omber                                     |
            Self::CompressedOmber                           |
            Self::Plagioclase                               |
            Self::CompressedPlagioclase                     |
            Self::Pyroxeres                                 |
            Self::CompressedPyroxeres                       |
            Self::Rakovene                                  |
            Self::CompressedRakovene                        |
            Self::Scordite                                  |
            Self::CompressedScordite                        |
            Self::Spodumain                                 |
            Self::CompressedSpodumain                       |
            Self::Talassonite                               |
            Self::CompressedTalassonite                     |
            Self::Ueganite                                  |
            Self::CompressedUeganite                        |
            Self::Veldspar                                  |
            Self::CompressedVeldspar                        |
            Self::Ytirium                                   |
            Self::CompressedYtirium                         => 1.00,

            Self::ArkonorIIGrade                            |
            Self::CompressedArkonorIIGrade                  |
            Self::BezdnacineIIGrade                         |
            Self::CompressedBezdnacineIIGrade               |
            Self::BistotIIGrade                           |
            Self::CompressedBistotIIGrade                 |
            Self::CrokiteIIGrade                              |
            Self::CompressedCrokiteIIGrade                    |
            Self::OchreIIGrade                                 |
            Self::CompressedOchreIIGrade                       |
            Self::DuciniumIIGrade                             |
            Self::CompressedDuciniumIIGrade                   |
            Self::EifyriumIIGrade                             |
            Self::CompressedEifyriumIIGrade                   |
            Self::GneissIIGrade                          |
            Self::CompressedGneissIIGrade                |
            Self::GriemeerIIGrade                             |
            Self::CompressedGriemeerIIGrade                   |
            Self::HedbergiteIIGrade                          |
            Self::CompressedHedbergiteIIGrade                |
            Self::HemorphiteIIGrade                           |
            Self::CompressedHemorphiteIIGrade                 |
            Self::HezorimeIIGrade                              |
            Self::CompressedHezorimeIIGrade                    |
            Self::JaspetIIGrade                                |
            Self::CompressedJaspetIIGrade                      |
            Self::KerniteIIGrade                           |
            Self::CompressedKerniteIIGrade                 |
            Self::KylixiumIIGrade                            |
            Self::CompressedKylixiumIIGrade                  |
            Self::MagmaMercoxit                             |
            Self::CompressedMagmaMercoxit                   |
            Self::PlumMordunium                             |
            Self::CompressedPlumMordunium                   |
            Self::FragrantNocxite                           |
            Self::CompressedFragrantNocxite                 |
            Self::SilveryOmber                              |
            Self::CompressedSilveryOmber                    |
            Self::AzurePlagioclase                          |
            Self::CompressedAzurePlagioclase                |
            Self::SolidPyroxeres                            |
            Self::CompressedSolidPyroxeres                  |
            Self::RakoveneIIGrade                           |
            Self::CompressedRakoveneIIGrade                 |
            Self::CondensedScordite                         |
            Self::CompressedCondensedScordite               |
            Self::BrightSpodumain                           |
            Self::CompressedBrightSpodumain                 |
            Self::TalassoniteIIGrade                        |
            Self::CompressedTalassoniteIIGrade              |
            Self::FoggyUeganite                             |
            Self::CompressedFoggyUeganite                   |
            Self::ConcentratedVeldspar                      |
            Self::CompressedConcentratedVeldspar            |
            Self::BootlegYtirium                            |
            Self::CompressedBootlegYtirium                  => 1.05,

            Self::ArkonorIIIGrade                              |
            Self::CompressedArkonorIIIGrade                    |
            Self::BezdnacineIIIGrade                           |
            Self::CompressedBezdnacineIIIGrade                 |
            Self::BistotIIIGrade                          |
            Self::CompressedBistotIIIGrade                |
            Self::CrokiteIIIGrade                        |
            Self::CompressedCrokiteIIIGrade              |
            Self::OchreIIIGrade                             |
            Self::CompressedOchreIIIGrade                   |
            Self::DuciniumIIIGrade                             |
            Self::CompressedDuciniumIIIGrade                   |
            Self::EifyriumIIIGrade                           |
            Self::CompressedEifyriumIIIGrade                 |
            Self::GneissIIIGrade                           |
            Self::CompressedGneissIIIGrade                 |
            Self::GriemeerIIIGrade                              |
            Self::CompressedGriemeerIIIGrade                    |
            Self::HedbergiteIIIGrade                          |
            Self::CompressedHedbergiteIIIGrade                |
            Self::HemorphiteIIIGrade                         |
            Self::CompressedHemorphiteIIIGrade               |
            Self::HezorimeIIIGrade                          |
            Self::CompressedHezorimeIIIGrade                |
            Self::JaspetIIIGrade                            |
            Self::CompressedJaspetIIIGrade                  |
            Self::KerniteIIIGrade                              |
            Self::CompressedKerniteIIIGrade                    |
            Self::KylixiumIIIGrade                             |
            Self::CompressedKylixiumIIIGrade                   |
            Self::VitreousMercoxit                          |
            Self::CompressedVitreousMercoxit                |
            Self::PrizeMordunium                            |
            Self::CompressedPrizeMordunium                  |
            Self::IntoxicatingNocxite                       |
            Self::CompressedIntoxicatingNocxite             |
            Self::GoldenOmber                               |
            Self::CompressedGoldenOmber                     |
            Self::RichPlagioclase                           |
            Self::CompressedRichPlagioclase                 |
            Self::ViscousPyroxeres                          |
            Self::CompressedViscousPyroxeres                |
            Self::RakoveneIIIGrade                             |
            Self::CompressedRakoveneIIIGrade                   |
            Self::MassiveScordite                           |
            Self::CompressedMassiveScordite                 |
            Self::GleamingSpodumain                         |
            Self::CompressedGleamingSpodumain               |
            Self::TalassoniteIIIGrade                          |
            Self::CompressedTalassoniteIIIGrade                |
            Self::OvercastUeganite                          |
            Self::CompressedOvercastUeganite                |
            Self::DenseVeldspar                             |
            Self::CompressedDenseVeldspar                   |
            Self::FirewaterYtirium                          |
            Self::CompressedFirewaterYtirium                => 1.10,

            Self::ArkonorIVGrade                           |
            Self::CompressedArkonorIVGrade                 |
            Self::BistotIVGrade                               |
            Self::CompressedBistotIVGrade                     |
            Self::CrokiteIVGrade                           |
            Self::CompressedCrokiteIVGrade                 |
            Self::OchreIVGrade                                  |
            Self::CompressedOchreIVGrade                        |
            Self::DuciniumIVGrade                          |
            Self::CompressedDuciniumIVGrade                |
            Self::EifyriumIVGrade                         |
            Self::CompressedEifyriumIVGrade               |
            Self::GneissIVGrade                           |
            Self::CompressedGneissIVGrade                 |
            Self::GriemeerIVGrade                            |
            Self::CompressedGriemeerIVGrade                  |
            Self::HedbergiteIVGrade                        |
            Self::CompressedHedbergiteIVGrade              |
            Self::HemorphiteIVGrade                   |
            Self::CompressedHemorphiteIVGrade         |
            Self::HezorimeIVGrade                             |
            Self::CompressedHezorimeIVGrade                   |
            Self::JaspetIVGrade                          |
            Self::CompressedJaspetIVGrade                |
            Self::KerniteIVGrade                        |
            Self::CompressedKerniteIVGrade              |
            Self::KylixiumIVGrade                             |
            Self::CompressedKylixiumIVGrade                   |
            Self::PlunderMordunium                          |
            Self::CompressedPlunderMordunium                |
            Self::AmbrosialNocxite                          |
            Self::CompressedAmbrosialNocxite                |
            Self::PlatinoidOmber                            |
            Self::CompressedPlatinoidOmber                  |
            Self::SparklingPlagioclase                      |
            Self::CompressedSparklingPlagioclase            |
            Self::OpulentPyroxeres                          |
            Self::CompressedOpulentPyroxeres                |
            Self::GlossyScordite                            |
            Self::CompressedGlossyScordite                  |
            Self::DazzlingSpodumain                         |
            Self::CompressedDazzlingSpodumain               |
            Self::StormyUeganite                            |
            Self::CompressedStormyUeganite                  |
            Self::StableVeldspar                            |
            Self::CompressedStableVeldspar                  |
            Self::MoonshineYtirium                          |
            Self::CompressedMoonshineYtirium                => 1.15,

            // moon
            Self::Bitumens                                  |
            Self::CompressedBitumens                        |
            Self::Coesite                                   |
            Self::CompressedCoesite                         |
            Self::Sylvite                                   |
            Self::CompressedSylvite                         |
            Self::Zeolites                                  |
            Self::CompressedZeolites                        |
            Self::Cobaltite                                 |
            Self::CompressedCobaltite                       |
            Self::Euxenite                                  |
            Self::CompressedEuxenite                        |
            Self::Scheelite                                 |
            Self::CompressedScheelite                       |
            Self::Titanite                                  |
            Self::CompressedTitanite                        |
            Self::Chromite                                  |
            Self::CompressedChromite                        |
            Self::Otavite                                   |
            Self::CompressedOtavite                         |
            Self::Sperrylite                                |
            Self::CompressedSperrylite                      |
            Self::Vanadinite                                |
            Self::CompressedVanadinite                      |
            Self::Carnotite                                 |
            Self::CompressedCarnotite                       |
            Self::Cinnabar                                  |
            Self::CompressedCinnabar                        |
            Self::Pollucite                                 |
            Self::CompressedPollucite                       |
            Self::Zircon                                    |
            Self::CompressedZircon                          |
            Self::Loparite                                  |
            Self::CompressedLoparite                        |
            Self::Monazite                                  |
            Self::CompressedMonazite                        |
            Self::Xenotime                                  |
            Self::CompressedXenotime                        |
            Self::Ytterbite                                 |
            Self::CompressedYtterbite                       => 1.00,

            Self::BrimfulBitumens                           |
            Self::CompressedBrimfulBitumens                 |
            Self::BrimfulCoesite                            |
            Self::CompressedBrimfulCoesite                  |
            Self::BrimfulSylvite                            |
            Self::CompressedBrimfulSylvite                  |
            Self::BrimfulZeolites                           |
            Self::CompressedBrimfulZeolites                 |
            Self::CopiousCobaltite                          |
            Self::CompressedCopiousCobaltite                |
            Self::CopiousEuxenite                           |
            Self::CompressedCopiousEuxenite                 |
            Self::CopiousScheelite                          |
            Self::CompressedCopiousScheelite                |
            Self::CopiousTitanite                           |
            Self::CompressedCopiousTitanite                 |
            Self::LavishChromite                            |
            Self::CompressedLavishChromite                  |
            Self::LavishOtavite                             |
            Self::CompressedLavishOtavite                   |
            Self::LavishSperrylite                          |
            Self::CompressedLavishSperrylite                |
            Self::LavishVanadinite                          |
            Self::CompressedLavishVanadinite                |
            Self::RepleteCarnotite                          |
            Self::CompressedRepleteCarnotite                |
            Self::RepleteCinnabar                           |
            Self::CompressedRepleteCinnabar                 |
            Self::RepletePollucite                          |
            Self::CompressedRepletePollucite                |
            Self::RepleteZircon                             |
            Self::CompressedRepleteZircon                   |
            Self::BountifulLoparite                         |
            Self::CompressedBountifulLoparite               |
            Self::BountifulMonazite                         |
            Self::CompressedBountifulMonazite               |
            Self::BountifulXenotime                         |
            Self::CompressedBountifulXenotime               |
            Self::BountifulYtterbite                        |
            Self::CompressedBountifulYtterbite              => 1.15,

            Self::GlisteningBitumens                        |
            Self::CompressedGlisteningBitumens              |
            Self::GlisteningCoesite                         |
            Self::CompressedGlisteningCoesite               |
            Self::GlisteningSylvite                         |
            Self::CompressedGlisteningSylvite               |
            Self::GlisteningZeolites                        |
            Self::CompressedGlisteningZeolites              |
            Self::TwinklingCobaltite                        |
            Self::CompressedTwinklingCobaltite              |
            Self::TwinklingEuxenite                         |
            Self::CompressedTwinklingEuxenite               |
            Self::TwinklingScheelite                        |
            Self::CompressedTwinklingScheelite              |
            Self::TwinklingTitanite                         |
            Self::CompressedTwinklingTitanite               |
            Self::ShimmeringChromite                        |
            Self::CompressedShimmeringChromite              |
            Self::ShimmeringOtavite                         |
            Self::CompressedShimmeringOtavite               |
            Self::ShimmeringSperrylite                      |
            Self::CompressedShimmeringSperrylite            |
            Self::ShimmeringVanadinite                      |
            Self::CompressedShimmeringVanadinite            |
            Self::GlowingCarnotite                          |
            Self::CompressedGlowingCarnotite                |
            Self::GlowingCinnabar                           |
            Self::CompressedGlowingCinnabar                 |
            Self::GlowingPollucite                          |
            Self::CompressedGlowingPollucite                |
            Self::GlowingZircon                             |
            Self::CompressedGlowingZircon                   |
            Self::ShiningLoparite                           |
            Self::CompressedShiningLoparite                 |
            Self::ShiningMonazite                           |
            Self::CompressedShiningMonazite                 |
            Self::ShiningXenotime                           |
            Self::CompressedShiningXenotime                 |
            Self::ShiningYtterbite                          |
            Self::CompressedShiningYtterbite                => 2.00,

            Self::Tritanium                                 |
            Self::Pyerite                                   |
            Self::Mexallon                                  |
            Self::Isogen                                    |
            Self::Nocxium                                   |
            Self::Zydrine                                   |
            Self::Megacyte                                  |
            Self::Morphite                                  |
            Self::AtmosphericGases                          |
            Self::EvaporiteDeposits                         |
            Self::Hydrocarbons                              |
            Self::Silicates                                 |
            Self::Cobalt                                    |
            Self::Scandium                                  |
            Self::Titanium                                  |
            Self::Tungsten                                  |
            Self::Chromium                                  |
            Self::Cadmium                                   |
            Self::Platinum                                  |
            Self::Vanadium                                  |
            Self::Caesium                                   |
            Self::Hafnium                                   |
            Self::Mercury                                   |
            Self::Technetium                                |
            Self::Promethium                                |
            Self::Neodymium                                 |
            Self::Dysprosium                                |
            Self::Thulium                                   => 1.00,

            Self::BatchCompressedArkonorIGrade                    |
            Self::BatchCompressedBistotIGrade                     |
            Self::BatchCompressedCrokiteIGrade                    |
            Self::BatchCompressedDarkOchreIGrade                  |
            Self::BatchCompressedGneissIGrade                     |
            Self::BatchCompressedHedbergiteIGrade                 |
            Self::BatchCompressedHemorphiteIGrade                 |
            Self::BatchCompressedJaspetIGrade                     |
            Self::BatchCompressedKerniteIGrade                    |
            Self::BatchCompressedMercoxit                   |
            Self::BatchCompressedOmber                      |
            Self::BatchCompressedPlagioclase                |
            Self::BatchCompressedPyroxeres                  |
            Self::BatchCompressedScordite                   |
            Self::BatchCompressedSpodumain                  |
            Self::BatchCompressedVeldspar                   => 100.00,

            Self::BatchCompressedArkonorIIGrade             |
            Self::BatchCompressedBistotIIGrade            |
            Self::BatchCompressedCrokiteIIGrade               |
            Self::BatchCompressedOchreIIGrade                  |
            Self::BatchCompressedGneissIIGrade           |
            Self::BatchCompressedHedbergiteIIGrade           |
            Self::BatchCompressedHemorphiteIIGrade            |
            Self::BatchCompressedJaspetIIGrade                 |
            Self::BatchCompressedKerniteIIGrade            |
            Self::BatchCompressedMagmaMercoxit              |
            Self::BatchCompressedSilveryOmber               |
            Self::BatchCompressedAzurePlagioclase           |
            Self::BatchCompressedSolidPyroxeres             |
            Self::BatchCompressedCondensedScordite          |
            Self::BatchCompressedBrightSpodumain            |
            Self::BatchCompressedConcentratedVeldspar       => 105.00,

            Self::BatchCompressedArkonorIIIGrade               |
            Self::BatchCompressedBistotIIIGrade           |
            Self::BatchCompressedCrokiteIIIGrade         |
            Self::BatchCompressedOchreIIIGrade              |
            Self::BatchCompressedGneissIIIGrade            |
            Self::BatchCompressedHedbergiteIIIGrade           |
            Self::BatchCompressedHemorphiteIIIGrade          |
            Self::BatchCompressedJaspetIIIGrade             |
            Self::BatchCompressedKerniteIIIGrade               |
            Self::BatchCompressedVitreousMercoxit           |
            Self::BatchCompressedGoldenOmber                |
            Self::BatchCompressedRichPlagioclase            |
            Self::BatchCompressedViscousPyroxeres           |
            Self::BatchCompressedMassiveScordite            |
            Self::BatchCompressedGleamingSpodumain          |
            Self::BatchCompressedDenseVeldspar              => 110.00,

            Self::BatchCompressedArkonorIVGrade            |
            Self::BatchCompressedBistotIVGrade                |
            Self::BatchCompressedCrokiteIVGrade            |
            Self::BatchCompressedOchreIVGrade                   |
            Self::BatchCompressedGneissIVGrade            |
            Self::BatchCompressedHedbergiteIVGrade         |
            Self::BatchCompressedHemorphiteIVGrade    |
            Self::BatchCompressedJaspetIVGrade           |
            Self::BatchCompressedKerniteIVGrade         |
            Self::BatchCompressedPlatinoidOmber             |
            Self::BatchCompressedSparklingPlagioclase       |
            Self::BatchCompressedOpulentPyroxeres           |
            Self::BatchCompressedGlossyScordite             |
            Self::BatchCompressedDazzlingSpodumain          |
            Self::BatchCompressedStableVeldspar             => 115.00,

            Self::BlueIce                                   |
            Self::BlueIceIVGrade                            |
            Self::CompressedBlueIce                         |
            Self::CompressedBlueIceIVGrade                  |
            Self::IcicleIIGrade                               |
            Self::IcicleIVGradeIIGrade                        |
            Self::CompressedIcicleIIGrade                     |
            Self::CompressedIcicleIVGradeIIGrade              |
            Self::GlacialMass                               |
            Self::GlacialMassIVGrade                        |
            Self::CompressedGlacialMass                     |
            Self::CompressedGlacialMassIVGrade              |
            Self::WhiteGlaze                                |
            Self::WhiteGlazeIVGrade                         |
            Self::CompressedWhiteGlaze                      |
            Self::CompressedWhiteGlazeIVGrade               |
            Self::DarkGlitter                               |
            Self::CompressedDarkGlitter                     |
            Self::Gelidus                                   |
            Self::CompressedGelidus                         |
            Self::GlareCrust                                |
            Self::CompressedGlareCrust                      |
            Self::Krystallos                                |
            Self::CompressedKrystallos                      => 100.00,
        }
    }

    /// adds the compressed and uncompressed variation of an ore
    /// 
    pub fn blacklist(value: i32) -> Vec<i32> {
        match value {
            22    | 62568 => vec![22,    62568],
            17425 | 62569 => vec![17425, 62569],
            17426 | 62570 => vec![17426, 62570],
            46678 | 62571 => vec![46678, 62571],

            52316 | 62576 => vec![52316, 62576],
            56627 | 62577 => vec![56627, 62577],
            56628 | 62578 => vec![56628, 62578],

            1223  | 62564 => vec![1223,  62564],
            17428 | 62565 => vec![17428, 62565],
            17429 | 62566 => vec![17429, 62566],
            46676 | 62567 => vec![46676, 62567],

            1225  | 62560 => vec![1225,  62560],
            17432 | 62561 => vec![17432, 62561],
            17433 | 62562 => vec![17433, 62562],
            46677 | 62563 => vec![46677, 62563],

            1232  | 62556 => vec![1232,  62556],
            17436 | 62557 => vec![17436, 62557],
            17437 | 62558 => vec![17437, 62558],
            46675 | 62559 => vec![46675, 62559],

            74533 | 75287 => vec![74533, 75287],
            74534 | 75288 => vec![74534, 75288],
            74535 | 75289 => vec![74535, 75289],
            74536 | 75290 => vec![74536, 75290],

            74529 | 75283 => vec![74529, 75283],
            74530 | 75284 => vec![74530, 75284],
            74531 | 75285 => vec![74531, 75285],
            74532 | 75286 => vec![74532, 75286],

            1229  | 62552 => vec![1229,  62552],
            17865 | 62553 => vec![17865, 62553],
            17866 | 62554 => vec![17866, 62554],
            46679 | 62555 => vec![46679, 62555],

            81975 | 82316 => vec![81975, 82316],
            81976 | 82317 => vec![81976, 82317],
            81977 | 82318 => vec![81977, 82318],
            81978 | 82319 => vec![81978, 82319],

            21    | 62548 => vec![21,    62548],
            17440 | 62549 => vec![17440, 62549],
            17441 | 62550 => vec![17441, 62550],
            46680 | 62551 => vec![46680, 62551],

            1231  | 62544 => vec![1231,  62544],
            17444 | 62545 => vec![17444, 62545],
            17445 | 62546 => vec![17445, 62546],
            46681 | 62547 => vec![46681, 62547],

            82163 | 82312 => vec![82163, 82312],
            82164 | 82313 => vec![82164, 82313],
            82165 | 82314 => vec![82165, 82314],
            82166 | 82315 => vec![82166, 82315],

            1226  | 62540 => vec![1226,  62540],
            17448 | 62541 => vec![17448, 62541],
            17449 | 62542 => vec![17449, 62542],
            46682 | 62543 => vec![46682, 62543],

            20    | 62536 => vec![20,    62536],
            17452 | 62537 => vec![17452, 62537],
            17453 | 62538 => vec![17453, 62538],
            46683 | 62539 => vec![46683, 62539],

            81900 | 82300 => vec![81900, 82300],
            81901 | 82301 => vec![81901, 82301],
            81902 | 82302 => vec![81902, 82302],
            81903 | 82303 => vec![81903, 82303],

            11396 | 62586 => vec![11396, 62586],
            17869 | 62587 => vec![17869, 62587],
            17870 | 62588 => vec![17870, 62588],

            74521 | 75275 => vec![74521, 75275],
            74522 | 75276 => vec![74522, 75276],
            74523 | 75277 => vec![74523, 75277],
            74524 | 75278 => vec![74524, 75278],

            82016 | 82304 => vec![82016, 82304],
            82017 | 82305 => vec![82017, 82305],
            82018 | 82306 => vec![82018, 82306],
            82019 | 82307 => vec![82019, 82307],

            1227  | 62532 => vec![1227,  62532],
            17867 | 62533 => vec![17867, 62533],
            17868 | 62534 => vec![17868, 62534],
            46684 | 62535 => vec![46684, 62535],

            18    | 62528 => vec![18,    62528],
            17455 | 62529 => vec![17455, 62529],
            17456 | 62530 => vec![17456, 62530],
            46685 | 62531 => vec![46685, 62531],

            1224  | 62524 => vec![1224,  62524],
            17459 | 62525 => vec![17459, 62525],
            17460 | 62526 => vec![17460, 62526],
            46686 | 62527 => vec![46686, 62527],

            52315 | 62579 => vec![52315, 62579],
            56629 | 62580 => vec![56629, 62580],
            56630 | 62581 => vec![56630, 62581],

            1228  | 62520 => vec![1228,  62520],
            17463 | 62521 => vec![17463, 62521],
            17464 | 62522 => vec![17464, 62522],
            46687 | 62523 => vec![46687, 62523],

            19    | 62572 => vec![19,    62572],
            17466 | 62573 => vec![17466, 62573],
            17467 | 62574 => vec![17467, 62574],
            46688 | 62575 => vec![46688, 62575],

            52306 | 62582 => vec![52306, 62582],
            56625 | 62583 => vec![56625, 62583],
            56626 | 62584 => vec![56626, 62584],

            82205 | 82308 => vec![82205, 82308],
            82206 | 82309 => vec![82206, 82309],
            82207 | 82310 => vec![82207, 82310],
            82208 | 82311 => vec![82208, 82311],

            1230  | 62516 => vec![1230,  62516],
            17470 | 62517 => vec![17470, 62517],
            17471 | 62518 => vec![17471, 62518],
            46689 | 62519 => vec![46689, 62519],

            74525 | 75279 => vec![74525, 75279],
            74526 | 75280 => vec![74526, 75280],
            74527 | 75281 => vec![74527, 75281],
            74528 | 75282 => vec![74528, 75282],

            // MoonGoo
            45492 | 62454 => vec![45492, 62454],
            46284 | 62455 => vec![46284, 62455],
            46285 | 62456 => vec![46285, 62456],

            45493 | 62457 => vec![45493, 62457],
            46286 | 62458 => vec![46286, 62458],
            46287 | 62459 => vec![46287, 62459],

            45491 | 62460 => vec![45491, 62460],
            46282 | 62461 => vec![46282, 62461],
            46283 | 62466 => vec![46283, 62466],

            45490 | 62463 => vec![45490, 62463],
            46280 | 62464 => vec![46280, 62464],
            46281 | 62467 => vec![46281, 62467],

            45494 | 62474 => vec![45494, 62474],
            46288 | 62475 => vec![46288, 62475],
            46289 | 62476 => vec![46289, 62476],

            45495 | 62471 => vec![45495, 62471],
            46290 | 62472 => vec![46290, 62472],
            46291 | 62473 => vec![46291, 62473],

            45497 | 62468 => vec![45497, 62468],
            46294 | 62469 => vec![46294, 62469],
            46295 | 62470 => vec![46295, 62470],

            45496 | 62477 => vec![45496, 62477],
            46292 | 62478 => vec![46292, 62478],
            46293 | 62479 => vec![46293, 62479],

            45501 | 62480 => vec![45501, 62480],
            46302 | 62481 => vec![46302, 62481],
            46303 | 62482 => vec![46303, 62482],

            45498 | 62483 => vec![45498, 62483],
            46296 | 62484 => vec![46296, 62484],
            46297 | 62485 => vec![46297, 62485],

            45499 | 62486 => vec![45499, 62486],
            46298 | 62487 => vec![46298, 62487],
            46299 | 62488 => vec![46299, 62488],

            45500 | 62489 => vec![45500, 62489],
            46300 | 62490 => vec![46300, 62490],
            46301 | 62491 => vec![46301, 62491],

            45502 | 62492 => vec![45502, 62492],
            46304 | 62493 => vec![46304, 62493],
            46305 | 62494 => vec![46305, 62494],

            45506 | 62495 => vec![45506, 62495],
            46310 | 62496 => vec![46310, 62496],
            46311 | 62497 => vec![46311, 62497],

            45504 | 62498 => vec![45504, 62498],
            46308 | 62499 => vec![46308, 62499],
            46309 | 62500 => vec![46309, 62500],

            45503 | 62501 => vec![45503, 62501],
            46306 | 62502 => vec![46306, 62502],
            46307 | 62503 => vec![46307, 62503],

            45512 | 62504 => vec![45512, 62504],
            46316 | 62505 => vec![46316, 62505],
            46317 | 62506 => vec![46317, 62506],

            45511 | 62507 => vec![45511, 62507],
            46314 | 62508 => vec![46314, 62508],
            46315 | 62509 => vec![46315, 62509],

            45510 | 62510 => vec![45510, 62510],
            46312 | 62511 => vec![46312, 62511],
            46313 | 62512 => vec![46313, 62512],

            45513 | 62513 => vec![45513, 62513],
            46318 | 62514 => vec![46318, 62514],
            46319 | 62515 => vec![46319, 62515],

            _ => Vec::new(),
        }
    }

    pub fn is_raw(&self) -> bool {
        match self {
            Self::Tritanium |
            Self::Pyerite |
            Self::Mexallon |
            Self::Isogen |
            Self::Nocxium |
            Self::Zydrine |
            Self::Megacyte |
            Self::Morphite |
            Self::AtmosphericGases |
            Self::EvaporiteDeposits |
            Self::Hydrocarbons |
            Self::Silicates |
            Self::Cobalt |
            Self::Scandium |
            Self::Titanium |
            Self::Tungsten |
            Self::Chromium |
            Self::Cadmium |
            Self::Platinum |
            Self::Vanadium |
            Self::Caesium |
            Self::Hafnium |
            Self::Mercury |
            Self::Technetium |
            Self::Promethium |
            Self::Neodymium |
            Self::Dysprosium |
            Self::Thulium => true,
            _ => false
        }
    }

    pub fn is_ore(&self) -> bool {
        match self {
            Self::ArkonorIGrade |
            Self::ArkonorIIGrade |
            Self::ArkonorIIIGrade |
            Self::ArkonorIVGrade |

            Self::BezdnacineIGrade |
            Self::BezdnacineIIGrade |
            Self::BezdnacineIIIGrade |

            Self::BistotIGrade |
            Self::BistotIIGrade |
            Self::BistotIIIGrade |
            Self::BistotIVGrade |

            Self::CrokiteIGrade |
            Self::CrokiteIIGrade |
            Self::CrokiteIIIGrade |
            Self::CrokiteIVGrade |

            Self::DarkOchreIGrade |
            Self::OchreIIGrade |
            Self::OchreIIIGrade |
            Self::OchreIVGrade |

            Self::DuciniumIGrade |
            Self::DuciniumIIGrade |
            Self::DuciniumIIIGrade |
            Self::DuciniumIVGrade |

            Self::EifyriumIGrade |
            Self::EifyriumIIGrade |
            Self::EifyriumIIIGrade |
            Self::EifyriumIVGrade |

            Self::GneissIGrade |
            Self::GneissIIGrade |
            Self::GneissIIIGrade |
            Self::GneissIVGrade |

            Self::GriemeerIGrade |
            Self::GriemeerIIGrade |
            Self::GriemeerIIIGrade |
            Self::GriemeerIVGrade |

            Self::HedbergiteIGrade |
            Self::HedbergiteIIGrade |
            Self::HedbergiteIIIGrade |
            Self::HedbergiteIVGrade |

            Self::HemorphiteIGrade |
            Self::HemorphiteIIGrade |
            Self::HemorphiteIIIGrade |
            Self::HemorphiteIVGrade |

            Self::HezorimeIGrade |
            Self::HezorimeIIGrade |
            Self::HezorimeIIIGrade |
            Self::HezorimeIVGrade |

            Self::JaspetIGrade |
            Self::JaspetIIGrade |
            Self::JaspetIIIGrade |
            Self::JaspetIVGrade |

            Self::KerniteIGrade |
            Self::KerniteIIGrade |
            Self::KerniteIIIGrade |
            Self::KerniteIVGrade |

            Self::KylixiumIGrade |
            Self::KylixiumIIGrade |
            Self::KylixiumIIIGrade |
            Self::KylixiumIVGrade |

            Self::Mercoxit |
            Self::MagmaMercoxit |
            Self::VitreousMercoxit |

            Self::Mordunium |
            Self::PlumMordunium |
            Self::PrizeMordunium |
            Self::PlunderMordunium |

            Self::Nocxite |
            Self::FragrantNocxite |
            Self::IntoxicatingNocxite |
            Self::AmbrosialNocxite |

            Self::Omber |
            Self::SilveryOmber |
            Self::GoldenOmber |
            Self::PlatinoidOmber |

            Self::Plagioclase |
            Self::AzurePlagioclase |
            Self::RichPlagioclase |
            Self::SparklingPlagioclase |

            Self::Pyroxeres |
            Self::SolidPyroxeres |
            Self::ViscousPyroxeres |
            Self::OpulentPyroxeres |

            Self::Rakovene |
            Self::RakoveneIIGrade |
            Self::RakoveneIIIGrade |

            Self::Scordite |
            Self::CondensedScordite |
            Self::MassiveScordite |
            Self::GlossyScordite |

            Self::Spodumain |
            Self::BrightSpodumain |
            Self::GleamingSpodumain |
            Self::DazzlingSpodumain |

            Self::Talassonite |
            Self::TalassoniteIIGrade |
            Self::TalassoniteIIIGrade |

            Self::Ueganite |
            Self::FoggyUeganite |
            Self::OvercastUeganite |
            Self::StormyUeganite |

            Self::Veldspar |
            Self::ConcentratedVeldspar |
            Self::DenseVeldspar |
            Self::StableVeldspar |

            Self::Ytirium |
            Self::BootlegYtirium |
            Self::FirewaterYtirium |
            Self::MoonshineYtirium => true,
            _ => false,
        }
    }

    pub fn is_uncompressed_moon(&self) -> bool {
        match self {
            Self::Bitumens |
            Self::BrimfulBitumens |
            Self::GlisteningBitumens |

            Self::Coesite |
            Self::BrimfulCoesite |
            Self::GlisteningCoesite |

            Self::Sylvite |
            Self::BrimfulSylvite |
            Self::GlisteningSylvite |

            Self::Zeolites |
            Self::BrimfulZeolites |
            Self::GlisteningZeolites |

            Self::Cobaltite |
            Self::CopiousCobaltite |
            Self::TwinklingCobaltite |

            Self::Euxenite |
            Self::CopiousEuxenite |
            Self::TwinklingEuxenite |

            Self::Scheelite |
            Self::CopiousScheelite |
            Self::TwinklingScheelite |

            Self::Titanite |
            Self::CopiousTitanite |
            Self::TwinklingTitanite |

            Self::Chromite |
            Self::LavishChromite |
            Self::ShimmeringChromite |

            Self::Otavite |
            Self::LavishOtavite |
            Self::ShimmeringOtavite |

            Self::Sperrylite |
            Self::LavishSperrylite |
            Self::ShimmeringSperrylite |

            Self::Vanadinite |
            Self::LavishVanadinite |
            Self::ShimmeringVanadinite |

            Self::Carnotite |
            Self::RepleteCarnotite |
            Self::GlowingCarnotite |

            Self::Cinnabar |
            Self::RepleteCinnabar |
            Self::GlowingCinnabar |

            Self::Pollucite |
            Self::RepletePollucite |
            Self::GlowingPollucite |

            Self::Zircon |
            Self::RepleteZircon |
            Self::GlowingZircon |

            Self::Loparite |
            Self::BountifulLoparite |
            Self::ShiningLoparite |

            Self::Monazite |
            Self::BountifulMonazite |
            Self::ShiningMonazite |

            Self::Xenotime |
            Self::BountifulXenotime |
            Self::ShiningXenotime |

            Self::Ytterbite |
            Self::BountifulYtterbite |
            Self::ShiningYtterbite => true,
            _ => false,
        }
    }

    pub fn is_compressed_moon(&self) -> bool {
        match self {
            Self::CompressedBitumens |
            Self::CompressedBrimfulBitumens |
            Self::CompressedGlisteningBitumens |

            Self::CompressedCoesite |
            Self::CompressedBrimfulCoesite |
            Self::CompressedGlisteningCoesite |

            Self::CompressedSylvite |
            Self::CompressedBrimfulSylvite |
            Self::CompressedGlisteningSylvite |

            Self::CompressedZeolites |
            Self::CompressedBrimfulZeolites |
            Self::CompressedGlisteningZeolites |

            Self::CompressedCobaltite |
            Self::CompressedCopiousCobaltite |
            Self::CompressedTwinklingCobaltite |

            Self::CompressedEuxenite |
            Self::CompressedCopiousEuxenite |
            Self::CompressedTwinklingEuxenite |

            Self::CompressedScheelite |
            Self::CompressedCopiousScheelite |
            Self::CompressedTwinklingScheelite |

            Self::CompressedTitanite |
            Self::CompressedCopiousTitanite |
            Self::CompressedTwinklingTitanite |

            Self::CompressedChromite |
            Self::CompressedLavishChromite |
            Self::CompressedShimmeringChromite |

            Self::CompressedOtavite |
            Self::CompressedLavishOtavite |
            Self::CompressedShimmeringOtavite |

            Self::CompressedSperrylite |
            Self::CompressedLavishSperrylite |
            Self::CompressedShimmeringSperrylite |

            Self::CompressedVanadinite |
            Self::CompressedLavishVanadinite |
            Self::CompressedShimmeringVanadinite |

            Self::CompressedCarnotite |
            Self::CompressedRepleteCarnotite |
            Self::CompressedGlowingCarnotite |

            Self::CompressedCinnabar |
            Self::CompressedRepleteCinnabar |
            Self::CompressedGlowingCinnabar |

            Self::CompressedPollucite |
            Self::CompressedRepletePollucite |
            Self::CompressedGlowingPollucite |

            Self::CompressedZircon |
            Self::CompressedRepleteZircon |
            Self::CompressedGlowingZircon |

            Self::CompressedLoparite |
            Self::CompressedBountifulLoparite |
            Self::CompressedShiningLoparite |

            Self::CompressedMonazite |
            Self::CompressedBountifulMonazite |
            Self::CompressedShiningMonazite |

            Self::CompressedXenotime |
            Self::CompressedBountifulXenotime |
            Self::CompressedShiningXenotime |

            Self::CompressedYtterbite |
            Self::CompressedBountifulYtterbite |
            Self::CompressedShiningYtterbite => true,
            _ => false,
        }
    }
}

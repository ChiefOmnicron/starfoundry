use crate::Mineral;
use std::collections::HashMap;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Asteroid {
    Arkonor,
    CrimsonArkonor,
    PrimeArkonor,
    FlawlessArkonor,
    CompressedArkonor,
    CompressedCrimsonArkonor,
    CompressedPrimeArkonor,
    CompressedFlawlessArkonor,

    Bezdnacine,
    AbyssalBezdnacine,
    HadalBezdnacine,
    CompressedBezdnacine,
    CompressedAbyssalBezdnacine,
    CompressedHadalBezdnacine,

    Bistot,
    TriclinicBistot,
    MonoclinicBistot,
    CubicBistot,
    CompressedBistot,
    CompressedTriclinicBistot,
    CompressedMonoclinicBistot,
    CompressedCubicBistot,

    Crokite,
    SharpCrokite,
    CrystallineCrokite,
    PellucidCrokite,
    CompressedCrokite,
    CompressedSharpCrokite,
    CompressedCrystallineCrokite,
    CompressedPellucidCrokite,

    DarkOchre,
    OnyxOchre,
    ObsidianOchre,
    JetOchre,
    CompressedDarkOchre,
    CompressedOnyxOchre,
    CompressedObsidianOchre,
    CompressedJetOchre,

    Ducinium,
    NobleDucinium,
    RoyalDucinium,
    ImperialDucinium,
    CompressedDucinium,
    CompressedNobleDucinium,
    CompressedRoyalDucinium,
    CompressedImperialDucinium,

    Eifyrium,
    DopedEifyrium,
    BoostedEifyrium,
    AugmentedEifyrium,
    CompressedEifyrium,
    CompressedDopedEifyrium,
    CompressedBoostedEifyrium,
    CompressedAugmentedEifyrium,

    Gneiss,
    IridescentGneiss,
    PrismaticGneiss,
    BrilliantGneiss,
    CompressedGneiss,
    CompressedIridescentGneiss,
    CompressedPrismaticGneiss,
    CompressedBrilliantGneiss,

    Griemeer,
    ClearGriemeer,
    InkyGriemeer,
    OpaqueGriemeer,
    CompressedGriemeer,
    CompressedClearGriemeer,
    CompressedInkyGriemeer,
    CompressedOpaqueGriemeer,

    Hedbergite,
    VitricHedbergite,
    GlazedHedbergite,
    LustrousHedbergite,
    CompressedHedbergite,
    CompressedVitricHedbergite,
    CompressedGlazedHedbergite,
    CompressedLustrousHedbergite,

    Hemorphite,
    VividHemorphite,
    RadiantHemorphite,
    ScintillatingHemorphite,
    CompressedHemorphite,
    CompressedVividHemorphite,
    CompressedRadiantHemorphite,
    CompressedScintillatingHemorphite,

    Hezorime,
    DullHezorime,
    SerratedHezorime,
    SharpHezorime,
    CompressedHezorime,
    CompressedDullHezorime,
    CompressedSerratedHezorime,
    CompressedSharpHezorime,

    Jaspet,
    PureJaspet,
    PristineJaspet,
    ImmaculateJaspet,
    CompressedJaspet,
    CompressedPureJaspet,
    CompressedPristineJaspet,
    CompressedImmaculateJaspet,

    Kernite,
    LuminousKernite,
    FieryKernite,
    ResplendantKernite,
    CompressedKernite,
    CompressedLuminousKernite,
    CompressedFieryKernite,
    CompressedResplendantKernite,

    Kylixium,
    KaolinKylixium,
    ArgilKylixium,
    AdobeKylixium,
    CompressedKylixium,
    CompressedKaolinKylixium,
    CompressedArgilKylixium,
    CompressedAdobeKylixium,

    Mercoxit,
    MagmaMercoxit,
    VitreousMercoxit,
    CompressedMercoxit,
    CompressedMagmaMercoxit,
    CompressedVitreousMercoxit,

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

    Plagioclase,
    AzurePlagioclase,
    RichPlagioclase,
    SparklingPlagioclase,
    CompressedPlagioclase,
    CompressedAzurePlagioclase,
    CompressedRichPlagioclase,
    CompressedSparklingPlagioclase,

    Pyroxeres,
    SolidPyroxeres,
    ViscousPyroxeres,
    OpulentPyroxeres,
    CompressedPyroxeres,
    CompressedSolidPyroxeres,
    CompressedViscousPyroxeres,
    CompressedOpulentPyroxeres,

    Rakovene,
    AbyssalRakovene,
    HadalRakovene,
    CompressedRakovene,
    CompressedAbyssalRakovene,
    CompressedHadalRakovene,

    Scordite,
    CondensedScordite,
    MassiveScordite,
    GlossyScordite,
    CompressedScordite,
    CompressedCondensedScordite,
    CompressedMassiveScordite,
    CompressedGlossyScordite,

    Spodumain,
    BrightSpodumain,
    GleamingSpodumain,
    DazzlingSpodumain,
    CompressedSpodumain,
    CompressedBrightSpodumain,
    CompressedGleamingSpodumain,
    CompressedDazzlingSpodumain,

    Talassonite,
    AbyssalTalassonite,
    HadalTalassonite,
    CompressedTalassonite,
    CompressedAbyssalTalassonite,
    CompressedHadalTalassonite,

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
}

impl Asteroid {
    pub fn type_ids() -> Vec<i32> {
        vec![
            Self::Arkonor.to_type_id(),
            Self::CrimsonArkonor.to_type_id(),
            Self::PrimeArkonor.to_type_id(),
            Self::FlawlessArkonor.to_type_id(),
            Self::CompressedArkonor.to_type_id(),
            Self::CompressedCrimsonArkonor.to_type_id(),
            Self::CompressedPrimeArkonor.to_type_id(),
            Self::CompressedFlawlessArkonor.to_type_id(),

            Self::Bezdnacine.to_type_id(),
            Self::AbyssalBezdnacine.to_type_id(),
            Self::HadalBezdnacine.to_type_id(),
            Self::CompressedBezdnacine.to_type_id(),
            Self::CompressedAbyssalBezdnacine.to_type_id(),
            Self::CompressedHadalBezdnacine.to_type_id(),

            Self::Bistot.to_type_id(),
            Self::TriclinicBistot.to_type_id(),
            Self::MonoclinicBistot.to_type_id(),
            Self::CubicBistot.to_type_id(),
            Self::CompressedBistot.to_type_id(),
            Self::CompressedTriclinicBistot.to_type_id(),
            Self::CompressedMonoclinicBistot.to_type_id(),
            Self::CompressedCubicBistot.to_type_id(),

            Self::Crokite.to_type_id(),
            Self::SharpCrokite.to_type_id(),
            Self::CrystallineCrokite.to_type_id(),
            Self::PellucidCrokite.to_type_id(),
            Self::CompressedCrokite.to_type_id(),
            Self::CompressedSharpCrokite.to_type_id(),
            Self::CompressedCrystallineCrokite.to_type_id(),
            Self::CompressedPellucidCrokite.to_type_id(),

            Self::DarkOchre.to_type_id(),
            Self::OnyxOchre.to_type_id(),
            Self::ObsidianOchre.to_type_id(),
            Self::JetOchre.to_type_id(),
            Self::CompressedDarkOchre.to_type_id(),
            Self::CompressedOnyxOchre.to_type_id(),
            Self::CompressedObsidianOchre.to_type_id(),
            Self::CompressedJetOchre.to_type_id(),

            Self::Ducinium.to_type_id(),
            Self::NobleDucinium.to_type_id(),
            Self::RoyalDucinium.to_type_id(),
            Self::ImperialDucinium.to_type_id(),
            Self::CompressedDucinium.to_type_id(),
            Self::CompressedNobleDucinium.to_type_id(),
            Self::CompressedRoyalDucinium.to_type_id(),
            Self::CompressedImperialDucinium.to_type_id(),

            Self::Eifyrium.to_type_id(),
            Self::DopedEifyrium.to_type_id(),
            Self::BoostedEifyrium.to_type_id(),
            Self::AugmentedEifyrium.to_type_id(),
            Self::CompressedEifyrium.to_type_id(),
            Self::CompressedDopedEifyrium.to_type_id(),
            Self::CompressedBoostedEifyrium.to_type_id(),
            Self::CompressedAugmentedEifyrium.to_type_id(),

            Self::Gneiss.to_type_id(),
            Self::IridescentGneiss.to_type_id(),
            Self::PrismaticGneiss.to_type_id(),
            Self::BrilliantGneiss.to_type_id(),
            Self::CompressedGneiss.to_type_id(),
            Self::CompressedIridescentGneiss.to_type_id(),
            Self::CompressedPrismaticGneiss.to_type_id(),
            Self::CompressedBrilliantGneiss.to_type_id(),

            Self::Griemeer.to_type_id(),
            Self::ClearGriemeer.to_type_id(),
            Self::InkyGriemeer.to_type_id(),
            Self::OpaqueGriemeer.to_type_id(),
            Self::CompressedGriemeer.to_type_id(),
            Self::CompressedClearGriemeer.to_type_id(),
            Self::CompressedInkyGriemeer.to_type_id(),
            Self::CompressedOpaqueGriemeer.to_type_id(),

            Self::Hedbergite.to_type_id(),
            Self::VitricHedbergite.to_type_id(),
            Self::GlazedHedbergite.to_type_id(),
            Self::LustrousHedbergite.to_type_id(),
            Self::CompressedHedbergite.to_type_id(),
            Self::CompressedVitricHedbergite.to_type_id(),
            Self::CompressedGlazedHedbergite.to_type_id(),
            Self::CompressedLustrousHedbergite.to_type_id(),

            Self::Hemorphite.to_type_id(),
            Self::VividHemorphite.to_type_id(),
            Self::RadiantHemorphite.to_type_id(),
            Self::ScintillatingHemorphite.to_type_id(),
            Self::CompressedHemorphite.to_type_id(),
            Self::CompressedVividHemorphite.to_type_id(),
            Self::CompressedRadiantHemorphite.to_type_id(),
            Self::CompressedScintillatingHemorphite.to_type_id(),

            Self::Hezorime.to_type_id(),
            Self::DullHezorime.to_type_id(),
            Self::SerratedHezorime.to_type_id(),
            Self::SharpHezorime.to_type_id(),
            Self::CompressedHezorime.to_type_id(),
            Self::CompressedDullHezorime.to_type_id(),
            Self::CompressedSerratedHezorime.to_type_id(),
            Self::CompressedSharpHezorime.to_type_id(),

            Self::Jaspet.to_type_id(),
            Self::PureJaspet.to_type_id(),
            Self::PristineJaspet.to_type_id(),
            Self::ImmaculateJaspet.to_type_id(),
            Self::CompressedJaspet.to_type_id(),
            Self::CompressedPureJaspet.to_type_id(),
            Self::CompressedPristineJaspet.to_type_id(),
            Self::CompressedImmaculateJaspet.to_type_id(),

            Self::Kernite.to_type_id(),
            Self::LuminousKernite.to_type_id(),
            Self::FieryKernite.to_type_id(),
            Self::ResplendantKernite.to_type_id(),
            Self::CompressedKernite.to_type_id(),
            Self::CompressedLuminousKernite.to_type_id(),
            Self::CompressedFieryKernite.to_type_id(),
            Self::CompressedResplendantKernite.to_type_id(),

            Self::Kylixium.to_type_id(),
            Self::KaolinKylixium.to_type_id(),
            Self::ArgilKylixium.to_type_id(),
            Self::AdobeKylixium.to_type_id(),
            Self::CompressedKylixium.to_type_id(),
            Self::CompressedKaolinKylixium.to_type_id(),
            Self::CompressedArgilKylixium.to_type_id(),
            Self::CompressedAdobeKylixium.to_type_id(),

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
            Self::AbyssalRakovene.to_type_id(),
            Self::HadalRakovene.to_type_id(),
            Self::CompressedRakovene.to_type_id(),
            Self::CompressedAbyssalRakovene.to_type_id(),
            Self::CompressedHadalRakovene.to_type_id(),

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
            Self::AbyssalTalassonite.to_type_id(),
            Self::HadalTalassonite.to_type_id(),
            Self::CompressedTalassonite.to_type_id(),
            Self::CompressedAbyssalTalassonite.to_type_id(),
            Self::CompressedHadalTalassonite.to_type_id(),

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
        ]
    }

    pub fn to_type_id(&self) -> i32 {
        match self {
            Self::Arkonor                               => 22,
            Self::CrimsonArkonor                        => 17425,
            Self::PrimeArkonor                          => 17426,
            Self::FlawlessArkonor                       => 46678,
            Self::CompressedArkonor                     => 62568,
            Self::CompressedCrimsonArkonor              => 62569,
            Self::CompressedPrimeArkonor                => 62570,
            Self::CompressedFlawlessArkonor             => 62571,

            Self::Bezdnacine                            => 52316,
            Self::AbyssalBezdnacine                     => 56627,
            Self::HadalBezdnacine                       => 56628,
            Self::CompressedBezdnacine                  => 62576,
            Self::CompressedAbyssalBezdnacine           => 62577,
            Self::CompressedHadalBezdnacine             => 62578,

            Self::Bistot                                => 1223,
            Self::TriclinicBistot                       => 17428,
            Self::MonoclinicBistot                      => 17429,
            Self::CubicBistot                           => 46676,
            Self::CompressedBistot                      => 62564,
            Self::CompressedTriclinicBistot             => 62565,
            Self::CompressedMonoclinicBistot            => 62566,
            Self::CompressedCubicBistot                 => 62567,

            Self::Crokite                               => 1225,
            Self::SharpCrokite                          => 17432,
            Self::CrystallineCrokite                    => 17433,
            Self::PellucidCrokite                       => 46677,
            Self::CompressedCrokite                     => 62560,
            Self::CompressedSharpCrokite                => 62561,
            Self::CompressedCrystallineCrokite          => 62562,
            Self::CompressedPellucidCrokite             => 62563,

            Self::DarkOchre                             => 1232,
            Self::OnyxOchre                             => 17436,
            Self::ObsidianOchre                         => 17437,
            Self::JetOchre                              => 46675,
            Self::CompressedDarkOchre                   => 62556,
            Self::CompressedOnyxOchre                   => 62557,
            Self::CompressedObsidianOchre               => 62558,
            Self::CompressedJetOchre                    => 62559,

            Self::Ducinium                              => 74533,
            Self::NobleDucinium                         => 74534,
            Self::RoyalDucinium                         => 74535,
            Self::ImperialDucinium                      => 74536,
            Self::CompressedDucinium                    => 75287,
            Self::CompressedNobleDucinium               => 75288,
            Self::CompressedRoyalDucinium               => 75289,
            Self::CompressedImperialDucinium            => 75290,

            Self::Eifyrium                              => 74529,
            Self::DopedEifyrium                         => 74530,
            Self::BoostedEifyrium                       => 74531,
            Self::AugmentedEifyrium                     => 74532,
            Self::CompressedEifyrium                    => 75283,
            Self::CompressedDopedEifyrium               => 75284,
            Self::CompressedBoostedEifyrium             => 75285,
            Self::CompressedAugmentedEifyrium           => 75286,

            Self::Gneiss                                => 1229,
            Self::IridescentGneiss                      => 17865,
            Self::PrismaticGneiss                       => 17866,
            Self::BrilliantGneiss                       => 46679,
            Self::CompressedGneiss                      => 62552,
            Self::CompressedIridescentGneiss            => 62553,
            Self::CompressedPrismaticGneiss             => 62554,
            Self::CompressedBrilliantGneiss             => 62555,

            Self::Griemeer                              => 81975,
            Self::ClearGriemeer                         => 81976,
            Self::InkyGriemeer                          => 81977,
            Self::OpaqueGriemeer                        => 81978,
            Self::CompressedGriemeer                    => 82316,
            Self::CompressedClearGriemeer               => 82317,
            Self::CompressedInkyGriemeer                => 82318,
            Self::CompressedOpaqueGriemeer              => 82319,

            Self::Hedbergite                            => 21,
            Self::VitricHedbergite                      => 17440,
            Self::GlazedHedbergite                      => 17441,
            Self::LustrousHedbergite                    => 46680,
            Self::CompressedHedbergite                  => 62548,
            Self::CompressedVitricHedbergite            => 62549,
            Self::CompressedGlazedHedbergite            => 62550,
            Self::CompressedLustrousHedbergite          => 62551,

            Self::Hemorphite                            => 1231,
            Self::VividHemorphite                       => 17444,
            Self::RadiantHemorphite                     => 17445,
            Self::ScintillatingHemorphite               => 46681,
            Self::CompressedHemorphite                  => 62544,
            Self::CompressedVividHemorphite             => 62545,
            Self::CompressedRadiantHemorphite           => 62546,
            Self::CompressedScintillatingHemorphite     => 62547,

            Self::Hezorime                              => 82163,
            Self::DullHezorime                          => 82164,
            Self::SerratedHezorime                      => 82165,
            Self::SharpHezorime                         => 82166,
            Self::CompressedHezorime                    => 82312,
            Self::CompressedDullHezorime                => 82313,
            Self::CompressedSerratedHezorime            => 82314,
            Self::CompressedSharpHezorime               => 82315,

            Self::Jaspet                                => 1226,
            Self::PureJaspet                            => 17448,
            Self::PristineJaspet                        => 17449,
            Self::ImmaculateJaspet                      => 46682,
            Self::CompressedJaspet                      => 62540,
            Self::CompressedPureJaspet                  => 62541,
            Self::CompressedPristineJaspet              => 62542,
            Self::CompressedImmaculateJaspet            => 62543,

            Self::Kernite                               => 20,
            Self::LuminousKernite                       => 17452,
            Self::FieryKernite                          => 17453,
            Self::ResplendantKernite                    => 46683,
            Self::CompressedKernite                     => 62536,
            Self::CompressedLuminousKernite             => 62537,
            Self::CompressedFieryKernite                => 62538,
            Self::CompressedResplendantKernite          => 62539,

            Self::Kylixium                              => 81900,
            Self::KaolinKylixium                        => 81901,
            Self::ArgilKylixium                         => 81902,
            Self::AdobeKylixium                         => 81903,
            Self::CompressedKylixium                    => 82300,
            Self::CompressedKaolinKylixium              => 82301,
            Self::CompressedArgilKylixium               => 82302,
            Self::CompressedAdobeKylixium               => 82303,

            Self::Mercoxit                              => 11396,
            Self::MagmaMercoxit                         => 17869,
            Self::VitreousMercoxit                      => 17870,
            Self::CompressedMercoxit                    => 62586,
            Self::CompressedMagmaMercoxit               => 62587,
            Self::CompressedVitreousMercoxit            => 62588,

            Self::Mordunium                             => 74521,
            Self::PlumMordunium                         => 74522,
            Self::PrizeMordunium                        => 74523,
            Self::PlunderMordunium                      => 74524,
            Self::CompressedMordunium                   => 75275,
            Self::CompressedPlumMordunium               => 75276,
            Self::CompressedPrizeMordunium              => 75277,
            Self::CompressedPlunderMordunium            => 75278,

            Self::Nocxite                               => 82016,
            Self::FragrantNocxite                       => 82017,
            Self::IntoxicatingNocxite                   => 82018,
            Self::AmbrosialNocxite                      => 82019,
            Self::CompressedNocxite                     => 82304,
            Self::CompressedFragrantNocxite             => 82305,
            Self::CompressedIntoxicatingNocxite         => 82306,
            Self::CompressedAmbrosialNocxite            => 82307,

            Self::Omber                                 => 1227,
            Self::SilveryOmber                          => 17867,
            Self::GoldenOmber                           => 17868,
            Self::PlatinoidOmber                        => 46684,
            Self::CompressedOmber                       => 62532,
            Self::CompressedSilveryOmber                => 62533,
            Self::CompressedGoldenOmber                 => 62534,
            Self::CompressedPlatinoidOmber              => 62535,

            Self::Plagioclase                           => 18,
            Self::AzurePlagioclase                      => 17455,
            Self::RichPlagioclase                       => 17456,
            Self::SparklingPlagioclase                  => 46685,
            Self::CompressedPlagioclase                 => 62528,
            Self::CompressedAzurePlagioclase            => 62529,
            Self::CompressedRichPlagioclase             => 62530,
            Self::CompressedSparklingPlagioclase        => 62531,

            Self::Pyroxeres                             => 1224,
            Self::SolidPyroxeres                        => 17459,
            Self::ViscousPyroxeres                      => 17460,
            Self::OpulentPyroxeres                      => 46686,
            Self::CompressedPyroxeres                   => 62524,
            Self::CompressedSolidPyroxeres              => 62525,
            Self::CompressedViscousPyroxeres            => 62526,
            Self::CompressedOpulentPyroxeres            => 62527,

            Self::Rakovene                              => 52315,
            Self::AbyssalRakovene                       => 56629,
            Self::HadalRakovene                         => 56630,
            Self::CompressedRakovene                    => 62579,
            Self::CompressedAbyssalRakovene             => 62580,
            Self::CompressedHadalRakovene               => 62581,

            Self::Scordite                              => 1228,
            Self::CondensedScordite                     => 17463,
            Self::MassiveScordite                       => 17464,
            Self::GlossyScordite                        => 46687,
            Self::CompressedScordite                    => 62520,
            Self::CompressedCondensedScordite           => 62521,
            Self::CompressedMassiveScordite             => 62522,
            Self::CompressedGlossyScordite              => 62523,

            Self::Spodumain                             => 19,
            Self::BrightSpodumain                       => 17466,
            Self::GleamingSpodumain                     => 17467,
            Self::DazzlingSpodumain                     => 46688,
            Self::CompressedSpodumain                   => 62572,
            Self::CompressedBrightSpodumain             => 62573,
            Self::CompressedGleamingSpodumain           => 62574,
            Self::CompressedDazzlingSpodumain           => 62575,

            Self::Talassonite                           => 52306,
            Self::AbyssalTalassonite                    => 56625,
            Self::HadalTalassonite                      => 56626,
            Self::CompressedTalassonite                 => 62582,
            Self::CompressedAbyssalTalassonite          => 62583,
            Self::CompressedHadalTalassonite            => 62584,

            Self::Ueganite                              => 82205,
            Self::FoggyUeganite                         => 82206,
            Self::OvercastUeganite                      => 82207,
            Self::StormyUeganite                        => 82208,
            Self::CompressedUeganite                    => 82308,
            Self::CompressedFoggyUeganite               => 82309,
            Self::CompressedOvercastUeganite            => 82310,
            Self::CompressedStormyUeganite              => 82311,

            Self::Veldspar                              => 1230,
            Self::ConcentratedVeldspar                  => 17470,
            Self::DenseVeldspar                         => 17471,
            Self::StableVeldspar                        => 46689,
            Self::CompressedVeldspar                    => 62516,
            Self::CompressedConcentratedVeldspar        => 62517,
            Self::CompressedDenseVeldspar               => 62518,
            Self::CompressedStableVeldspar              => 62519,

            Self::Ytirium                               => 74525,
            Self::BootlegYtirium                        => 74526,
            Self::FirewaterYtirium                      => 74527,
            Self::MoonshineYtirium                      => 74528,
            Self::CompressedYtirium                     => 75279,
            Self::CompressedBootlegYtirium              => 75280,
            Self::CompressedFirewaterYtirium            => 75281,
            Self::CompressedMoonshineYtirium            => 75282,

            // MoonGoo
            Self::Bitumens                              => 45492,
            Self::BrimfulBitumens                       => 46284,
            Self::GlisteningBitumens                    => 46285,
            Self::CompressedBitumens                    => 62454,
            Self::CompressedBrimfulBitumens             => 62455,
            Self::CompressedGlisteningBitumens          => 62456,

            Self::Coesite                               => 45493,
            Self::BrimfulCoesite                        => 46286,
            Self::GlisteningCoesite                     => 46287,
            Self::CompressedCoesite                     => 62457,
            Self::CompressedBrimfulCoesite              => 62458,
            Self::CompressedGlisteningCoesite           => 62459,

            Self::Sylvite                               => 45491,
            Self::BrimfulSylvite                        => 46282,
            Self::GlisteningSylvite                     => 46283,
            Self::CompressedSylvite                     => 62460,
            Self::CompressedBrimfulSylvite              => 62461,
            Self::CompressedGlisteningSylvite           => 62466,

            Self::Zeolites                              => 45490,
            Self::BrimfulZeolites                       => 46280,
            Self::GlisteningZeolites                    => 46281,
            Self::CompressedZeolites                    => 62463,
            Self::CompressedBrimfulZeolites             => 62464,
            Self::CompressedGlisteningZeolites          => 62467,

            Self::Cobaltite                             => 45494,
            Self::CopiousCobaltite                      => 46288,
            Self::TwinklingCobaltite                    => 46289,
            Self::CompressedCobaltite                   => 62474,
            Self::CompressedCopiousCobaltite            => 62475,
            Self::CompressedTwinklingCobaltite          => 62476,

            Self::Euxenite                              => 45495,
            Self::CopiousEuxenite                       => 46290,
            Self::TwinklingEuxenite                     => 46291,
            Self::CompressedEuxenite                    => 62471,
            Self::CompressedCopiousEuxenite             => 62472,
            Self::CompressedTwinklingEuxenite           => 62473,

            Self::Scheelite                             => 45497,
            Self::CopiousScheelite                      => 46294,
            Self::TwinklingScheelite                    => 46295,
            Self::CompressedScheelite                   => 62468,
            Self::CompressedCopiousScheelite            => 62469,
            Self::CompressedTwinklingScheelite          => 62470,

            Self::Titanite                              => 45496,
            Self::CopiousTitanite                       => 46292,
            Self::TwinklingTitanite                     => 46293,
            Self::CompressedTitanite                    => 62477,
            Self::CompressedCopiousTitanite             => 62478,
            Self::CompressedTwinklingTitanite           => 62479,

            Self::Chromite                              => 45501,
            Self::LavishChromite                        => 46302,
            Self::ShimmeringChromite                    => 46303,
            Self::CompressedChromite                    => 62480,
            Self::CompressedLavishChromite              => 62481,
            Self::CompressedShimmeringChromite          => 62482,

            Self::Otavite                               => 45498,
            Self::LavishOtavite                         => 46296,
            Self::ShimmeringOtavite                     => 46297,
            Self::CompressedOtavite                     => 62483,
            Self::CompressedLavishOtavite               => 62484,
            Self::CompressedShimmeringOtavite           => 62485,

            Self::Sperrylite                            => 45499,
            Self::LavishSperrylite                      => 46298,
            Self::ShimmeringSperrylite                  => 46299,
            Self::CompressedSperrylite                  => 62486,
            Self::CompressedLavishSperrylite            => 62487,
            Self::CompressedShimmeringSperrylite        => 62488,

            Self::Vanadinite                            => 45500,
            Self::LavishVanadinite                      => 46300,
            Self::ShimmeringVanadinite                  => 46301,
            Self::CompressedVanadinite                  => 62489,
            Self::CompressedLavishVanadinite            => 62490,
            Self::CompressedShimmeringVanadinite        => 62491,

            Self::Carnotite                             => 45502,
            Self::RepleteCarnotite                      => 46304,
            Self::GlowingCarnotite                      => 46305,
            Self::CompressedCarnotite                   => 62492,
            Self::CompressedRepleteCarnotite            => 62493,
            Self::CompressedGlowingCarnotite            => 62494,

            Self::Cinnabar                              => 45506,
            Self::RepleteCinnabar                       => 46310,
            Self::GlowingCinnabar                       => 46311,
            Self::CompressedCinnabar                    => 62495,
            Self::CompressedRepleteCinnabar             => 62496,
            Self::CompressedGlowingCinnabar             => 62497,

            Self::Pollucite                             => 45504,
            Self::RepletePollucite                      => 46308,
            Self::GlowingPollucite                      => 46309,
            Self::CompressedPollucite                   => 62498,
            Self::CompressedRepletePollucite            => 62499,
            Self::CompressedGlowingPollucite            => 62500,

            Self::Zircon                                => 45503,
            Self::RepleteZircon                         => 46306,
            Self::GlowingZircon                         => 46307,
            Self::CompressedZircon                      => 62501,
            Self::CompressedRepleteZircon               => 62502,
            Self::CompressedGlowingZircon               => 62503,

            Self::Loparite                              => 45512,
            Self::BountifulLoparite                     => 46316,
            Self::ShiningLoparite                       => 46317,
            Self::CompressedLoparite                    => 62504,
            Self::CompressedBountifulLoparite           => 62505,
            Self::CompressedShiningLoparite             => 62506,

            Self::Monazite                              => 45511,
            Self::BountifulMonazite                     => 46314,
            Self::ShiningMonazite                       => 46315,
            Self::CompressedMonazite                    => 62507,
            Self::CompressedBountifulMonazite           => 62508,
            Self::CompressedShiningMonazite             => 62509,

            Self::Xenotime                              => 45510,
            Self::BountifulXenotime                     => 46312,
            Self::ShiningXenotime                       => 46313,
            Self::CompressedXenotime                    => 62510,
            Self::CompressedBountifulXenotime           => 62511,
            Self::CompressedShiningXenotime             => 62512,

            Self::Ytterbite                             => 45513,
            Self::BountifulYtterbite                    => 46318,
            Self::ShiningYtterbite                      => 46319,
            Self::CompressedYtterbite                   => 62513,
            Self::CompressedBountifulYtterbite          => 62514,
            Self::CompressedShiningYtterbite            => 62515,

            Self::Tritanium                             => 34,
            Self::Pyerite                               => 35,
            Self::Mexallon                              => 36,
            Self::Isogen                                => 37,
            Self::Nocxium                               => 38,
            Self::Zydrine                               => 39,
            Self::Megacyte                              => 40,
            Self::Morphite                              => 11399,

            Self::AtmosphericGases                      => 16634,
            Self::EvaporiteDeposits                     => 16635,
            Self::Hydrocarbons                          => 16633,
            Self::Silicates                             => 16636,
            Self::Cobalt                                => 16640,
            Self::Scandium                              => 16639,
            Self::Titanium                              => 16638,
            Self::Tungsten                              => 16637,
            Self::Chromium                              => 16641,
            Self::Cadmium                               => 16643,
            Self::Platinum                              => 16644,
            Self::Vanadium                              => 16642,
            Self::Caesium                               => 16647,
            Self::Hafnium                               => 16648,
            Self::Mercury                               => 16646,
            Self::Technetium                            => 16649,
            Self::Promethium                            => 16652,
            Self::Neodymium                             => 16651,
            Self::Dysprosium                            => 16650,
            Self::Thulium                               => 16653,
        }
    }

    pub fn from_type_id(value: i32) -> Self {
        match value {
            22    => Self::Arkonor,
            17425 => Self::CrimsonArkonor,
            17426 => Self::PrimeArkonor,
            46678 => Self::FlawlessArkonor,
            62568 => Self::CompressedArkonor,
            62569 => Self::CompressedCrimsonArkonor,
            62570 => Self::CompressedPrimeArkonor,
            62571 => Self::CompressedFlawlessArkonor,

            52316 => Self::Bezdnacine,
            56627 => Self::AbyssalBezdnacine,
            56628 => Self::HadalBezdnacine,
            62576 => Self::CompressedBezdnacine,
            62577 => Self::CompressedAbyssalBezdnacine,
            62578 => Self::CompressedHadalBezdnacine,

            1223  => Self::Bistot,
            17428 => Self::TriclinicBistot,
            17429 => Self::MonoclinicBistot,
            46676 => Self::CubicBistot,
            62564 => Self::CompressedBistot,
            62565 => Self::CompressedTriclinicBistot,
            62566 => Self::CompressedMonoclinicBistot,
            62567 => Self::CompressedCubicBistot,

            1225  => Self::Crokite,
            17432 => Self::SharpCrokite,
            17433 => Self::CrystallineCrokite,
            46677 => Self::PellucidCrokite,
            62560 => Self::CompressedCrokite,
            62561 => Self::CompressedSharpCrokite,
            62562 => Self::CompressedCrystallineCrokite,
            62563 => Self::CompressedPellucidCrokite,

            1232  => Self::DarkOchre,
            17436 => Self::OnyxOchre,
            17437 => Self::ObsidianOchre,
            46675 => Self::JetOchre,
            62556 => Self::CompressedDarkOchre,
            62557 => Self::CompressedOnyxOchre,
            62558 => Self::CompressedObsidianOchre,
            62559 => Self::CompressedJetOchre,

            74533 => Self::Ducinium,
            74534 => Self::NobleDucinium,
            74535 => Self::RoyalDucinium,
            74536 => Self::ImperialDucinium,
            75287 => Self::CompressedDucinium,
            75288 => Self::CompressedNobleDucinium,
            75289 => Self::CompressedRoyalDucinium,
            75290 => Self::CompressedImperialDucinium,

            74529 => Self::Eifyrium,
            74530 => Self::DopedEifyrium,
            74531 => Self::BoostedEifyrium,
            74532 => Self::AugmentedEifyrium,
            75283 => Self::CompressedEifyrium,
            75284 => Self::CompressedDopedEifyrium,
            75285 => Self::CompressedBoostedEifyrium,
            75286 => Self::CompressedAugmentedEifyrium,

            1229  => Self::Gneiss,
            17865 => Self::IridescentGneiss,
            17866 => Self::PrismaticGneiss,
            46679 => Self::BrilliantGneiss,
            62552 => Self::CompressedGneiss,
            62553 => Self::CompressedIridescentGneiss,
            62554 => Self::CompressedPrismaticGneiss,
            62555 => Self::CompressedBrilliantGneiss,

            81975 => Self::Griemeer,
            81976 => Self::ClearGriemeer,
            81977 => Self::InkyGriemeer,
            81978 => Self::OpaqueGriemeer,
            82316 => Self::CompressedGriemeer,
            82317 => Self::CompressedClearGriemeer,
            82318 => Self::CompressedInkyGriemeer,
            82319 => Self::CompressedOpaqueGriemeer,

            21    => Self::Hedbergite,
            17440 => Self::VitricHedbergite,
            17441 => Self::GlazedHedbergite,
            46680 => Self::LustrousHedbergite,
            62548 => Self::CompressedHedbergite,
            62549 => Self::CompressedVitricHedbergite,
            62550 => Self::CompressedGlazedHedbergite,
            62551 => Self::CompressedLustrousHedbergite,

            1231  => Self::Hemorphite,
            17444 => Self::VividHemorphite,
            17445 => Self::RadiantHemorphite,
            46681 => Self::ScintillatingHemorphite,
            62544 => Self::CompressedHemorphite,
            62545 => Self::CompressedVividHemorphite,
            62546 => Self::CompressedRadiantHemorphite,
            62547 => Self::CompressedScintillatingHemorphite,

            82163 => Self::Hezorime,
            82164 => Self::DullHezorime,
            82165 => Self::SerratedHezorime,
            82166 => Self::SharpHezorime,
            82312 => Self::CompressedHezorime,
            82313 => Self::CompressedDullHezorime,
            82314 => Self::CompressedSerratedHezorime,
            82315 => Self::CompressedSharpHezorime,

            1226  => Self::Jaspet,
            17448 => Self::PureJaspet,
            17449 => Self::PristineJaspet,
            46682 => Self::ImmaculateJaspet,
            62540 => Self::CompressedJaspet,
            62541 => Self::CompressedPureJaspet,
            62542 => Self::CompressedPristineJaspet,
            62543 => Self::CompressedImmaculateJaspet,

            20    => Self::Kernite,
            17452 => Self::LuminousKernite,
            17453 => Self::FieryKernite,
            46683 => Self::ResplendantKernite,
            62536 => Self::CompressedKernite,
            62537 => Self::CompressedLuminousKernite,
            62538 => Self::CompressedFieryKernite,
            62539 => Self::CompressedResplendantKernite,

            81900 => Self::Kylixium,
            81901 => Self::KaolinKylixium,
            81902 => Self::ArgilKylixium,
            81903 => Self::AdobeKylixium,
            82300 => Self::CompressedKylixium,
            82301 => Self::CompressedKaolinKylixium,
            82302 => Self::CompressedArgilKylixium,
            82303 => Self::CompressedAdobeKylixium,

            11396 => Self::Mercoxit,
            17869 => Self::MagmaMercoxit,
            17870 => Self::VitreousMercoxit,
            62586 => Self::CompressedMercoxit,
            62587 => Self::CompressedMagmaMercoxit,
            62588 => Self::CompressedVitreousMercoxit,

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

            18    => Self::Plagioclase,
            17455 => Self::AzurePlagioclase,
            17456 => Self::RichPlagioclase,
            46685 => Self::SparklingPlagioclase,
            62528 => Self::CompressedPlagioclase,
            62529 => Self::CompressedAzurePlagioclase,
            62530 => Self::CompressedRichPlagioclase,
            62531 => Self::CompressedSparklingPlagioclase,

            1224  => Self::Pyroxeres,
            17459 => Self::SolidPyroxeres,
            17460 => Self::ViscousPyroxeres,
            46686 => Self::OpulentPyroxeres,
            62524 => Self::CompressedPyroxeres,
            62525 => Self::CompressedSolidPyroxeres,
            62526 => Self::CompressedViscousPyroxeres,
            62527 => Self::CompressedOpulentPyroxeres,

            52315 => Self::Rakovene,
            56629 => Self::AbyssalRakovene,
            56630 => Self::HadalRakovene,
            62579 => Self::CompressedRakovene,
            62580 => Self::CompressedAbyssalRakovene,
            62581 => Self::CompressedHadalRakovene,

            1228  => Self::Scordite,
            17463 => Self::CondensedScordite,
            17464 => Self::MassiveScordite,
            46687 => Self::GlossyScordite,
            62520 => Self::CompressedScordite,
            62521 => Self::CompressedCondensedScordite,
            62522 => Self::CompressedMassiveScordite,
            62523 => Self::CompressedGlossyScordite,

            19    => Self::Spodumain,
            17466 => Self::BrightSpodumain,
            17467 => Self::GleamingSpodumain,
            46688 => Self::DazzlingSpodumain,
            62572 => Self::CompressedSpodumain,
            62573 => Self::CompressedBrightSpodumain,
            62574 => Self::CompressedGleamingSpodumain,
            62575 => Self::CompressedDazzlingSpodumain,

            52306 => Self::Talassonite,
            56625 => Self::AbyssalTalassonite,
            56626 => Self::HadalTalassonite,
            62582 => Self::CompressedTalassonite,
            62583 => Self::CompressedAbyssalTalassonite,
            62584 => Self::CompressedHadalTalassonite,

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
            Self::Arkonor                               |
            Self::CrimsonArkonor                        |
            Self::PrimeArkonor                          |
            Self::FlawlessArkonor                       |
            Self::CompressedArkonor                     |
            Self::CompressedCrimsonArkonor              |
            Self::CompressedPrimeArkonor                |
            Self::CompressedFlawlessArkonor             => {
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

            Self::Bezdnacine                            |
            Self::AbyssalBezdnacine                     |
            Self::HadalBezdnacine                       |
            Self::CompressedBezdnacine                  |
            Self::CompressedAbyssalBezdnacine           |
            Self::CompressedHadalBezdnacine             => {
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

            Self::Bistot                                |
            Self::TriclinicBistot                       |
            Self::MonoclinicBistot                      |
            Self::CubicBistot                           |
            Self::CompressedBistot                      |
            Self::CompressedTriclinicBistot             |
            Self::CompressedMonoclinicBistot            |
            Self::CompressedCubicBistot                 => {
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

            Self::Crokite                               |
            Self::SharpCrokite                          |
            Self::CrystallineCrokite                    |
            Self::PellucidCrokite                       |
            Self::CompressedCrokite                     |
            Self::CompressedSharpCrokite                |
            Self::CompressedCrystallineCrokite          |
            Self::CompressedPellucidCrokite             => {
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

            Self::DarkOchre                             |
            Self::OnyxOchre                             |
            Self::ObsidianOchre                         |
            Self::JetOchre                              |
            Self::CompressedDarkOchre                   |
            Self::CompressedOnyxOchre                   |
            Self::CompressedObsidianOchre               |
            Self::CompressedJetOchre                    => {
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

            Self::Ducinium                              |
            Self::NobleDucinium                         |
            Self::RoyalDucinium                         |
            Self::ImperialDucinium                      |
            Self::CompressedDucinium                    |
            Self::CompressedNobleDucinium               |
            Self::CompressedRoyalDucinium               |
            Self::CompressedImperialDucinium            => {
                mineral_init(
                    vec![
                        Mineral::Megacyte,
                    ],
                    vec![
                        170f64 * self.ore_modifier(),
                    ],
                )
            },

            Self::Eifyrium                              |
            Self::DopedEifyrium                         |
            Self::BoostedEifyrium                       |
            Self::AugmentedEifyrium                     |
            Self::CompressedEifyrium                    |
            Self::CompressedDopedEifyrium               |
            Self::CompressedBoostedEifyrium             |
            Self::CompressedAugmentedEifyrium           => {
                mineral_init(
                    vec![
                        Mineral::Zydrine,
                    ],
                    vec![
                        266f64 * self.ore_modifier(),
                    ],
                )
            },

            Self::Gneiss                                |
            Self::IridescentGneiss                      |
            Self::PrismaticGneiss                       |
            Self::BrilliantGneiss                       |
            Self::CompressedGneiss                      |
            Self::CompressedIridescentGneiss            |
            Self::CompressedPrismaticGneiss             |
            Self::CompressedBrilliantGneiss             => {
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

            Self::Griemeer                              |
            Self::ClearGriemeer                         |
            Self::InkyGriemeer                          |
            Self::OpaqueGriemeer                        |
            Self::CompressedGriemeer                    |
            Self::CompressedClearGriemeer               |
            Self::CompressedInkyGriemeer                |
            Self::CompressedOpaqueGriemeer              => {
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

            Self::Hedbergite                            |
            Self::VitricHedbergite                      |
            Self::GlazedHedbergite                      |
            Self::LustrousHedbergite                    |
            Self::CompressedHedbergite                  |
            Self::CompressedVitricHedbergite            |
            Self::CompressedGlazedHedbergite            |
            Self::CompressedLustrousHedbergite          => {
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

            Self::Hemorphite                            |
            Self::VividHemorphite                       |
            Self::RadiantHemorphite                     |
            Self::ScintillatingHemorphite               |
            Self::CompressedHemorphite                  |
            Self::CompressedVividHemorphite             |
            Self::CompressedRadiantHemorphite           |
            Self::CompressedScintillatingHemorphite     => {
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

            Self::Hezorime                              |
            Self::DullHezorime                          |
            Self::SerratedHezorime                      |
            Self::SharpHezorime                         |
            Self::CompressedHezorime                    |
            Self::CompressedDullHezorime                |
            Self::CompressedSerratedHezorime            |
            Self::CompressedSharpHezorime               => {
                mineral_init(
                    vec![
                        Mineral::Isogen,
                        Mineral::Zydrine,
                    ],
                    vec![
                        120f64 * self.ore_modifier(),
                        60f64 * self.ore_modifier(),
                    ],
                )
            },

            Self::Jaspet                                |
            Self::PureJaspet                            |
            Self::PristineJaspet                        |
            Self::ImmaculateJaspet                      |
            Self::CompressedJaspet                      |
            Self::CompressedPureJaspet                  |
            Self::CompressedPristineJaspet              |
            Self::CompressedImmaculateJaspet            => {
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

            Self::Kernite                               |
            Self::LuminousKernite                       |
            Self::FieryKernite                          |
            Self::ResplendantKernite                    |
            Self::CompressedKernite                     |
            Self::CompressedLuminousKernite             |
            Self::CompressedFieryKernite                |
            Self::CompressedResplendantKernite          => {
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

            Self::Kylixium                              |
            Self::KaolinKylixium                        |
            Self::ArgilKylixium                         |
            Self::AdobeKylixium                         |
            Self::CompressedKylixium                    |
            Self::CompressedKaolinKylixium              |
            Self::CompressedArgilKylixium               |
            Self::CompressedAdobeKylixium               => {
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

            Self::Mercoxit                              |
            Self::MagmaMercoxit                         |
            Self::VitreousMercoxit                      |
            Self::CompressedMercoxit                    |
            Self::CompressedMagmaMercoxit               |
            Self::CompressedVitreousMercoxit            => {
                mineral_init(
                    vec![
                        Mineral::Morphite,
                    ],
                    vec![
                        140f64 * self.ore_modifier(),
                    ],
                )
            },

            Self::Mordunium                             |
            Self::PlumMordunium                         |
            Self::PrizeMordunium                        |
            Self::PlunderMordunium                      |
            Self::CompressedMordunium                   |
            Self::CompressedPlumMordunium               |
            Self::CompressedPrizeMordunium              |
            Self::CompressedPlunderMordunium            => {
                mineral_init(
                    vec![
                        Mineral::Pyerite,
                    ],
                    vec![
                        84f64 * self.ore_modifier(),
                    ],
                )
            },

            Self::Nocxite                               |
            Self::FragrantNocxite                       |
            Self::IntoxicatingNocxite                   |
            Self::AmbrosialNocxite                      |
            Self::CompressedNocxite                     |
            Self::CompressedFragrantNocxite             |
            Self::CompressedIntoxicatingNocxite         |
            Self::CompressedAmbrosialNocxite            => {
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

            Self::Omber                                 |
            Self::SilveryOmber                          |
            Self::GoldenOmber                           |
            Self::PlatinoidOmber                        |
            Self::CompressedOmber                       |
            Self::CompressedSilveryOmber                |
            Self::CompressedGoldenOmber                 |
            Self::CompressedPlatinoidOmber              => {
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

            Self::Plagioclase                           |
            Self::AzurePlagioclase                      |
            Self::RichPlagioclase                       |
            Self::SparklingPlagioclase                  |
            Self::CompressedPlagioclase                 |
            Self::CompressedAzurePlagioclase            |
            Self::CompressedRichPlagioclase             |
            Self::CompressedSparklingPlagioclase        => {
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

            Self::Pyroxeres                             |
            Self::SolidPyroxeres                        |
            Self::ViscousPyroxeres                      |
            Self::OpulentPyroxeres                      |
            Self::CompressedPyroxeres                   |
            Self::CompressedSolidPyroxeres              |
            Self::CompressedViscousPyroxeres            |
            Self::CompressedOpulentPyroxeres            => {
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

            Self::Rakovene                              |
            Self::AbyssalRakovene                       |
            Self::HadalRakovene                         |
            Self::CompressedRakovene                    |
            Self::CompressedAbyssalRakovene             |
            Self::CompressedHadalRakovene               => {
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

            Self::Scordite                              |
            Self::CondensedScordite                     |
            Self::MassiveScordite                       |
            Self::GlossyScordite                        |
            Self::CompressedScordite                    |
            Self::CompressedCondensedScordite           |
            Self::CompressedMassiveScordite             |
            Self::CompressedGlossyScordite              => {
                mineral_init(
                    vec![
                        Mineral::Tritanium,
                        Mineral::Pyerite,
                    ],
                    vec![
                        150f64 * self.ore_modifier(),
                        90f64 * self.ore_modifier(),
                    ],
                )
            },

            Self::Spodumain                             |
            Self::BrightSpodumain                       |
            Self::GleamingSpodumain                     |
            Self::DazzlingSpodumain                     |
            Self::CompressedSpodumain                   |
            Self::CompressedBrightSpodumain             |
            Self::CompressedGleamingSpodumain           |
            Self::CompressedDazzlingSpodumain           => {
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

            Self::Talassonite                           |
            Self::AbyssalTalassonite                    |
            Self::HadalTalassonite                      |
            Self::CompressedTalassonite                 |
            Self::CompressedAbyssalTalassonite          |
            Self::CompressedHadalTalassonite            => {
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

            Self::Ueganite                              |
            Self::FoggyUeganite                         |
            Self::OvercastUeganite                      |
            Self::StormyUeganite                        |
            Self::CompressedUeganite                    |
            Self::CompressedFoggyUeganite               |
            Self::CompressedOvercastUeganite            |
            Self::CompressedStormyUeganite              => {
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

            Self::Veldspar                              |
            Self::ConcentratedVeldspar                  |
            Self::DenseVeldspar                         |
            Self::StableVeldspar                        |
            Self::CompressedVeldspar                    |
            Self::CompressedConcentratedVeldspar        |
            Self::CompressedDenseVeldspar               |
            Self::CompressedStableVeldspar              => {
                mineral_init(
                    vec![
                        Mineral::Tritanium,
                    ],
                    vec![
                        400f64 * self.ore_modifier(),
                    ],
                )
            },

            Self::Ytirium                               |
            Self::BootlegYtirium                        |
            Self::FirewaterYtirium                      |
            Self::MoonshineYtirium                      |
            Self::CompressedYtirium                     |
            Self::CompressedBootlegYtirium              |
            Self::CompressedFirewaterYtirium            |
            Self::CompressedMoonshineYtirium            => {
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
            Self::Bitumens                              |
            Self::BrimfulBitumens                       |
            Self::GlisteningBitumens                    |
            Self::CompressedBitumens                    |
            Self::CompressedBrimfulBitumens             |
            Self::CompressedGlisteningBitumens          => {
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

            Self::Coesite                               |
            Self::BrimfulCoesite                        |
            Self::GlisteningCoesite                     |
            Self::CompressedCoesite                     |
            Self::CompressedBrimfulCoesite              |
            Self::CompressedGlisteningCoesite           => {
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

            Self::Sylvite                               |
            Self::BrimfulSylvite                        |
            Self::GlisteningSylvite                     |
            Self::CompressedSylvite                     |
            Self::CompressedBrimfulSylvite              |
            Self::CompressedGlisteningSylvite           => {
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

            Self::Zeolites                              |
            Self::BrimfulZeolites                       |
            Self::GlisteningZeolites                    |
            Self::CompressedZeolites                    |
            Self::CompressedBrimfulZeolites             |
            Self::CompressedGlisteningZeolites          => {
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

            Self::Cobaltite                             |
            Self::CopiousCobaltite                      |
            Self::TwinklingCobaltite                    |
            Self::CompressedCobaltite                   |
            Self::CompressedCopiousCobaltite            |
            Self::CompressedTwinklingCobaltite          => {
                mineral_init(
                    vec![
                        Mineral::Cobalt,
                    ],
                    vec![
                        40f64 * self.ore_modifier(),
                    ],
                )
            },

            Self::Euxenite                              |
            Self::CopiousEuxenite                       |
            Self::TwinklingEuxenite                     |
            Self::CompressedEuxenite                    |
            Self::CompressedCopiousEuxenite             |
            Self::CompressedTwinklingEuxenite           => {
                mineral_init(
                    vec![
                        Mineral::Scandium,
                    ],
                    vec![
                        40f64 * self.ore_modifier(),
                    ],
                )
            },

            Self::Scheelite                             |
            Self::CopiousScheelite                      |
            Self::TwinklingScheelite                    |
            Self::CompressedScheelite                   |
            Self::CompressedCopiousScheelite            |
            Self::CompressedTwinklingScheelite          => {
                mineral_init(
                    vec![
                        Mineral::Tungsten,
                    ],
                    vec![
                        40f64 * self.ore_modifier(),
                    ],
                )
            },

            Self::Titanite                              |
            Self::CopiousTitanite                       |
            Self::TwinklingTitanite                     |
            Self::CompressedTitanite                    |
            Self::CompressedCopiousTitanite             |
            Self::CompressedTwinklingTitanite           => {
                mineral_init(
                    vec![
                        Mineral::Titanium,
                    ],
                    vec![
                        40f64 * self.ore_modifier(),
                    ],
                )
            },

            Self::Chromite                              |
            Self::LavishChromite                        |
            Self::ShimmeringChromite                    |
            Self::CompressedChromite                    |
            Self::CompressedLavishChromite              |
            Self::CompressedShimmeringChromite          => {
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

            Self::Otavite                               |
            Self::LavishOtavite                         |
            Self::ShimmeringOtavite                     |
            Self::CompressedOtavite                     |
            Self::CompressedLavishOtavite               |
            Self::CompressedShimmeringOtavite           => {
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

            Self::Sperrylite                            |
            Self::LavishSperrylite                      |
            Self::ShimmeringSperrylite                  |
            Self::CompressedSperrylite                  |
            Self::CompressedLavishSperrylite            |
            Self::CompressedShimmeringSperrylite        => {
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

            Self::Vanadinite                            |
            Self::LavishVanadinite                      |
            Self::ShimmeringVanadinite                  |
            Self::CompressedVanadinite                  |
            Self::CompressedLavishVanadinite            |
            Self::CompressedShimmeringVanadinite        => {
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

            Self::Carnotite                             |
            Self::RepleteCarnotite                      |
            Self::GlowingCarnotite                      |
            Self::CompressedCarnotite                   |
            Self::CompressedRepleteCarnotite            |
            Self::CompressedGlowingCarnotite            => {
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

            Self::Cinnabar                              |
            Self::RepleteCinnabar                       |
            Self::GlowingCinnabar                       |
            Self::CompressedCinnabar                    |
            Self::CompressedRepleteCinnabar             |
            Self::CompressedGlowingCinnabar             => {
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

            Self::Pollucite                             |
            Self::RepletePollucite                      |
            Self::GlowingPollucite                      |
            Self::CompressedPollucite                   |
            Self::CompressedRepletePollucite            |
            Self::CompressedGlowingPollucite            => {
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

            Self::Zircon                                |
            Self::RepleteZircon                         |
            Self::GlowingZircon                         |
            Self::CompressedZircon                      |
            Self::CompressedRepleteZircon               |
            Self::CompressedGlowingZircon               => {
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

            Self::Loparite                              |
            Self::BountifulLoparite                     |
            Self::ShiningLoparite                       |
            Self::CompressedLoparite                    |
            Self::CompressedBountifulLoparite           |
            Self::CompressedShiningLoparite             => {
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

            Self::Monazite                              |
            Self::BountifulMonazite                     |
            Self::ShiningMonazite                       |
            Self::CompressedMonazite                    |
            Self::CompressedBountifulMonazite           |
            Self::CompressedShiningMonazite             => {
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

            Self::Xenotime                              |
            Self::BountifulXenotime                     |
            Self::ShiningXenotime                       |
            Self::CompressedXenotime                    |
            Self::CompressedBountifulXenotime           |
            Self::CompressedShiningXenotime             => {
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

            Self::Ytterbite                             |
            Self::BountifulYtterbite                    |
            Self::ShiningYtterbite                      |
            Self::CompressedYtterbite                   |
            Self::CompressedBountifulYtterbite          |
            Self::CompressedShiningYtterbite            => {
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
        }
    }

    pub fn ore_modifier(&self) -> f64 {
        match self {
            Self::Arkonor                               |
            Self::CompressedArkonor                     |
            Self::Bezdnacine                            |
            Self::CompressedBezdnacine                  |
            Self::Bistot                                |
            Self::CompressedBistot                      |
            Self::Crokite                               |
            Self::CompressedCrokite                     |
            Self::DarkOchre                             |
            Self::CompressedDarkOchre                   |
            Self::Ducinium                              |
            Self::CompressedDucinium                    |
            Self::Eifyrium                              |
            Self::CompressedEifyrium                    |
            Self::Gneiss                                |
            Self::CompressedGneiss                      |
            Self::Griemeer                              |
            Self::CompressedGriemeer                    |
            Self::Hedbergite                            |
            Self::CompressedHedbergite                  |
            Self::Hemorphite                            |
            Self::CompressedHemorphite                  |
            Self::Hezorime                              |
            Self::CompressedHezorime                    |
            Self::Jaspet                                |
            Self::CompressedJaspet                      |
            Self::Kernite                               |
            Self::CompressedKernite                     |
            Self::Kylixium                              |
            Self::CompressedKylixium                    |
            Self::Mercoxit                              |
            Self::CompressedMercoxit                    |
            Self::Mordunium                             |
            Self::CompressedMordunium                   |
            Self::Nocxite                               |
            Self::CompressedNocxite                     |
            Self::Omber                                 |
            Self::CompressedOmber                       |
            Self::Plagioclase                           |
            Self::CompressedPlagioclase                 |
            Self::Pyroxeres                             |
            Self::CompressedPyroxeres                   |
            Self::Rakovene                              |
            Self::CompressedRakovene                    |
            Self::Scordite                              |
            Self::CompressedScordite                    |
            Self::Spodumain                             |
            Self::CompressedSpodumain                   |
            Self::Talassonite                           |
            Self::CompressedTalassonite                 |
            Self::Ueganite                              |
            Self::CompressedUeganite                    |
            Self::Veldspar                              |
            Self::CompressedVeldspar                    |
            Self::Ytirium                               |
            Self::CompressedYtirium                     => 1.00,

            Self::CrimsonArkonor                        |
            Self::CompressedCrimsonArkonor              |
            Self::AbyssalBezdnacine                     |
            Self::CompressedAbyssalBezdnacine           |
            Self::TriclinicBistot                       |
            Self::CompressedTriclinicBistot             |
            Self::SharpCrokite                          |
            Self::CompressedSharpCrokite                |
            Self::OnyxOchre                             |
            Self::CompressedOnyxOchre                   |
            Self::NobleDucinium                         |
            Self::CompressedNobleDucinium               |
            Self::DopedEifyrium                         |
            Self::CompressedDopedEifyrium               |
            Self::IridescentGneiss                      |
            Self::CompressedIridescentGneiss            |
            Self::ClearGriemeer                         |
            Self::CompressedClearGriemeer               |
            Self::VitricHedbergite                      |
            Self::CompressedVitricHedbergite            |
            Self::VividHemorphite                       |
            Self::CompressedVividHemorphite             |
            Self::DullHezorime                          |
            Self::CompressedDullHezorime                |
            Self::PureJaspet                            |
            Self::CompressedPureJaspet                  |
            Self::LuminousKernite                       |
            Self::CompressedLuminousKernite             |
            Self::KaolinKylixium                        |
            Self::CompressedKaolinKylixium              |
            Self::MagmaMercoxit                         |
            Self::CompressedMagmaMercoxit               |
            Self::PlumMordunium                         |
            Self::CompressedPlumMordunium               |
            Self::FragrantNocxite                       |
            Self::CompressedFragrantNocxite             |
            Self::SilveryOmber                          |
            Self::CompressedSilveryOmber                |
            Self::AzurePlagioclase                      |
            Self::CompressedAzurePlagioclase            |
            Self::SolidPyroxeres                        |
            Self::CompressedSolidPyroxeres              |
            Self::AbyssalRakovene                       |
            Self::CompressedAbyssalRakovene             |
            Self::CondensedScordite                     |
            Self::CompressedCondensedScordite           |
            Self::BrightSpodumain                       |
            Self::CompressedBrightSpodumain             |
            Self::AbyssalTalassonite                    |
            Self::CompressedAbyssalTalassonite          |
            Self::FoggyUeganite                         |
            Self::CompressedFoggyUeganite               |
            Self::ConcentratedVeldspar                  |
            Self::CompressedConcentratedVeldspar        |
            Self::BootlegYtirium                        |
            Self::CompressedBootlegYtirium              => 1.05,

            Self::PrimeArkonor                          |
            Self::CompressedPrimeArkonor                |
            Self::HadalBezdnacine                       |
            Self::CompressedHadalBezdnacine             |
            Self::MonoclinicBistot                      |
            Self::CompressedMonoclinicBistot            |
            Self::CrystallineCrokite                    |
            Self::CompressedCrystallineCrokite          |
            Self::ObsidianOchre                         |
            Self::CompressedObsidianOchre               |
            Self::RoyalDucinium                         |
            Self::CompressedRoyalDucinium               |
            Self::BoostedEifyrium                       |
            Self::CompressedBoostedEifyrium             |
            Self::PrismaticGneiss                       |
            Self::CompressedPrismaticGneiss             |
            Self::InkyGriemeer                          |
            Self::CompressedInkyGriemeer                |
            Self::GlazedHedbergite                      |
            Self::CompressedGlazedHedbergite            |
            Self::RadiantHemorphite                     |
            Self::CompressedRadiantHemorphite           |
            Self::SerratedHezorime                      |
            Self::CompressedSerratedHezorime            |
            Self::PristineJaspet                        |
            Self::CompressedPristineJaspet              |
            Self::FieryKernite                          |
            Self::CompressedFieryKernite                |
            Self::ArgilKylixium                         |
            Self::CompressedArgilKylixium               |
            Self::VitreousMercoxit                      |
            Self::CompressedVitreousMercoxit            |
            Self::PrizeMordunium                        |
            Self::CompressedPrizeMordunium              |
            Self::IntoxicatingNocxite                   |
            Self::CompressedIntoxicatingNocxite         |
            Self::GoldenOmber                           |
            Self::CompressedGoldenOmber                 |
            Self::RichPlagioclase                       |
            Self::CompressedRichPlagioclase             |
            Self::ViscousPyroxeres                      |
            Self::CompressedViscousPyroxeres            |
            Self::HadalRakovene                         |
            Self::CompressedHadalRakovene               |
            Self::MassiveScordite                       |
            Self::CompressedMassiveScordite             |
            Self::GleamingSpodumain                     |
            Self::CompressedGleamingSpodumain           |
            Self::HadalTalassonite                      |
            Self::CompressedHadalTalassonite            |
            Self::OvercastUeganite                      |
            Self::CompressedOvercastUeganite            |
            Self::DenseVeldspar                         |
            Self::CompressedDenseVeldspar               |
            Self::FirewaterYtirium                      |
            Self::CompressedFirewaterYtirium            => 1.10,

            Self::FlawlessArkonor                       |
            Self::CompressedFlawlessArkonor             |
            Self::CubicBistot                           |
            Self::CompressedCubicBistot                 |
            Self::PellucidCrokite                       |
            Self::CompressedPellucidCrokite             |
            Self::JetOchre                              |
            Self::CompressedJetOchre                    |
            Self::ImperialDucinium                      |
            Self::CompressedImperialDucinium            |
            Self::AugmentedEifyrium                     |
            Self::CompressedAugmentedEifyrium           |
            Self::BrilliantGneiss                       |
            Self::CompressedBrilliantGneiss             |
            Self::OpaqueGriemeer                        |
            Self::CompressedOpaqueGriemeer              |
            Self::LustrousHedbergite                    |
            Self::CompressedLustrousHedbergite          |
            Self::ScintillatingHemorphite               |
            Self::CompressedScintillatingHemorphite     |
            Self::SharpHezorime                         |
            Self::CompressedSharpHezorime               |
            Self::ImmaculateJaspet                      |
            Self::CompressedImmaculateJaspet            |
            Self::ResplendantKernite                    |
            Self::CompressedResplendantKernite          |
            Self::AdobeKylixium                         |
            Self::CompressedAdobeKylixium               |
            Self::PlunderMordunium                      |
            Self::CompressedPlunderMordunium            |
            Self::AmbrosialNocxite                      |
            Self::CompressedAmbrosialNocxite            |
            Self::PlatinoidOmber                        |
            Self::CompressedPlatinoidOmber              |
            Self::SparklingPlagioclase                  |
            Self::CompressedSparklingPlagioclase        |
            Self::OpulentPyroxeres                      |
            Self::CompressedOpulentPyroxeres            |
            Self::GlossyScordite                        |
            Self::CompressedGlossyScordite              |
            Self::DazzlingSpodumain                     |
            Self::CompressedDazzlingSpodumain           |
            Self::StormyUeganite                        |
            Self::CompressedStormyUeganite              |
            Self::StableVeldspar                        |
            Self::CompressedStableVeldspar              |
            Self::MoonshineYtirium                      |
            Self::CompressedMoonshineYtirium            => 1.15,

            // moon
            Self::Bitumens                              |
            Self::CompressedBitumens                    |
            Self::Coesite                               |
            Self::CompressedCoesite                     |
            Self::Sylvite                               |
            Self::CompressedSylvite                     |
            Self::Zeolites                              |
            Self::CompressedZeolites                    |
            Self::Cobaltite                             |
            Self::CompressedCobaltite                   |
            Self::Euxenite                              |
            Self::CompressedEuxenite                    |
            Self::Scheelite                             |
            Self::CompressedScheelite                   |
            Self::Titanite                              |
            Self::CompressedTitanite                    |
            Self::Chromite                              |
            Self::CompressedChromite                    |
            Self::Otavite                               |
            Self::CompressedOtavite                     |
            Self::Sperrylite                            |
            Self::CompressedSperrylite                  |
            Self::Vanadinite                            |
            Self::CompressedVanadinite                  |
            Self::Carnotite                             |
            Self::CompressedCarnotite                   |
            Self::Cinnabar                              |
            Self::CompressedCinnabar                    |
            Self::Pollucite                             |
            Self::CompressedPollucite                   |
            Self::Zircon                                |
            Self::CompressedZircon                      |
            Self::Loparite                              |
            Self::CompressedLoparite                    |
            Self::Monazite                              |
            Self::CompressedMonazite                    |
            Self::Xenotime                              |
            Self::CompressedXenotime                    |
            Self::Ytterbite                             |
            Self::CompressedYtterbite                   => 1.00,

            Self::BrimfulBitumens                       |
            Self::CompressedBrimfulBitumens             |
            Self::BrimfulCoesite                        |
            Self::CompressedBrimfulCoesite              |
            Self::BrimfulSylvite                        |
            Self::CompressedBrimfulSylvite              |
            Self::BrimfulZeolites                       |
            Self::CompressedBrimfulZeolites             |
            Self::CopiousCobaltite                      |
            Self::CompressedCopiousCobaltite            |
            Self::CopiousEuxenite                       |
            Self::CompressedCopiousEuxenite             |
            Self::CopiousScheelite                      |
            Self::CompressedCopiousScheelite            |
            Self::CopiousTitanite                       |
            Self::CompressedCopiousTitanite             |
            Self::LavishChromite                        |
            Self::CompressedLavishChromite              |
            Self::LavishOtavite                         |
            Self::CompressedLavishOtavite               |
            Self::LavishSperrylite                      |
            Self::CompressedLavishSperrylite            |
            Self::LavishVanadinite                      |
            Self::CompressedLavishVanadinite            |
            Self::RepleteCarnotite                      |
            Self::CompressedRepleteCarnotite            |
            Self::RepleteCinnabar                       |
            Self::CompressedRepleteCinnabar             |
            Self::RepletePollucite                      |
            Self::CompressedRepletePollucite            |
            Self::RepleteZircon                         |
            Self::CompressedRepleteZircon               |
            Self::BountifulLoparite                     |
            Self::CompressedBountifulLoparite           |
            Self::BountifulMonazite                     |
            Self::CompressedBountifulMonazite           |
            Self::BountifulXenotime                     |
            Self::CompressedBountifulXenotime           |
            Self::BountifulYtterbite                    |
            Self::CompressedBountifulYtterbite          => 1.15,

            Self::GlisteningBitumens                    |
            Self::CompressedGlisteningBitumens          |
            Self::GlisteningCoesite                     |
            Self::CompressedGlisteningCoesite           |
            Self::GlisteningSylvite                     |
            Self::CompressedGlisteningSylvite           |
            Self::GlisteningZeolites                    |
            Self::CompressedGlisteningZeolites          |
            Self::TwinklingCobaltite                    |
            Self::CompressedTwinklingCobaltite          |
            Self::TwinklingEuxenite                     |
            Self::CompressedTwinklingEuxenite           |
            Self::TwinklingScheelite                    |
            Self::CompressedTwinklingScheelite          |
            Self::TwinklingTitanite                     |
            Self::CompressedTwinklingTitanite           |
            Self::ShimmeringChromite                    |
            Self::CompressedShimmeringChromite          |
            Self::ShimmeringOtavite                     |
            Self::CompressedShimmeringOtavite           |
            Self::ShimmeringSperrylite                  |
            Self::CompressedShimmeringSperrylite        |
            Self::ShimmeringVanadinite                  |
            Self::CompressedShimmeringVanadinite        |
            Self::GlowingCarnotite                      |
            Self::CompressedGlowingCarnotite            |
            Self::GlowingCinnabar                       |
            Self::CompressedGlowingCinnabar             |
            Self::GlowingPollucite                      |
            Self::CompressedGlowingPollucite            |
            Self::GlowingZircon                         |
            Self::CompressedGlowingZircon               |
            Self::ShiningLoparite                       |
            Self::CompressedShiningLoparite             |
            Self::ShiningMonazite                       |
            Self::CompressedShiningMonazite             |
            Self::ShiningXenotime                       |
            Self::CompressedShiningXenotime             |
            Self::ShiningYtterbite                      |
            Self::CompressedShiningYtterbite            => 2.00,

            _                                           => 1.00,
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

            _ => unimplemented!()
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
            Self::Arkonor |
            Self::CrimsonArkonor |
            Self::PrimeArkonor |
            Self::FlawlessArkonor |

            Self::Bezdnacine |
            Self::AbyssalBezdnacine |
            Self::HadalBezdnacine |

            Self::Bistot |
            Self::TriclinicBistot |
            Self::MonoclinicBistot |
            Self::CubicBistot |

            Self::Crokite |
            Self::SharpCrokite |
            Self::CrystallineCrokite |
            Self::PellucidCrokite |

            Self::DarkOchre |
            Self::OnyxOchre |
            Self::ObsidianOchre |
            Self::JetOchre |

            Self::Ducinium |
            Self::NobleDucinium |
            Self::RoyalDucinium |
            Self::ImperialDucinium |

            Self::Eifyrium |
            Self::DopedEifyrium |
            Self::BoostedEifyrium |
            Self::AugmentedEifyrium |

            Self::Gneiss |
            Self::IridescentGneiss |
            Self::PrismaticGneiss |
            Self::BrilliantGneiss |

            Self::Griemeer |
            Self::ClearGriemeer |
            Self::InkyGriemeer |
            Self::OpaqueGriemeer |

            Self::Hedbergite |
            Self::VitricHedbergite |
            Self::GlazedHedbergite |
            Self::LustrousHedbergite |

            Self::Hemorphite |
            Self::VividHemorphite |
            Self::RadiantHemorphite |
            Self::ScintillatingHemorphite |

            Self::Hezorime |
            Self::DullHezorime |
            Self::SerratedHezorime |
            Self::SharpHezorime |

            Self::Jaspet |
            Self::PureJaspet |
            Self::PristineJaspet |
            Self::ImmaculateJaspet |

            Self::Kernite |
            Self::LuminousKernite |
            Self::FieryKernite |
            Self::ResplendantKernite |

            Self::Kylixium |
            Self::KaolinKylixium |
            Self::ArgilKylixium |
            Self::AdobeKylixium |

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
            Self::AbyssalRakovene |
            Self::HadalRakovene |

            Self::Scordite |
            Self::CondensedScordite |
            Self::MassiveScordite |
            Self::GlossyScordite |

            Self::Spodumain |
            Self::BrightSpodumain |
            Self::GleamingSpodumain |
            Self::DazzlingSpodumain |

            Self::Talassonite |
            Self::AbyssalTalassonite |
            Self::HadalTalassonite |

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

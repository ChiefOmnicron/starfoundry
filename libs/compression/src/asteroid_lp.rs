use good_lp::{constraint, default_solver, variables, Solution, SolverModel};
use std::collections::HashMap;

use crate::asteroid::Asteroid;
use crate::config::{Config, Mineral};
use crate::{Error, Result};

pub fn calculate_ore(
    config: &Config,
) -> Result<HashMap<i32, f64>> {
    variables! {
        vars:
            arkonor;
            crimson_arkonor;
            prime_arkonor;
            flawless_arkonor;
            compressed_arkonor;
            compressed_crimson_arkonor;
            compressed_prime_arkonor;
            compressed_flawless_arkonor;

            bezdnacine;
            abyssal_bezdnacine;
            hadal_bezdnacine;
            compressed_bezdnacine;
            compressed_abyssal_bezdnacine;
            compressed_hadal_bezdnacine;

            bistot;
            triclinic_bistot;
            monoclinic_bistot;
            cubic_bistot;
            compressed_bistot;
            compressed_triclinic_bistot;
            compressed_monoclinic_bistot;
            compressed_cubic_bistot;

            crokite;
            sharp_crokite;
            crystalline_crokite;
            pellucid_crokite;
            compressed_crokite;
            compressed_sharp_crokite;
            compressed_crystalline_crokite;
            compressed_pellucid_crokite;

            dark_ochre;
            onyx_ochre;
            obsidian_ochre;
            jet_ochre;
            compressed_dark_ochre;
            compressed_onyx_ochre;
            compressed_obsidian_ochre;
            compressed_jet_ochre;

            ducinium;
            noble_ducinium;
            royal_ducinium;
            imperial_ducinium;
            compressed_ducinium;
            compressed_noble_ducinium;
            compressed_royal_ducinium;
            compressed_imperial_ducinium;

            eifyrium;
            doped_eifyrium;
            boosted_eifyrium;
            augmented_eifyrium;
            compressed_eifyrium;
            compressed_doped_eifyrium;
            compressed_boosted_eifyrium;
            compressed_augmented_eifyrium;

            gneiss;
            iridescent_gneiss;
            prismatic_gneiss;
            brilliant_gneiss;
            compressed_gneiss;
            compressed_iridescent_gneiss;
            compressed_prismatic_gneiss;
            compressed_brilliant_gneiss;

            griemeer;
            clear_griemeer;
            inky_griemeer;
            opaque_griemeer;
            compressed_griemeer;
            compressed_clear_griemeer;
            compressed_inky_griemeer;
            compressed_opaque_griemeer;

            hedbergite;
            vitric_hedbergite;
            glazed_hedbergite;
            lustrous_hedbergite;
            compressed_hedbergite;
            compressed_vitric_hedbergite;
            compressed_glazed_hedbergite;
            compressed_lustrous_hedbergite;

            hemorphite;
            vivid_hemorphite;
            radiant_hemorphite;
            scintillating_hemorphite;
            compressed_hemorphite;
            compressed_vivid_hemorphite;
            compressed_radiant_hemorphite;
            compressed_scintillating_hemorphite;

            hezorime;
            dull_hezorime;
            serrated_hezorime;
            sharp_hezorime;
            compressed_hezorime;
            compressed_dull_hezorime;
            compressed_serrated_hezorime;
            compressed_sharp_hezorime;

            jaspet;
            pure_jaspet;
            pristine_jaspet;
            immaculate_jaspet;
            compressed_jaspet;
            compressed_pure_jaspet;
            compressed_pristine_jaspet;
            compressed_immaculate_jaspet;

            kernite;
            luminous_kernite;
            fiery_kernite;
            resplendant_kernite;
            compressed_kernite;
            compressed_luminous_kernite;
            compressed_fiery_kernite;
            compressed_resplendant_kernite;

            kylixium;
            kaolin_kylixium;
            argil_kylixium;
            adobe_kylixium;
            compressed_kylixium;
            compressed_kaolin_kylixium;
            compressed_argil_kylixium;
            compressed_adobe_kylixium;

            mercoxit;
            magma_mercoxit;
            vitreous_mercoxit;
            compressed_mercoxit;
            compressed_magma_mercoxit;
            compressed_vitreous_mercoxit;

            mordunium;
            plum_mordunium;
            prize_mordunium;
            plunder_mordunium;
            compressed_mordunium;
            compressed_plum_mordunium;
            compressed_prize_mordunium;
            compressed_plunder_mordunium;

            nocxite;
            fragrant_nocxite;
            intoxicating_nocxite;
            ambrosial_nocxite;
            compressed_nocxite;
            compressed_fragrant_nocxite;
            compressed_intoxicating_nocxite;
            compressed_ambrosial_nocxite;

            omber;
            silvery_omber;
            golden_omber;
            platinoid_omber;
            compressed_omber;
            compressed_silvery_omber;
            compressed_golden_omber;
            compressed_platinoid_omber;

            plagioclase;
            azure_plagioclase;
            rich_plagioclase;
            sparkling_plagioclase;
            compressed_plagioclase;
            compressed_azure_plagioclase;
            compressed_rich_plagioclase;
            compressed_sparkling_plagioclase;

            pyroxeres;
            solid_pyroxeres;
            viscous_pyroxeres;
            opulent_pyroxeres;
            compressed_pyroxeres;
            compressed_solid_pyroxeres;
            compressed_viscous_pyroxeres;
            compressed_opulent_pyroxeres;

            rakovene;
            abyssal_rakovene;
            hadal_rakovene;
            compressed_rakovene;
            compressed_abyssal_rakovene;
            compressed_hadal_rakovene;

            scordite;
            condensed_scordite;
            massive_scordite;
            glossy_scordite;
            compressed_scordite;
            compressed_condensed_scordite;
            compressed_massive_scordite;
            compressed_glossy_scordite;

            spodumain;
            bright_spodumain;
            gleaming_spodumain;
            dazzling_spodumain;
            compressed_spodumain;
            compressed_bright_spodumain;
            compressed_gleaming_spodumain;
            compressed_dazzling_spodumain;

            talassonite;
            abyssal_talassonite;
            hadal_talassonite;
            compressed_talassonite;
            compressed_abyssal_talassonite;
            compressed_hadal_talassonite;

            ueganite;
            foggy_ueganite;
            overcast_ueganite;
            stormy_ueganite;
            compressed_ueganite;
            compressed_foggy_ueganite;
            compressed_overcast_ueganite;
            compressed_stormy_ueganite;

            veldspar;
            concentrated_veldspar;
            dense_veldspar;
            stable_veldspar;
            compressed_veldspar;
            compressed_concentrated_veldspar;
            compressed_dense_veldspar;
            compressed_stable_veldspar;

            ytirium;
            bootleg_ytirium;
            firewater_ytirium;
            moonshine_ytirium;
            compressed_ytirium;
            compressed_bootleg_ytirium;
            compressed_firewater_ytirium;
            compressed_moonshine_ytirium;

            // R4
            bitumens;
            brimful_bitumens;
            glistening_bitumens;
            compressed_bitumens;
            compressed_brimful_bitumens;
            compressed_glistering_bitumens;

            coesite;
            brimful_coesite;
            glistening_coesite;
            compressed_coesite;
            compressed_brimful_coesite;
            compressed_glistering_coesite;

            sylvite;
            brimful_sylvite;
            glistening_sylvite;
            compressed_sylvite;
            compressed_brimful_sylvite;
            compressed_glistering_sylvite;

            zeolites;
            brimful_zeolites;
            glistening_zeolites;
            compressed_zeolites;
            compressed_brimful_zeolites;
            compressed_glistering_zeolites;

            // R8
            cobaltite;
            copious_cobaltite;
            twinkling_cobaltite;
            compressed_cobaltite;
            compressed_copious_cobaltite;
            compressed_twinkling_cobaltite;

            euxenite;
            copious_euxenite;
            twinkling_euxenite;
            compressed_euxenite;
            compressed_copious_euxenite;
            compressed_twinkling_euxenite;

            scheelite;
            copious_scheelite;
            twinkling_scheelite;
            compressed_scheelite;
            compressed_copious_scheelite;
            compressed_twinkling_scheelite;

            titanite;
            copious_titanite;
            twinkling_titanite;
            compressed_titanite;
            compressed_copious_titanite;
            compressed_twinkling_titanite;

            // R16
            chromite;
            lavish_chromite;
            shimmering_chromite;
            compressed_chromite;
            compressed_lavish_chromite;
            compressed_shimmering_chromite;

            otavite;
            lavish_otavite;
            shimmering_otavite;
            compressed_otavite;
            compressed_lavish_otavite;
            compressed_shimmering_otavite;

            sperrylite;
            lavish_sperrylite;
            shimmering_sperrylite;
            compressed_sperrylite;
            compressed_lavish_sperrylite;
            compressed_shimmering_sperrylite;

            vanadinite;
            lavish_vanadinite;
            shimmering_vanadinite;
            compressed_vanadinite;
            compressed_lavish_vanadinite;
            compressed_shimmering_vanadinite;

            // R32
            carnotite;
            replete_carnotite;
            glowing_carnotite;
            compressed_carnotite;
            compressed_replete_carnotite;
            compressed_glowing_carnotite;

            cinnabar;
            replete_cinnabar;
            glowing_cinnabar;
            compressed_cinnabar;
            compressed_replete_cinnabar;
            compressed_glowing_cinnabar;

            pollucite;
            replete_pollucite;
            glowing_pollucite;
            compressed_pollucite;
            compressed_replete_pollucite;
            compressed_glowing_pollucite;

            zircon;
            replete_zircon;
            glowing_zircon;
            compressed_zircon;
            compressed_replete_zircon;
            compressed_glowing_zircon;

            // R64
            loparite;
            bountiful_loparite;
            shining_loparite;
            compressed_loparite;
            compressed_bountiful_loparite;
            compressed_shining_loparite;

            monazite;
            bountiful_monazite;
            shining_monazite;
            compressed_monazite;
            compressed_bountiful_monazite;
            compressed_shining_monazite;

            xenotime;
            bountiful_xenotime;
            shining_xenotime;
            compressed_xenotime;
            compressed_bountiful_xenotime;
            compressed_shining_xenotime;

            ytterbite;
            bountiful_ytterbite;
            shining_ytterbite;
            compressed_ytterbite;
            compressed_bountiful_ytterbite;
            compressed_shining_ytterbite;

            tritanium;
            pyerite;
            mexallon;
            isogen;
            nocxium;
            zydrine;
            megacyte;
            morphite;

            // R4
            atmospheric_gases;
            evaporite_deposits;
            hydrocarbons;
            silicates;

            // R8
            cobalt;
            scandium;
            titanium;
            tungsten;

            // R16
            chromium;
            cadmium;
            platinum;
            vanadium;

            // R32
            caesium;
            hafnium;
            mercury;
            technetium;

            // R64
            promethium;
            neodymium;
            dysprosium;
            thulium;
    }

    // create the problem, find the lowest possible price while still fulfulling
    // the wanted minerals
    let mut problem = vars.minimise(
            (arkonor                             * config.asteroid_price(&Asteroid::Arkonor)) +
            (crimson_arkonor                     * config.asteroid_price(&Asteroid::CrimsonArkonor)) +
            (prime_arkonor                       * config.asteroid_price(&Asteroid::PrimeArkonor)) +
            (flawless_arkonor                    * config.asteroid_price(&Asteroid::FlawlessArkonor)) +
            (compressed_arkonor                  * config.asteroid_price(&Asteroid::CompressedArkonor)) +
            (compressed_crimson_arkonor          * config.asteroid_price(&Asteroid::CompressedCrimsonArkonor)) +
            (compressed_prime_arkonor            * config.asteroid_price(&Asteroid::CompressedPrimeArkonor)) +
            (compressed_flawless_arkonor         * config.asteroid_price(&Asteroid::CompressedFlawlessArkonor)) +

            (bezdnacine                          * config.asteroid_price(&Asteroid::Bezdnacine)) +
            (abyssal_bezdnacine                  * config.asteroid_price(&Asteroid::AbyssalBezdnacine)) +
            (hadal_bezdnacine                    * config.asteroid_price(&Asteroid::HadalBezdnacine)) +
            (compressed_bezdnacine               * config.asteroid_price(&Asteroid::CompressedBezdnacine)) +
            (compressed_abyssal_bezdnacine       * config.asteroid_price(&Asteroid::CompressedAbyssalBezdnacine)) +
            (compressed_hadal_bezdnacine         * config.asteroid_price(&Asteroid::CompressedHadalBezdnacine)) +

            (bistot                              * config.asteroid_price(&Asteroid::Bistot)) +
            (triclinic_bistot                    * config.asteroid_price(&Asteroid::TriclinicBistot)) +
            (monoclinic_bistot                   * config.asteroid_price(&Asteroid::MonoclinicBistot)) +
            (cubic_bistot                        * config.asteroid_price(&Asteroid::CubicBistot)) +
            (compressed_bistot                   * config.asteroid_price(&Asteroid::CompressedBistot)) +
            (compressed_triclinic_bistot         * config.asteroid_price(&Asteroid::CompressedTriclinicBistot)) +
            (compressed_monoclinic_bistot        * config.asteroid_price(&Asteroid::CompressedMonoclinicBistot)) +
            (compressed_cubic_bistot             * config.asteroid_price(&Asteroid::CompressedCubicBistot)) +

            (crokite                             * config.asteroid_price(&Asteroid::Crokite)) +
            (sharp_crokite                       * config.asteroid_price(&Asteroid::SharpCrokite)) +
            (crystalline_crokite                 * config.asteroid_price(&Asteroid::CrystallineCrokite)) +
            (pellucid_crokite                    * config.asteroid_price(&Asteroid::PellucidCrokite)) +
            (compressed_crokite                  * config.asteroid_price(&Asteroid::CompressedCrokite)) +
            (compressed_sharp_crokite            * config.asteroid_price(&Asteroid::CompressedSharpCrokite)) +
            (compressed_crystalline_crokite      * config.asteroid_price(&Asteroid::CompressedCrystallineCrokite)) +
            (compressed_pellucid_crokite         * config.asteroid_price(&Asteroid::CompressedPellucidCrokite)) +

            (dark_ochre                          * config.asteroid_price(&Asteroid::DarkOchre)) +
            (onyx_ochre                          * config.asteroid_price(&Asteroid::OnyxOchre)) +
            (obsidian_ochre                      * config.asteroid_price(&Asteroid::ObsidianOchre)) +
            (jet_ochre                           * config.asteroid_price(&Asteroid::JetOchre)) +
            (compressed_dark_ochre               * config.asteroid_price(&Asteroid::CompressedDarkOchre)) +
            (compressed_onyx_ochre               * config.asteroid_price(&Asteroid::CompressedOnyxOchre)) +
            (compressed_obsidian_ochre           * config.asteroid_price(&Asteroid::CompressedObsidianOchre)) +
            (compressed_jet_ochre                * config.asteroid_price(&Asteroid::CompressedJetOchre)) +

            (ducinium                            * config.asteroid_price(&Asteroid::Ducinium)) +
            (noble_ducinium                      * config.asteroid_price(&Asteroid::NobleDucinium)) +
            (royal_ducinium                      * config.asteroid_price(&Asteroid::RoyalDucinium)) +
            (imperial_ducinium                   * config.asteroid_price(&Asteroid::ImperialDucinium)) +
            (compressed_ducinium                 * config.asteroid_price(&Asteroid::CompressedDucinium)) +
            (compressed_noble_ducinium           * config.asteroid_price(&Asteroid::CompressedNobleDucinium)) +
            (compressed_royal_ducinium           * config.asteroid_price(&Asteroid::CompressedRoyalDucinium)) +
            (compressed_imperial_ducinium        * config.asteroid_price(&Asteroid::CompressedImperialDucinium)) +

            (eifyrium                            * config.asteroid_price(&Asteroid::Eifyrium)) +
            (doped_eifyrium                      * config.asteroid_price(&Asteroid::DopedEifyrium)) +
            (boosted_eifyrium                    * config.asteroid_price(&Asteroid::BoostedEifyrium)) +
            (augmented_eifyrium                  * config.asteroid_price(&Asteroid::AugmentedEifyrium)) +
            (compressed_eifyrium                 * config.asteroid_price(&Asteroid::CompressedEifyrium)) +
            (compressed_doped_eifyrium           * config.asteroid_price(&Asteroid::CompressedDopedEifyrium)) +
            (compressed_boosted_eifyrium         * config.asteroid_price(&Asteroid::CompressedBoostedEifyrium)) +
            (compressed_augmented_eifyrium       * config.asteroid_price(&Asteroid::CompressedAugmentedEifyrium)) +

            (gneiss                              * config.asteroid_price(&Asteroid::Gneiss)) +
            (iridescent_gneiss                   * config.asteroid_price(&Asteroid::IridescentGneiss)) +
            (prismatic_gneiss                    * config.asteroid_price(&Asteroid::PrismaticGneiss)) +
            (brilliant_gneiss                    * config.asteroid_price(&Asteroid::BrilliantGneiss)) +
            (compressed_gneiss                   * config.asteroid_price(&Asteroid::CompressedGneiss)) +
            (compressed_iridescent_gneiss        * config.asteroid_price(&Asteroid::CompressedIridescentGneiss)) +
            (compressed_prismatic_gneiss         * config.asteroid_price(&Asteroid::CompressedPrismaticGneiss)) +
            (compressed_brilliant_gneiss         * config.asteroid_price(&Asteroid::CompressedBrilliantGneiss)) +

            (griemeer                            * config.asteroid_price(&Asteroid::Griemeer)) +
            (clear_griemeer                      * config.asteroid_price(&Asteroid::ClearGriemeer)) +
            (inky_griemeer                       * config.asteroid_price(&Asteroid::InkyGriemeer)) +
            (opaque_griemeer                     * config.asteroid_price(&Asteroid::OpaqueGriemeer)) +
            (compressed_griemeer                 * config.asteroid_price(&Asteroid::CompressedGriemeer)) +
            (compressed_clear_griemeer           * config.asteroid_price(&Asteroid::CompressedClearGriemeer)) +
            (compressed_inky_griemeer            * config.asteroid_price(&Asteroid::CompressedInkyGriemeer)) +
            (compressed_opaque_griemeer          * config.asteroid_price(&Asteroid::CompressedOpaqueGriemeer)) +

            (hedbergite                          * config.asteroid_price(&Asteroid::Hedbergite)) +
            (vitric_hedbergite                   * config.asteroid_price(&Asteroid::VitricHedbergite)) +
            (glazed_hedbergite                   * config.asteroid_price(&Asteroid::GlazedHedbergite)) +
            (lustrous_hedbergite                 * config.asteroid_price(&Asteroid::LustrousHedbergite)) +
            (compressed_hedbergite               * config.asteroid_price(&Asteroid::CompressedHedbergite)) +
            (compressed_vitric_hedbergite        * config.asteroid_price(&Asteroid::CompressedVitricHedbergite)) +
            (compressed_glazed_hedbergite        * config.asteroid_price(&Asteroid::CompressedGlazedHedbergite)) +
            (compressed_lustrous_hedbergite      * config.asteroid_price(&Asteroid::CompressedLustrousHedbergite)) +

            (hemorphite                          * config.asteroid_price(&Asteroid::Hemorphite)) +
            (vivid_hemorphite                    * config.asteroid_price(&Asteroid::VividHemorphite)) +
            (radiant_hemorphite                  * config.asteroid_price(&Asteroid::RadiantHemorphite)) +
            (scintillating_hemorphite            * config.asteroid_price(&Asteroid::ScintillatingHemorphite)) +
            (compressed_hemorphite               * config.asteroid_price(&Asteroid::CompressedHemorphite)) +
            (compressed_vivid_hemorphite         * config.asteroid_price(&Asteroid::CompressedVividHemorphite)) +
            (compressed_radiant_hemorphite       * config.asteroid_price(&Asteroid::CompressedRadiantHemorphite)) +
            (compressed_scintillating_hemorphite * config.asteroid_price(&Asteroid::CompressedScintillatingHemorphite)) +

            (hezorime                            * config.asteroid_price(&Asteroid::Hezorime)) +
            (dull_hezorime                       * config.asteroid_price(&Asteroid::DullHezorime)) +
            (serrated_hezorime                   * config.asteroid_price(&Asteroid::SerratedHezorime)) +
            (sharp_hezorime                      * config.asteroid_price(&Asteroid::SharpHezorime)) +
            (compressed_hezorime                 * config.asteroid_price(&Asteroid::CompressedHezorime)) +
            (compressed_dull_hezorime            * config.asteroid_price(&Asteroid::CompressedDullHezorime)) +
            (compressed_serrated_hezorime        * config.asteroid_price(&Asteroid::CompressedSerratedHezorime)) +
            (compressed_sharp_hezorime           * config.asteroid_price(&Asteroid::CompressedSharpHezorime)) +

            (jaspet                              * config.asteroid_price(&Asteroid::Jaspet)) +
            (pure_jaspet                         * config.asteroid_price(&Asteroid::PureJaspet)) +
            (pristine_jaspet                     * config.asteroid_price(&Asteroid::PristineJaspet)) +
            (immaculate_jaspet                   * config.asteroid_price(&Asteroid::ImmaculateJaspet)) +
            (compressed_jaspet                   * config.asteroid_price(&Asteroid::CompressedJaspet)) +
            (compressed_pure_jaspet              * config.asteroid_price(&Asteroid::CompressedPureJaspet)) +
            (compressed_pristine_jaspet          * config.asteroid_price(&Asteroid::CompressedPristineJaspet)) +
            (compressed_immaculate_jaspet        * config.asteroid_price(&Asteroid::CompressedImmaculateJaspet)) +

            (kernite                             * config.asteroid_price(&Asteroid::Kernite)) +
            (luminous_kernite                    * config.asteroid_price(&Asteroid::LuminousKernite)) +
            (fiery_kernite                       * config.asteroid_price(&Asteroid::FieryKernite)) +
            (resplendant_kernite                 * config.asteroid_price(&Asteroid::ResplendantKernite)) +
            (compressed_kernite                  * config.asteroid_price(&Asteroid::CompressedKernite)) +
            (compressed_luminous_kernite         * config.asteroid_price(&Asteroid::CompressedLuminousKernite)) +
            (compressed_fiery_kernite            * config.asteroid_price(&Asteroid::CompressedFieryKernite)) +
            (compressed_resplendant_kernite      * config.asteroid_price(&Asteroid::CompressedResplendantKernite)) +

            (kylixium                            * config.asteroid_price(&Asteroid::Kylixium)) +
            (kaolin_kylixium                     * config.asteroid_price(&Asteroid::KaolinKylixium)) +
            (argil_kylixium                      * config.asteroid_price(&Asteroid::ArgilKylixium)) +
            (adobe_kylixium                      * config.asteroid_price(&Asteroid::AdobeKylixium)) +
            (compressed_kylixium                 * config.asteroid_price(&Asteroid::CompressedKylixium)) +
            (compressed_kaolin_kylixium          * config.asteroid_price(&Asteroid::CompressedKaolinKylixium)) +
            (compressed_argil_kylixium           * config.asteroid_price(&Asteroid::CompressedArgilKylixium)) +
            (compressed_adobe_kylixium           * config.asteroid_price(&Asteroid::CompressedAdobeKylixium)) +

            (mercoxit                            * config.asteroid_price(&Asteroid::Mercoxit)) +
            (magma_mercoxit                      * config.asteroid_price(&Asteroid::MagmaMercoxit)) +
            (vitreous_mercoxit                   * config.asteroid_price(&Asteroid::VitreousMercoxit)) +
            (compressed_mercoxit                 * config.asteroid_price(&Asteroid::CompressedMercoxit)) +
            (compressed_magma_mercoxit           * config.asteroid_price(&Asteroid::CompressedMagmaMercoxit)) +
            (compressed_vitreous_mercoxit        * config.asteroid_price(&Asteroid::CompressedVitreousMercoxit)) +

            (mordunium                           * config.asteroid_price(&Asteroid::Mordunium)) +
            (plum_mordunium                      * config.asteroid_price(&Asteroid::PlumMordunium)) +
            (prize_mordunium                     * config.asteroid_price(&Asteroid::PrizeMordunium)) +
            (plunder_mordunium                   * config.asteroid_price(&Asteroid::PlunderMordunium)) +
            (compressed_mordunium                * config.asteroid_price(&Asteroid::CompressedMordunium)) +
            (compressed_plum_mordunium           * config.asteroid_price(&Asteroid::CompressedPlumMordunium)) +
            (compressed_prize_mordunium          * config.asteroid_price(&Asteroid::CompressedPrizeMordunium)) +
            (compressed_plunder_mordunium        * config.asteroid_price(&Asteroid::CompressedPlunderMordunium)) +

            (nocxite                             * config.asteroid_price(&Asteroid::Nocxite)) +
            (fragrant_nocxite                    * config.asteroid_price(&Asteroid::FragrantNocxite)) +
            (intoxicating_nocxite                * config.asteroid_price(&Asteroid::IntoxicatingNocxite)) +
            (ambrosial_nocxite                   * config.asteroid_price(&Asteroid::AmbrosialNocxite)) +
            (compressed_nocxite                  * config.asteroid_price(&Asteroid::CompressedNocxite)) +
            (compressed_fragrant_nocxite         * config.asteroid_price(&Asteroid::CompressedFragrantNocxite)) +
            (compressed_intoxicating_nocxite     * config.asteroid_price(&Asteroid::CompressedIntoxicatingNocxite)) +
            (compressed_ambrosial_nocxite        * config.asteroid_price(&Asteroid::CompressedAmbrosialNocxite)) +

            (omber                               * config.asteroid_price(&Asteroid::Omber)) +
            (silvery_omber                       * config.asteroid_price(&Asteroid::SilveryOmber)) +
            (golden_omber                        * config.asteroid_price(&Asteroid::GoldenOmber)) +
            (platinoid_omber                     * config.asteroid_price(&Asteroid::PlatinoidOmber)) +
            (compressed_omber                    * config.asteroid_price(&Asteroid::CompressedOmber)) +
            (compressed_silvery_omber            * config.asteroid_price(&Asteroid::CompressedSilveryOmber)) +
            (compressed_golden_omber             * config.asteroid_price(&Asteroid::CompressedGoldenOmber)) +
            (compressed_platinoid_omber          * config.asteroid_price(&Asteroid::CompressedPlatinoidOmber)) +

            (plagioclase                         * config.asteroid_price(&Asteroid::Plagioclase)) +
            (azure_plagioclase                   * config.asteroid_price(&Asteroid::AzurePlagioclase)) +
            (rich_plagioclase                    * config.asteroid_price(&Asteroid::RichPlagioclase)) +
            (sparkling_plagioclase               * config.asteroid_price(&Asteroid::SparklingPlagioclase)) + 
            (compressed_plagioclase              * config.asteroid_price(&Asteroid::CompressedPlagioclase)) +
            (compressed_azure_plagioclase        * config.asteroid_price(&Asteroid::CompressedAzurePlagioclase)) +
            (compressed_rich_plagioclase         * config.asteroid_price(&Asteroid::CompressedRichPlagioclase)) +
            (compressed_sparkling_plagioclase    * config.asteroid_price(&Asteroid::CompressedSparklingPlagioclase)) + 

            (pyroxeres                           * config.asteroid_price(&Asteroid::Pyroxeres)) +
            (solid_pyroxeres                     * config.asteroid_price(&Asteroid::SolidPyroxeres)) +
            (viscous_pyroxeres                   * config.asteroid_price(&Asteroid::ViscousPyroxeres)) +
            (opulent_pyroxeres                   * config.asteroid_price(&Asteroid::OpulentPyroxeres)) +
            (compressed_pyroxeres                * config.asteroid_price(&Asteroid::CompressedPyroxeres)) +
            (compressed_solid_pyroxeres          * config.asteroid_price(&Asteroid::CompressedSolidPyroxeres)) +
            (compressed_viscous_pyroxeres        * config.asteroid_price(&Asteroid::CompressedViscousPyroxeres)) +
            (compressed_opulent_pyroxeres        * config.asteroid_price(&Asteroid::CompressedOpulentPyroxeres)) +

            (rakovene                            * config.asteroid_price(&Asteroid::Rakovene)) +
            (abyssal_rakovene                    * config.asteroid_price(&Asteroid::AbyssalRakovene)) +
            (hadal_rakovene                      * config.asteroid_price(&Asteroid::HadalRakovene)) +
            (compressed_rakovene                 * config.asteroid_price(&Asteroid::CompressedRakovene)) +
            (compressed_abyssal_rakovene         * config.asteroid_price(&Asteroid::CompressedAbyssalRakovene)) +
            (compressed_hadal_rakovene           * config.asteroid_price(&Asteroid::CompressedHadalRakovene)) +

            (scordite                            * config.asteroid_price(&Asteroid::Scordite)) +
            (condensed_scordite                  * config.asteroid_price(&Asteroid::CondensedScordite)) +
            (massive_scordite                    * config.asteroid_price(&Asteroid::MassiveScordite)) +
            (glossy_scordite                     * config.asteroid_price(&Asteroid::GlossyScordite)) +
            (compressed_scordite                 * config.asteroid_price(&Asteroid::CompressedScordite)) +
            (compressed_condensed_scordite       * config.asteroid_price(&Asteroid::CompressedCondensedScordite)) +
            (compressed_massive_scordite         * config.asteroid_price(&Asteroid::CompressedMassiveScordite)) +
            (compressed_glossy_scordite          * config.asteroid_price(&Asteroid::CompressedGlossyScordite)) +

            (spodumain                           * config.asteroid_price(&Asteroid::Spodumain)) +
            (bright_spodumain                    * config.asteroid_price(&Asteroid::BrightSpodumain)) +
            (gleaming_spodumain                  * config.asteroid_price(&Asteroid::GleamingSpodumain)) +
            (dazzling_spodumain                  * config.asteroid_price(&Asteroid::DazzlingSpodumain)) +
            (compressed_spodumain                * config.asteroid_price(&Asteroid::CompressedSpodumain)) +
            (compressed_bright_spodumain         * config.asteroid_price(&Asteroid::CompressedBrightSpodumain)) +
            (compressed_gleaming_spodumain       * config.asteroid_price(&Asteroid::CompressedGleamingSpodumain)) +
            (compressed_dazzling_spodumain       * config.asteroid_price(&Asteroid::CompressedDazzlingSpodumain)) +

            (talassonite                         * config.asteroid_price(&Asteroid::Talassonite)) +
            (abyssal_talassonite                 * config.asteroid_price(&Asteroid::AbyssalTalassonite)) +
            (hadal_talassonite                   * config.asteroid_price(&Asteroid::HadalTalassonite)) +
            (compressed_talassonite              * config.asteroid_price(&Asteroid::CompressedTalassonite)) +
            (compressed_abyssal_talassonite      * config.asteroid_price(&Asteroid::CompressedAbyssalTalassonite)) +
            (compressed_hadal_talassonite        * config.asteroid_price(&Asteroid::CompressedHadalTalassonite)) +

            (ueganite                            * config.asteroid_price(&Asteroid::Ueganite)) +
            (foggy_ueganite                      * config.asteroid_price(&Asteroid::FoggyUeganite)) +
            (overcast_ueganite                   * config.asteroid_price(&Asteroid::OvercastUeganite)) +
            (stormy_ueganite                     * config.asteroid_price(&Asteroid::StormyUeganite)) +
            (compressed_ueganite                 * config.asteroid_price(&Asteroid::CompressedUeganite)) +
            (compressed_foggy_ueganite           * config.asteroid_price(&Asteroid::CompressedFoggyUeganite)) +
            (compressed_overcast_ueganite        * config.asteroid_price(&Asteroid::CompressedOvercastUeganite)) +
            (compressed_stormy_ueganite          * config.asteroid_price(&Asteroid::CompressedStormyUeganite)) +

            (plagioclase                         * config.asteroid_price(&Asteroid::Plagioclase)) +
            (azure_plagioclase                   * config.asteroid_price(&Asteroid::AzurePlagioclase)) +
            (rich_plagioclase                    * config.asteroid_price(&Asteroid::RichPlagioclase)) +
            (sparkling_plagioclase               * config.asteroid_price(&Asteroid::SparklingPlagioclase)) +
            (compressed_plagioclase              * config.asteroid_price(&Asteroid::CompressedPlagioclase)) +
            (compressed_azure_plagioclase        * config.asteroid_price(&Asteroid::CompressedAzurePlagioclase)) +
            (compressed_rich_plagioclase         * config.asteroid_price(&Asteroid::CompressedRichPlagioclase)) +
            (compressed_sparkling_plagioclase    * config.asteroid_price(&Asteroid::CompressedSparklingPlagioclase)) +

            (veldspar                            * config.asteroid_price(&Asteroid::Veldspar)) +
            (concentrated_veldspar               * config.asteroid_price(&Asteroid::ConcentratedVeldspar)) +
            (dense_veldspar                      * config.asteroid_price(&Asteroid::DenseVeldspar)) +
            (stable_veldspar                     * config.asteroid_price(&Asteroid::StableVeldspar)) +
            (compressed_veldspar                 * config.asteroid_price(&Asteroid::CompressedVeldspar)) +
            (compressed_concentrated_veldspar    * config.asteroid_price(&Asteroid::CompressedConcentratedVeldspar)) +
            (compressed_dense_veldspar           * config.asteroid_price(&Asteroid::CompressedDenseVeldspar)) +
            (compressed_stable_veldspar          * config.asteroid_price(&Asteroid::CompressedStableVeldspar)) +

            (ytirium                             * config.asteroid_price(&Asteroid::Ytirium)) +
            (bootleg_ytirium                     * config.asteroid_price(&Asteroid::BootlegYtirium)) +
            (firewater_ytirium                   * config.asteroid_price(&Asteroid::FirewaterYtirium)) +
            (moonshine_ytirium                   * config.asteroid_price(&Asteroid::MoonshineYtirium)) +
            (compressed_ytirium                  * config.asteroid_price(&Asteroid::CompressedYtirium)) +
            (compressed_bootleg_ytirium          * config.asteroid_price(&Asteroid::CompressedBootlegYtirium)) +
            (compressed_firewater_ytirium        * config.asteroid_price(&Asteroid::CompressedFirewaterYtirium)) +
            (compressed_moonshine_ytirium        * config.asteroid_price(&Asteroid::CompressedMoonshineYtirium)) +

            (bitumens                            * config.asteroid_price(&Asteroid::Bitumens)) +
            (brimful_bitumens                    * config.asteroid_price(&Asteroid::BrimfulBitumens)) +
            (glistening_bitumens                 * config.asteroid_price(&Asteroid::GlisteningBitumens)) +
            (compressed_bitumens                 * config.asteroid_price(&Asteroid::CompressedBitumens)) +
            (compressed_brimful_bitumens         * config.asteroid_price(&Asteroid::CompressedBrimfulBitumens)) +
            (compressed_glistering_bitumens      * config.asteroid_price(&Asteroid::CompressedGlisteningBitumens)) +

            (coesite                             * config.asteroid_price(&Asteroid::Coesite)) +
            (brimful_coesite                     * config.asteroid_price(&Asteroid::BrimfulCoesite)) +
            (glistening_coesite                  * config.asteroid_price(&Asteroid::GlisteningCoesite)) +
            (compressed_coesite                  * config.asteroid_price(&Asteroid::CompressedCoesite)) +
            (compressed_brimful_coesite          * config.asteroid_price(&Asteroid::CompressedBrimfulCoesite)) +
            (compressed_glistering_coesite       * config.asteroid_price(&Asteroid::CompressedGlisteningCoesite)) +

            (sylvite                             * config.asteroid_price(&Asteroid::Sylvite)) +
            (brimful_sylvite                     * config.asteroid_price(&Asteroid::BrimfulSylvite)) +
            (glistening_sylvite                  * config.asteroid_price(&Asteroid::GlisteningSylvite)) +
            (compressed_sylvite                  * config.asteroid_price(&Asteroid::CompressedSylvite)) +
            (compressed_brimful_sylvite          * config.asteroid_price(&Asteroid::CompressedBrimfulSylvite)) +
            (compressed_glistering_sylvite       * config.asteroid_price(&Asteroid::CompressedGlisteningSylvite)) +

            (zeolites                            * config.asteroid_price(&Asteroid::Zeolites)) +
            (brimful_zeolites                    * config.asteroid_price(&Asteroid::BrimfulZeolites)) +
            (glistening_zeolites                 * config.asteroid_price(&Asteroid::GlisteningZeolites)) +
            (compressed_zeolites                 * config.asteroid_price(&Asteroid::CompressedZeolites)) +
            (compressed_brimful_zeolites         * config.asteroid_price(&Asteroid::CompressedBrimfulZeolites)) +
            (compressed_glistering_zeolites      * config.asteroid_price(&Asteroid::CompressedGlisteningZeolites)) +

            (cobaltite                           * config.asteroid_price(&Asteroid::Cobaltite)) +
            (copious_cobaltite                   * config.asteroid_price(&Asteroid::CopiousCobaltite)) +
            (twinkling_cobaltite                 * config.asteroid_price(&Asteroid::TwinklingCobaltite)) +
            (compressed_cobaltite                * config.asteroid_price(&Asteroid::CompressedCobaltite)) +
            (compressed_copious_cobaltite        * config.asteroid_price(&Asteroid::CompressedCopiousCobaltite)) +
            (compressed_twinkling_cobaltite      * config.asteroid_price(&Asteroid::CompressedTwinklingCobaltite)) +

            (euxenite                            * config.asteroid_price(&Asteroid::Euxenite)) +
            (copious_euxenite                    * config.asteroid_price(&Asteroid::CopiousEuxenite)) +
            (twinkling_euxenite                  * config.asteroid_price(&Asteroid::TwinklingEuxenite)) +
            (compressed_euxenite                 * config.asteroid_price(&Asteroid::CompressedEuxenite)) +
            (compressed_copious_euxenite         * config.asteroid_price(&Asteroid::CompressedCopiousEuxenite)) +
            (compressed_twinkling_euxenite       * config.asteroid_price(&Asteroid::CompressedTwinklingEuxenite)) +

            (scheelite                           * config.asteroid_price(&Asteroid::Scheelite)) +
            (copious_scheelite                   * config.asteroid_price(&Asteroid::CopiousScheelite)) +
            (twinkling_scheelite                 * config.asteroid_price(&Asteroid::TwinklingScheelite)) +
            (compressed_scheelite                * config.asteroid_price(&Asteroid::CompressedScheelite)) +
            (compressed_copious_scheelite        * config.asteroid_price(&Asteroid::CompressedCopiousScheelite)) +
            (compressed_twinkling_scheelite      * config.asteroid_price(&Asteroid::CompressedTwinklingScheelite)) +

            (titanite                            * config.asteroid_price(&Asteroid::Titanite)) +
            (copious_titanite                    * config.asteroid_price(&Asteroid::CopiousTitanite)) +
            (twinkling_titanite                  * config.asteroid_price(&Asteroid::TwinklingTitanite)) +
            (compressed_titanite                 * config.asteroid_price(&Asteroid::CompressedTitanite)) +
            (compressed_copious_titanite         * config.asteroid_price(&Asteroid::CompressedCopiousTitanite)) +
            (compressed_twinkling_titanite       * config.asteroid_price(&Asteroid::CompressedTwinklingTitanite)) +

            (chromite                            * config.asteroid_price(&Asteroid::Chromite)) +
            (lavish_chromite                     * config.asteroid_price(&Asteroid::LavishChromite)) +
            (shimmering_chromite                 * config.asteroid_price(&Asteroid::ShimmeringChromite)) +
            (compressed_chromite                 * config.asteroid_price(&Asteroid::CompressedChromite)) +
            (compressed_lavish_chromite          * config.asteroid_price(&Asteroid::CompressedLavishChromite)) +
            (compressed_shimmering_chromite      * config.asteroid_price(&Asteroid::CompressedShimmeringChromite)) +

            (otavite                             * config.asteroid_price(&Asteroid::Otavite)) +
            (lavish_otavite                      * config.asteroid_price(&Asteroid::LavishOtavite)) +
            (shimmering_otavite                  * config.asteroid_price(&Asteroid::ShimmeringOtavite)) +
            (compressed_otavite                  * config.asteroid_price(&Asteroid::CompressedOtavite)) +
            (compressed_lavish_otavite           * config.asteroid_price(&Asteroid::CompressedLavishOtavite)) +
            (compressed_shimmering_otavite       * config.asteroid_price(&Asteroid::CompressedShimmeringOtavite)) +

            (sperrylite                          * config.asteroid_price(&Asteroid::Sperrylite)) +
            (lavish_sperrylite                   * config.asteroid_price(&Asteroid::LavishSperrylite)) +
            (shimmering_sperrylite               * config.asteroid_price(&Asteroid::ShimmeringSperrylite)) +
            (compressed_sperrylite               * config.asteroid_price(&Asteroid::CompressedSperrylite)) +
            (compressed_lavish_sperrylite        * config.asteroid_price(&Asteroid::CompressedLavishSperrylite)) +
            (compressed_shimmering_sperrylite    * config.asteroid_price(&Asteroid::CompressedShimmeringSperrylite)) +

            (vanadinite                          * config.asteroid_price(&Asteroid::Vanadinite)) +
            (lavish_vanadinite                   * config.asteroid_price(&Asteroid::LavishVanadinite)) +
            (shimmering_vanadinite               * config.asteroid_price(&Asteroid::ShimmeringVanadinite)) +
            (compressed_vanadinite               * config.asteroid_price(&Asteroid::CompressedVanadinite)) +
            (compressed_lavish_vanadinite        * config.asteroid_price(&Asteroid::CompressedLavishVanadinite)) +
            (compressed_shimmering_vanadinite    * config.asteroid_price(&Asteroid::CompressedShimmeringVanadinite)) +

            (carnotite                           * config.asteroid_price(&Asteroid::Carnotite)) +
            (replete_carnotite                   * config.asteroid_price(&Asteroid::RepleteCarnotite)) +
            (glowing_carnotite                   * config.asteroid_price(&Asteroid::GlowingCarnotite)) +
            (compressed_carnotite                * config.asteroid_price(&Asteroid::CompressedCarnotite)) +
            (compressed_replete_carnotite        * config.asteroid_price(&Asteroid::CompressedRepleteCarnotite)) +
            (compressed_glowing_carnotite        * config.asteroid_price(&Asteroid::CompressedGlowingCarnotite)) +

            (cinnabar                            * config.asteroid_price(&Asteroid::Cinnabar)) +
            (replete_cinnabar                    * config.asteroid_price(&Asteroid::RepleteCinnabar)) +
            (glowing_cinnabar                    * config.asteroid_price(&Asteroid::GlowingCinnabar)) +
            (compressed_cinnabar                 * config.asteroid_price(&Asteroid::CompressedCinnabar)) +
            (compressed_replete_cinnabar         * config.asteroid_price(&Asteroid::CompressedRepleteCinnabar)) +
            (compressed_glowing_cinnabar         * config.asteroid_price(&Asteroid::CompressedGlowingCinnabar)) +

            (pollucite                           * config.asteroid_price(&Asteroid::Pollucite)) +
            (replete_pollucite                   * config.asteroid_price(&Asteroid::RepletePollucite)) +
            (glowing_pollucite                   * config.asteroid_price(&Asteroid::GlowingPollucite)) +
            (compressed_pollucite                * config.asteroid_price(&Asteroid::CompressedPollucite)) +
            (compressed_replete_pollucite        * config.asteroid_price(&Asteroid::CompressedRepletePollucite)) +
            (compressed_glowing_pollucite        * config.asteroid_price(&Asteroid::CompressedGlowingPollucite)) +

            (zircon                              * config.asteroid_price(&Asteroid::Zircon)) +
            (replete_zircon                      * config.asteroid_price(&Asteroid::RepleteZircon)) +
            (glowing_zircon                      * config.asteroid_price(&Asteroid::GlowingZircon)) +
            (compressed_zircon                   * config.asteroid_price(&Asteroid::CompressedZircon)) +
            (compressed_replete_zircon           * config.asteroid_price(&Asteroid::CompressedRepleteZircon)) +
            (compressed_glowing_zircon           * config.asteroid_price(&Asteroid::CompressedGlowingZircon)) +

            (loparite                            * config.asteroid_price(&Asteroid::Loparite)) +
            (bountiful_loparite                  * config.asteroid_price(&Asteroid::BountifulLoparite)) +
            (shining_loparite                    * config.asteroid_price(&Asteroid::ShiningLoparite)) +
            (compressed_loparite                 * config.asteroid_price(&Asteroid::CompressedLoparite)) +
            (compressed_bountiful_loparite       * config.asteroid_price(&Asteroid::CompressedBountifulLoparite)) +
            (compressed_shining_loparite         * config.asteroid_price(&Asteroid::CompressedShiningLoparite)) +

            (monazite                            * config.asteroid_price(&Asteroid::Monazite)) +
            (bountiful_monazite                  * config.asteroid_price(&Asteroid::BountifulMonazite)) +
            (shining_monazite                    * config.asteroid_price(&Asteroid::ShiningMonazite)) +
            (compressed_monazite                 * config.asteroid_price(&Asteroid::CompressedMonazite)) +
            (compressed_bountiful_monazite       * config.asteroid_price(&Asteroid::CompressedBountifulMonazite)) +
            (compressed_shining_monazite         * config.asteroid_price(&Asteroid::CompressedShiningMonazite)) +

            (xenotime                            * config.asteroid_price(&Asteroid::Xenotime)) +
            (bountiful_xenotime                  * config.asteroid_price(&Asteroid::BountifulXenotime)) +
            (shining_xenotime                    * config.asteroid_price(&Asteroid::ShiningXenotime)) +
            (compressed_xenotime                 * config.asteroid_price(&Asteroid::CompressedXenotime)) +
            (compressed_bountiful_xenotime       * config.asteroid_price(&Asteroid::CompressedBountifulXenotime)) +
            (compressed_shining_xenotime         * config.asteroid_price(&Asteroid::CompressedShiningXenotime)) +

            (ytterbite                           * config.asteroid_price(&Asteroid::Ytterbite)) +
            (bountiful_ytterbite                 * config.asteroid_price(&Asteroid::BountifulYtterbite)) +
            (shining_ytterbite                   * config.asteroid_price(&Asteroid::ShiningYtterbite)) +
            (compressed_ytterbite                * config.asteroid_price(&Asteroid::CompressedYtterbite)) +
            (compressed_bountiful_ytterbite      * config.asteroid_price(&Asteroid::CompressedBountifulYtterbite)) +
            (compressed_shining_ytterbite        * config.asteroid_price(&Asteroid::CompressedShiningYtterbite)) +

            (tritanium          * config.asteroid_price(&Asteroid::Tritanium)) +
            (pyerite            * config.asteroid_price(&Asteroid::Pyerite)) +
            (mexallon           * config.asteroid_price(&Asteroid::Mexallon)) +
            (isogen             * config.asteroid_price(&Asteroid::Isogen)) +
            (nocxium            * config.asteroid_price(&Asteroid::Nocxium)) +
            (zydrine            * config.asteroid_price(&Asteroid::Zydrine)) +
            (megacyte           * config.asteroid_price(&Asteroid::Megacyte)) +
            (morphite           * config.asteroid_price(&Asteroid::Morphite)) +

            (atmospheric_gases  * config.asteroid_price(&Asteroid::AtmosphericGases)) +
            (evaporite_deposits * config.asteroid_price(&Asteroid::EvaporiteDeposits)) +
            (hydrocarbons       * config.asteroid_price(&Asteroid::Hydrocarbons)) +
            (silicates          * config.asteroid_price(&Asteroid::Silicates)) +
            (cobalt             * config.asteroid_price(&Asteroid::Cobalt)) +
            (scandium           * config.asteroid_price(&Asteroid::Scandium)) +
            (titanium           * config.asteroid_price(&Asteroid::Titanium)) +
            (tungsten           * config.asteroid_price(&Asteroid::Tungsten))+
            (chromium           * config.asteroid_price(&Asteroid::Chromium)) +
            (cadmium            * config.asteroid_price(&Asteroid::Cadmium)) +
            (platinum           * config.asteroid_price(&Asteroid::Platinum)) +
            (vanadium           * config.asteroid_price(&Asteroid::Vanadium)) +
            (caesium            * config.asteroid_price(&Asteroid::Caesium)) +
            (hafnium            * config.asteroid_price(&Asteroid::Hafnium)) +
            (mercury            * config.asteroid_price(&Asteroid::Mercury)) +
            (technetium         * config.asteroid_price(&Asteroid::Technetium))+
            (promethium         * config.asteroid_price(&Asteroid::Promethium)) +
            (neodymium          * config.asteroid_price(&Asteroid::Neodymium)) +
            (dysprosium         * config.asteroid_price(&Asteroid::Dysprosium)) +
            (thulium            * config.asteroid_price(&Asteroid::Thulium))
        )
        .using(default_solver);

    let entries = vec![
        (Asteroid::Tritanium, tritanium),
        (Asteroid::Pyerite, pyerite),
        (Asteroid::Mexallon, mexallon),
        (Asteroid::Isogen, isogen),
        (Asteroid::Nocxium, nocxium),
        (Asteroid::Zydrine, zydrine),
        (Asteroid::Megacyte, megacyte),
        (Asteroid::Morphite, morphite),

        (Asteroid::AtmosphericGases, atmospheric_gases),
        (Asteroid::EvaporiteDeposits, evaporite_deposits),
        (Asteroid::Hydrocarbons, hydrocarbons),
        (Asteroid::Silicates, silicates),
        (Asteroid::Cobalt, cobalt),
        (Asteroid::Scandium, scandium),
        (Asteroid::Titanium, titanium),
        (Asteroid::Tungsten, tungsten),
        (Asteroid::Chromium, chromium),
        (Asteroid::Cadmium, cadmium),
        (Asteroid::Platinum, platinum),
        (Asteroid::Vanadium, vanadium),
        (Asteroid::Caesium, caesium),
        (Asteroid::Hafnium, hafnium),
        (Asteroid::Mercury, mercury),
        (Asteroid::Technetium, technetium),
        (Asteroid::Promethium, promethium),
        (Asteroid::Neodymium, neodymium),
        (Asteroid::Dysprosium, dysprosium),
        (Asteroid::Thulium, thulium),

        (Asteroid::Arkonor, arkonor),
        (Asteroid::CrimsonArkonor, crimson_arkonor),
        (Asteroid::PrimeArkonor, prime_arkonor),
        (Asteroid::FlawlessArkonor, flawless_arkonor),
        (Asteroid::CompressedArkonor, compressed_arkonor),
        (Asteroid::CompressedCrimsonArkonor, compressed_crimson_arkonor),
        (Asteroid::CompressedPrimeArkonor, compressed_prime_arkonor),
        (Asteroid::CompressedFlawlessArkonor, compressed_flawless_arkonor),

        (Asteroid::Bezdnacine, bezdnacine),
        (Asteroid::AbyssalBezdnacine, abyssal_bezdnacine),
        (Asteroid::HadalBezdnacine, hadal_bezdnacine),
        (Asteroid::CompressedBezdnacine, compressed_bezdnacine),
        (Asteroid::CompressedAbyssalBezdnacine, compressed_abyssal_bezdnacine),
        (Asteroid::CompressedHadalBezdnacine, compressed_hadal_bezdnacine),

        (Asteroid::Bistot, bistot),
        (Asteroid::TriclinicBistot, triclinic_bistot),
        (Asteroid::MonoclinicBistot, monoclinic_bistot),
        (Asteroid::CubicBistot, cubic_bistot),
        (Asteroid::CompressedBistot, compressed_bistot),
        (Asteroid::CompressedTriclinicBistot, compressed_triclinic_bistot),
        (Asteroid::CompressedMonoclinicBistot, compressed_monoclinic_bistot),
        (Asteroid::CompressedCubicBistot, compressed_cubic_bistot),

        (Asteroid::Crokite, crokite),
        (Asteroid::SharpCrokite, sharp_crokite),
        (Asteroid::CrystallineCrokite, crystalline_crokite),
        (Asteroid::PellucidCrokite, pellucid_crokite),
        (Asteroid::CompressedCrokite, compressed_crokite),
        (Asteroid::CompressedSharpCrokite, compressed_sharp_crokite),
        (Asteroid::CompressedCrystallineCrokite, compressed_crystalline_crokite),
        (Asteroid::CompressedPellucidCrokite, compressed_pellucid_crokite),

        (Asteroid::DarkOchre, dark_ochre),
        (Asteroid::OnyxOchre, onyx_ochre),
        (Asteroid::ObsidianOchre, obsidian_ochre),
        (Asteroid::JetOchre, jet_ochre),
        (Asteroid::CompressedDarkOchre, compressed_dark_ochre),
        (Asteroid::CompressedOnyxOchre, compressed_onyx_ochre),
        (Asteroid::CompressedObsidianOchre, compressed_obsidian_ochre),
        (Asteroid::CompressedJetOchre, compressed_jet_ochre),

        (Asteroid::Ducinium, ducinium),
        (Asteroid::NobleDucinium, noble_ducinium),
        (Asteroid::RoyalDucinium, royal_ducinium),
        (Asteroid::ImperialDucinium, imperial_ducinium),
        (Asteroid::CompressedDucinium, compressed_ducinium),
        (Asteroid::CompressedNobleDucinium, compressed_noble_ducinium),
        (Asteroid::CompressedRoyalDucinium, compressed_royal_ducinium),
        (Asteroid::CompressedImperialDucinium, compressed_imperial_ducinium),

        (Asteroid::Eifyrium, eifyrium),
        (Asteroid::DopedEifyrium, doped_eifyrium),
        (Asteroid::BoostedEifyrium, boosted_eifyrium),
        (Asteroid::AugmentedEifyrium, augmented_eifyrium),
        (Asteroid::CompressedEifyrium, compressed_eifyrium),
        (Asteroid::CompressedDopedEifyrium, compressed_doped_eifyrium),
        (Asteroid::CompressedBoostedEifyrium, compressed_boosted_eifyrium),
        (Asteroid::CompressedAugmentedEifyrium, compressed_augmented_eifyrium),

        (Asteroid::Gneiss, gneiss),
        (Asteroid::IridescentGneiss, iridescent_gneiss),
        (Asteroid::PrismaticGneiss, prismatic_gneiss),
        (Asteroid::BrilliantGneiss, brilliant_gneiss),
        (Asteroid::CompressedGneiss, compressed_gneiss),
        (Asteroid::CompressedIridescentGneiss, compressed_iridescent_gneiss),
        (Asteroid::CompressedPrismaticGneiss, compressed_prismatic_gneiss),
        (Asteroid::CompressedBrilliantGneiss, compressed_brilliant_gneiss),

        (Asteroid::Griemeer, griemeer),
        (Asteroid::ClearGriemeer, clear_griemeer),
        (Asteroid::InkyGriemeer, inky_griemeer),
        (Asteroid::OpaqueGriemeer, opaque_griemeer),
        (Asteroid::CompressedGriemeer, compressed_griemeer),
        (Asteroid::CompressedClearGriemeer, compressed_clear_griemeer),
        (Asteroid::CompressedInkyGriemeer, compressed_inky_griemeer),
        (Asteroid::CompressedOpaqueGriemeer, compressed_opaque_griemeer),

        (Asteroid::Hedbergite, hedbergite),
        (Asteroid::VitricHedbergite, vitric_hedbergite),
        (Asteroid::GlazedHedbergite, glazed_hedbergite),
        (Asteroid::LustrousHedbergite, lustrous_hedbergite),
        (Asteroid::CompressedHedbergite, compressed_hedbergite),
        (Asteroid::CompressedVitricHedbergite, compressed_vitric_hedbergite),
        (Asteroid::CompressedGlazedHedbergite, compressed_glazed_hedbergite),
        (Asteroid::CompressedLustrousHedbergite, compressed_lustrous_hedbergite),

        (Asteroid::Hemorphite, hemorphite),
        (Asteroid::VividHemorphite, vivid_hemorphite),
        (Asteroid::RadiantHemorphite, radiant_hemorphite),
        (Asteroid::ScintillatingHemorphite, scintillating_hemorphite),
        (Asteroid::CompressedHemorphite, compressed_hemorphite),
        (Asteroid::CompressedVividHemorphite, compressed_vivid_hemorphite),
        (Asteroid::CompressedRadiantHemorphite, compressed_radiant_hemorphite),
        (Asteroid::CompressedScintillatingHemorphite, compressed_scintillating_hemorphite),

        (Asteroid::Hezorime, hezorime),
        (Asteroid::DullHezorime, dull_hezorime),
        (Asteroid::SerratedHezorime, serrated_hezorime),
        (Asteroid::SharpHezorime, sharp_hezorime),
        (Asteroid::CompressedHezorime, compressed_hezorime),
        (Asteroid::CompressedDullHezorime, compressed_dull_hezorime),
        (Asteroid::CompressedSerratedHezorime, compressed_serrated_hezorime),
        (Asteroid::CompressedSharpHezorime, compressed_sharp_hezorime),

        (Asteroid::Jaspet, jaspet),
        (Asteroid::PureJaspet, pure_jaspet),
        (Asteroid::PristineJaspet, pristine_jaspet),
        (Asteroid::ImmaculateJaspet, immaculate_jaspet),
        (Asteroid::CompressedJaspet, compressed_jaspet),
        (Asteroid::CompressedPureJaspet, compressed_pure_jaspet),
        (Asteroid::CompressedPristineJaspet, compressed_pristine_jaspet),
        (Asteroid::CompressedImmaculateJaspet, compressed_immaculate_jaspet),

        (Asteroid::Kernite, kernite),
        (Asteroid::LuminousKernite, luminous_kernite),
        (Asteroid::FieryKernite, fiery_kernite),
        (Asteroid::ResplendantKernite, resplendant_kernite),
        (Asteroid::CompressedKernite, compressed_kernite),
        (Asteroid::CompressedLuminousKernite, compressed_luminous_kernite),
        (Asteroid::CompressedFieryKernite, compressed_fiery_kernite),
        (Asteroid::CompressedResplendantKernite, compressed_resplendant_kernite),

        (Asteroid::Kylixium, kylixium),
        (Asteroid::KaolinKylixium, kaolin_kylixium),
        (Asteroid::ArgilKylixium, argil_kylixium),
        (Asteroid::AdobeKylixium, adobe_kylixium),
        (Asteroid::CompressedKylixium, compressed_kylixium),
        (Asteroid::CompressedKaolinKylixium, compressed_kaolin_kylixium),
        (Asteroid::CompressedArgilKylixium, compressed_argil_kylixium),
        (Asteroid::CompressedAdobeKylixium, compressed_adobe_kylixium),

        (Asteroid::Mercoxit, mercoxit),
        (Asteroid::MagmaMercoxit, magma_mercoxit),
        (Asteroid::VitreousMercoxit, vitreous_mercoxit),
        (Asteroid::CompressedMercoxit, compressed_mercoxit),
        (Asteroid::CompressedMagmaMercoxit, compressed_magma_mercoxit),
        (Asteroid::CompressedVitreousMercoxit, compressed_vitreous_mercoxit),

        (Asteroid::Mordunium, mordunium),
        (Asteroid::PlumMordunium, plum_mordunium),
        (Asteroid::PrizeMordunium, prize_mordunium),
        (Asteroid::PlunderMordunium, plunder_mordunium),
        (Asteroid::CompressedMordunium, compressed_mordunium),
        (Asteroid::CompressedPlumMordunium, compressed_plum_mordunium),
        (Asteroid::CompressedPrizeMordunium, compressed_prize_mordunium),
        (Asteroid::CompressedPlunderMordunium, compressed_plunder_mordunium),

        (Asteroid::Nocxite, nocxite),
        (Asteroid::FragrantNocxite, fragrant_nocxite),
        (Asteroid::IntoxicatingNocxite, intoxicating_nocxite),
        (Asteroid::AmbrosialNocxite, ambrosial_nocxite),
        (Asteroid::CompressedNocxite, compressed_nocxite),
        (Asteroid::CompressedFragrantNocxite, compressed_fragrant_nocxite),
        (Asteroid::CompressedIntoxicatingNocxite, compressed_intoxicating_nocxite),
        (Asteroid::CompressedAmbrosialNocxite, compressed_ambrosial_nocxite),

        (Asteroid::Omber, omber),
        (Asteroid::SilveryOmber, silvery_omber),
        (Asteroid::GoldenOmber, golden_omber),
        (Asteroid::PlatinoidOmber, platinoid_omber),
        (Asteroid::CompressedOmber, compressed_omber),
        (Asteroid::CompressedSilveryOmber, compressed_silvery_omber),
        (Asteroid::CompressedGoldenOmber, compressed_golden_omber),
        (Asteroid::CompressedPlatinoidOmber, compressed_platinoid_omber),

        (Asteroid::Plagioclase, plagioclase),
        (Asteroid::AzurePlagioclase, azure_plagioclase),
        (Asteroid::RichPlagioclase, rich_plagioclase),
        (Asteroid::SparklingPlagioclase, sparkling_plagioclase),
        (Asteroid::CompressedPlagioclase, compressed_plagioclase),
        (Asteroid::CompressedAzurePlagioclase, compressed_azure_plagioclase),
        (Asteroid::CompressedRichPlagioclase, compressed_rich_plagioclase),
        (Asteroid::CompressedSparklingPlagioclase, compressed_sparkling_plagioclase),

        (Asteroid::Pyroxeres, pyroxeres),
        (Asteroid::SolidPyroxeres, solid_pyroxeres),
        (Asteroid::ViscousPyroxeres, viscous_pyroxeres),
        (Asteroid::OpulentPyroxeres, opulent_pyroxeres),
        (Asteroid::CompressedPyroxeres, compressed_pyroxeres),
        (Asteroid::CompressedSolidPyroxeres, compressed_solid_pyroxeres),
        (Asteroid::CompressedViscousPyroxeres, compressed_viscous_pyroxeres),
        (Asteroid::CompressedOpulentPyroxeres, compressed_opulent_pyroxeres),

        (Asteroid::Rakovene, rakovene),
        (Asteroid::AbyssalRakovene, abyssal_rakovene),
        (Asteroid::HadalRakovene, hadal_rakovene),
        (Asteroid::CompressedRakovene, compressed_rakovene),
        (Asteroid::CompressedAbyssalRakovene, compressed_abyssal_rakovene),
        (Asteroid::CompressedHadalRakovene, compressed_hadal_rakovene),

        (Asteroid::Scordite, scordite),
        (Asteroid::CondensedScordite, condensed_scordite),
        (Asteroid::MassiveScordite, massive_scordite),
        (Asteroid::GlossyScordite, glossy_scordite),
        (Asteroid::CompressedScordite, compressed_scordite),
        (Asteroid::CompressedCondensedScordite, compressed_condensed_scordite),
        (Asteroid::CompressedMassiveScordite, compressed_massive_scordite),
        (Asteroid::CompressedGlossyScordite, compressed_glossy_scordite),

        (Asteroid::Spodumain, spodumain),
        (Asteroid::BrightSpodumain, bright_spodumain),
        (Asteroid::GleamingSpodumain, gleaming_spodumain),
        (Asteroid::DazzlingSpodumain, dazzling_spodumain),
        (Asteroid::CompressedSpodumain, compressed_spodumain),
        (Asteroid::CompressedBrightSpodumain, compressed_bright_spodumain),
        (Asteroid::CompressedGleamingSpodumain, compressed_gleaming_spodumain),
        (Asteroid::CompressedDazzlingSpodumain, compressed_dazzling_spodumain),

        (Asteroid::Talassonite, talassonite),
        (Asteroid::AbyssalTalassonite, abyssal_talassonite),
        (Asteroid::HadalTalassonite, hadal_talassonite),
        (Asteroid::CompressedTalassonite, compressed_talassonite),
        (Asteroid::CompressedAbyssalTalassonite, compressed_abyssal_talassonite),
        (Asteroid::CompressedHadalTalassonite, compressed_hadal_talassonite),

        (Asteroid::Ueganite, ueganite),
        (Asteroid::FoggyUeganite, foggy_ueganite),
        (Asteroid::OvercastUeganite, overcast_ueganite),
        (Asteroid::StormyUeganite, stormy_ueganite),
        (Asteroid::CompressedUeganite, compressed_ueganite),
        (Asteroid::CompressedFoggyUeganite, compressed_foggy_ueganite),
        (Asteroid::CompressedOvercastUeganite, compressed_overcast_ueganite),
        (Asteroid::CompressedStormyUeganite, compressed_stormy_ueganite),

        (Asteroid::Veldspar, veldspar),
        (Asteroid::ConcentratedVeldspar, concentrated_veldspar),
        (Asteroid::DenseVeldspar, dense_veldspar),
        (Asteroid::StableVeldspar, stable_veldspar),
        (Asteroid::CompressedVeldspar, compressed_veldspar),
        (Asteroid::CompressedConcentratedVeldspar, compressed_concentrated_veldspar),
        (Asteroid::CompressedDenseVeldspar, compressed_dense_veldspar),
        (Asteroid::CompressedStableVeldspar, compressed_stable_veldspar),

        (Asteroid::Ytirium, ytirium),
        (Asteroid::BootlegYtirium, bootleg_ytirium),
        (Asteroid::FirewaterYtirium, firewater_ytirium),
        (Asteroid::MoonshineYtirium, moonshine_ytirium),
        (Asteroid::CompressedYtirium, compressed_ytirium),
        (Asteroid::CompressedBootlegYtirium, compressed_bootleg_ytirium),
        (Asteroid::CompressedFirewaterYtirium, compressed_firewater_ytirium),
        (Asteroid::CompressedMoonshineYtirium, compressed_moonshine_ytirium),

        (Asteroid::Bitumens, bitumens),
        (Asteroid::BrimfulBitumens, brimful_bitumens),
        (Asteroid::GlisteningBitumens, glistening_bitumens),
        (Asteroid::CompressedBitumens, compressed_bitumens),
        (Asteroid::CompressedBrimfulBitumens, compressed_brimful_bitumens),
        (Asteroid::CompressedGlisteningBitumens, compressed_glistering_bitumens),

        (Asteroid::Coesite, coesite),
        (Asteroid::BrimfulCoesite, brimful_coesite),
        (Asteroid::GlisteningCoesite, glistening_coesite),
        (Asteroid::CompressedCoesite, compressed_coesite),
        (Asteroid::CompressedBrimfulCoesite, compressed_brimful_coesite),
        (Asteroid::CompressedGlisteningCoesite, compressed_glistering_coesite),

        (Asteroid::Sylvite, sylvite),
        (Asteroid::BrimfulSylvite, brimful_sylvite),
        (Asteroid::GlisteningSylvite, glistening_sylvite),
        (Asteroid::CompressedSylvite, compressed_sylvite),
        (Asteroid::CompressedBrimfulSylvite, compressed_brimful_sylvite),
        (Asteroid::CompressedGlisteningSylvite, compressed_glistering_sylvite),

        (Asteroid::Zeolites, zeolites),
        (Asteroid::BrimfulZeolites, brimful_zeolites),
        (Asteroid::GlisteningZeolites, glistening_zeolites),
        (Asteroid::CompressedZeolites, compressed_zeolites),
        (Asteroid::CompressedBrimfulZeolites, compressed_brimful_zeolites),
        (Asteroid::CompressedGlisteningZeolites, compressed_glistering_zeolites),

        (Asteroid::Cobaltite, cobaltite),
        (Asteroid::CopiousCobaltite, copious_cobaltite),
        (Asteroid::TwinklingCobaltite, twinkling_cobaltite),
        (Asteroid::CompressedCobaltite, compressed_cobaltite),
        (Asteroid::CompressedCopiousCobaltite, compressed_copious_cobaltite),
        (Asteroid::CompressedTwinklingCobaltite, compressed_twinkling_cobaltite),

        (Asteroid::Euxenite, euxenite),
        (Asteroid::CopiousEuxenite, copious_euxenite),
        (Asteroid::TwinklingEuxenite, twinkling_euxenite),
        (Asteroid::CompressedEuxenite, compressed_euxenite),
        (Asteroid::CompressedCopiousEuxenite, compressed_copious_euxenite),
        (Asteroid::CompressedTwinklingEuxenite, compressed_twinkling_euxenite),

        (Asteroid::Scheelite, scheelite),
        (Asteroid::CopiousScheelite, copious_scheelite),
        (Asteroid::TwinklingScheelite, twinkling_scheelite),
        (Asteroid::CompressedScheelite, compressed_scheelite),
        (Asteroid::CompressedCopiousScheelite, compressed_copious_scheelite),
        (Asteroid::CompressedTwinklingScheelite, compressed_twinkling_scheelite),

        (Asteroid::Titanite, titanite),
        (Asteroid::CopiousTitanite, copious_titanite),
        (Asteroid::TwinklingTitanite, twinkling_titanite),
        (Asteroid::CompressedTitanite, compressed_titanite),
        (Asteroid::CompressedCopiousTitanite, compressed_copious_titanite),
        (Asteroid::CompressedTwinklingTitanite, compressed_twinkling_titanite),

        (Asteroid::Chromite, chromite),
        (Asteroid::LavishChromite, lavish_chromite),
        (Asteroid::ShimmeringChromite, shimmering_chromite),
        (Asteroid::CompressedChromite, compressed_chromite),
        (Asteroid::CompressedLavishChromite, compressed_lavish_chromite),
        (Asteroid::CompressedShimmeringChromite, compressed_shimmering_chromite),

        (Asteroid::Otavite, otavite),
        (Asteroid::LavishOtavite, lavish_otavite),
        (Asteroid::ShimmeringOtavite, shimmering_otavite),
        (Asteroid::CompressedOtavite, compressed_otavite),
        (Asteroid::CompressedLavishOtavite, compressed_lavish_otavite),
        (Asteroid::CompressedShimmeringOtavite, compressed_shimmering_otavite),

        (Asteroid::Sperrylite, sperrylite),
        (Asteroid::LavishSperrylite, lavish_sperrylite),
        (Asteroid::ShimmeringSperrylite, shimmering_sperrylite),
        (Asteroid::CompressedSperrylite, compressed_sperrylite),
        (Asteroid::CompressedLavishSperrylite, compressed_lavish_sperrylite),
        (Asteroid::CompressedShimmeringSperrylite, compressed_shimmering_sperrylite),

        (Asteroid::Vanadinite, vanadinite),
        (Asteroid::LavishVanadinite, lavish_vanadinite),
        (Asteroid::ShimmeringVanadinite, shimmering_vanadinite),
        (Asteroid::CompressedVanadinite, compressed_vanadinite),
        (Asteroid::CompressedLavishVanadinite, compressed_lavish_vanadinite),
        (Asteroid::CompressedShimmeringVanadinite, compressed_shimmering_vanadinite),

        (Asteroid::Carnotite, carnotite),
        (Asteroid::RepleteCarnotite, replete_carnotite),
        (Asteroid::GlowingCarnotite, glowing_carnotite),
        (Asteroid::CompressedCarnotite, compressed_carnotite),
        (Asteroid::CompressedRepleteCarnotite, compressed_replete_carnotite),
        (Asteroid::CompressedGlowingCarnotite, compressed_glowing_carnotite),

        (Asteroid::Cinnabar, cinnabar),
        (Asteroid::RepleteCinnabar, replete_cinnabar),
        (Asteroid::GlowingCinnabar, glowing_cinnabar),
        (Asteroid::CompressedCinnabar, compressed_cinnabar),
        (Asteroid::CompressedRepleteCinnabar, compressed_replete_cinnabar),
        (Asteroid::CompressedGlowingCinnabar, compressed_glowing_cinnabar),

        (Asteroid::Pollucite, pollucite),
        (Asteroid::RepletePollucite, replete_pollucite),
        (Asteroid::GlowingPollucite, glowing_pollucite),
        (Asteroid::CompressedPollucite, compressed_pollucite),
        (Asteroid::CompressedRepletePollucite, compressed_replete_pollucite),
        (Asteroid::CompressedGlowingPollucite, compressed_glowing_pollucite),

        (Asteroid::Zircon, zircon),
        (Asteroid::RepleteZircon, replete_zircon),
        (Asteroid::GlowingZircon, glowing_zircon),
        (Asteroid::CompressedZircon, compressed_zircon),
        (Asteroid::CompressedRepleteZircon, compressed_replete_zircon),
        (Asteroid::CompressedGlowingZircon, compressed_glowing_zircon),

        (Asteroid::Loparite, loparite),
        (Asteroid::BountifulLoparite, bountiful_loparite),
        (Asteroid::ShiningLoparite, shining_loparite),
        (Asteroid::CompressedLoparite, compressed_loparite),
        (Asteroid::CompressedBountifulLoparite, compressed_bountiful_loparite),
        (Asteroid::CompressedShiningLoparite, compressed_shining_loparite),

        (Asteroid::Monazite, monazite),
        (Asteroid::BountifulMonazite, bountiful_monazite),
        (Asteroid::ShiningMonazite, shining_monazite),
        (Asteroid::CompressedMonazite, compressed_monazite),
        (Asteroid::CompressedBountifulMonazite, compressed_bountiful_monazite),
        (Asteroid::CompressedShiningMonazite, compressed_shining_monazite),

        (Asteroid::Xenotime, xenotime),
        (Asteroid::BountifulXenotime, bountiful_xenotime),
        (Asteroid::ShiningXenotime, shining_xenotime),
        (Asteroid::CompressedXenotime, compressed_xenotime),
        (Asteroid::CompressedBountifulXenotime, compressed_bountiful_xenotime),
        (Asteroid::CompressedShiningXenotime, compressed_shining_xenotime),

        (Asteroid::Ytterbite, ytterbite),
        (Asteroid::BountifulYtterbite, bountiful_ytterbite),
        (Asteroid::ShiningYtterbite, shining_ytterbite),
        (Asteroid::CompressedYtterbite, compressed_ytterbite),
        (Asteroid::CompressedBountifulYtterbite, compressed_bountiful_ytterbite),
        (Asteroid::CompressedShiningYtterbite, compressed_shining_ytterbite),
    ];

    for (asteroid, var) in entries.iter() {
        if config.allowed_asteroid(&asteroid) {
            if config.asteroid_price(&asteroid) >= 0.01 {
                problem = problem
                    .with(constraint!(*var >= 0))
                    .with(constraint!(*var <= config.asteroid_limit(&asteroid)))
            } else {
                problem = problem.with(constraint!(*var == 0))
            }
        } else {
            problem = problem.with(constraint!(*var == 0))
        }
    }

    let solution = problem
        .with(constraint!(
            (bezdnacine                          * (40000f64 * 1.00 * config.reprocessing_asteroid())) +
            (abyssal_bezdnacine                  * (40000f64 * 1.05 * config.reprocessing_asteroid())) +
            (hadal_bezdnacine                    * (40000f64 * 1.10 * config.reprocessing_asteroid())) +
            (compressed_bezdnacine               * (40000f64 * 1.00 * config.reprocessing_asteroid())) +
            (compressed_abyssal_bezdnacine       * (40000f64 * 1.05 * config.reprocessing_asteroid())) +
            (compressed_hadal_bezdnacine         * (40000f64 * 1.10 * config.reprocessing_asteroid())) +

            (plagioclase                         * (175f64 * 1.00 * config.reprocessing_asteroid())) +
            (azure_plagioclase                   * (175f64 * 1.05 * config.reprocessing_asteroid())) +
            (rich_plagioclase                    * (175f64 * 1.10 * config.reprocessing_asteroid())) +
            (sparkling_plagioclase               * (175f64 * 1.15 * config.reprocessing_asteroid())) +
            (compressed_plagioclase              * (175f64 * 1.00 * config.reprocessing_asteroid())) +
            (compressed_azure_plagioclase        * (175f64 * 1.05 * config.reprocessing_asteroid())) +
            (compressed_rich_plagioclase         * (175f64 * 1.10 * config.reprocessing_asteroid())) +
            (compressed_sparkling_plagioclase    * (175f64 * 1.15 * config.reprocessing_asteroid())) +

            (griemeer                            * (250f64 * 1.00 * config.reprocessing_asteroid())) +
            (clear_griemeer                      * (250f64 * 1.05 * config.reprocessing_asteroid())) +
            (inky_griemeer                       * (250f64 * 1.10 * config.reprocessing_asteroid())) +
            (opaque_griemeer                     * (250f64 * 1.15 * config.reprocessing_asteroid())) +
            (compressed_griemeer                 * (250f64 * 1.00 * config.reprocessing_asteroid())) +
            (compressed_clear_griemeer           * (250f64 * 1.05 * config.reprocessing_asteroid())) +
            (compressed_inky_griemeer            * (250f64 * 1.10 * config.reprocessing_asteroid())) +
            (compressed_opaque_griemeer          * (250f64 * 1.15 * config.reprocessing_asteroid())) +

            (hezorime                            * (2000f64 * 1.00 * config.reprocessing_asteroid())) +
            (dull_hezorime                       * (2000f64 * 1.05 * config.reprocessing_asteroid())) +
            (serrated_hezorime                   * (2000f64 * 1.10 * config.reprocessing_asteroid())) +
            (sharp_hezorime                      * (2000f64 * 1.15 * config.reprocessing_asteroid())) +
            (compressed_hezorime                 * (2000f64 * 1.00 * config.reprocessing_asteroid())) +
            (compressed_dull_hezorime            * (2000f64 * 1.05 * config.reprocessing_asteroid())) +
            (compressed_serrated_hezorime        * (2000f64 * 1.10 * config.reprocessing_asteroid())) +
            (compressed_sharp_hezorime           * (2000f64 * 1.15 * config.reprocessing_asteroid())) +

            (kylixium                            * (300f64 * 1.00 * config.reprocessing_asteroid())) +
            (kaolin_kylixium                     * (300f64 * 1.05 * config.reprocessing_asteroid())) +
            (argil_kylixium                      * (300f64 * 1.10 * config.reprocessing_asteroid())) +
            (adobe_kylixium                      * (300f64 * 1.15 * config.reprocessing_asteroid())) +
            (compressed_kylixium                 * (300f64 * 1.00 * config.reprocessing_asteroid())) +
            (compressed_kaolin_kylixium          * (300f64 * 1.05 * config.reprocessing_asteroid())) +
            (compressed_argil_kylixium           * (300f64 * 1.10 * config.reprocessing_asteroid())) +
            (compressed_adobe_kylixium           * (300f64 * 1.15 * config.reprocessing_asteroid())) +

            (nocxite                             * (900f64 * 1.00 * config.reprocessing_asteroid())) +
            (fragrant_nocxite                    * (900f64 * 1.05 * config.reprocessing_asteroid())) +
            (intoxicating_nocxite                * (900f64 * 1.10 * config.reprocessing_asteroid())) +
            (ambrosial_nocxite                   * (900f64 * 1.15 * config.reprocessing_asteroid())) +
            (compressed_nocxite                  * (900f64 * 1.00 * config.reprocessing_asteroid())) +
            (compressed_fragrant_nocxite         * (900f64 * 1.05 * config.reprocessing_asteroid())) +
            (compressed_intoxicating_nocxite     * (900f64 * 1.10 * config.reprocessing_asteroid())) +
            (compressed_ambrosial_nocxite        * (900f64 * 1.15 * config.reprocessing_asteroid())) +

            (rakovene                            * (40000f64 * 1.00 * config.reprocessing_asteroid())) +
            (abyssal_rakovene                    * (40000f64 * 1.05 * config.reprocessing_asteroid())) +
            (hadal_rakovene                      * (40000f64 * 1.10 * config.reprocessing_asteroid())) +
            (compressed_rakovene                 * (40000f64 * 1.00 * config.reprocessing_asteroid())) +
            (compressed_abyssal_rakovene         * (40000f64 * 1.05 * config.reprocessing_asteroid())) +
            (compressed_hadal_rakovene           * (40000f64 * 1.10 * config.reprocessing_asteroid())) +

            (scordite                            * (150f64 * 1.00 * config.reprocessing_asteroid())) +
            (condensed_scordite                  * (150f64 * 1.05 * config.reprocessing_asteroid())) +
            (massive_scordite                    * (150f64 * 1.10 * config.reprocessing_asteroid())) +
            (glossy_scordite                     * (150f64 * 1.15 * config.reprocessing_asteroid())) +
            (compressed_scordite                 * (150f64 * 1.00 * config.reprocessing_asteroid())) +
            (compressed_condensed_scordite       * (150f64 * 1.05 * config.reprocessing_asteroid())) +
            (compressed_massive_scordite         * (150f64 * 1.10 * config.reprocessing_asteroid())) +
            (compressed_glossy_scordite          * (150f64 * 1.15 * config.reprocessing_asteroid())) +

            (spodumain                           * (48000f64 * 1.00 * config.reprocessing_asteroid())) +
            (bright_spodumain                    * (48000f64 * 1.05 * config.reprocessing_asteroid())) +
            (gleaming_spodumain                  * (48000f64 * 1.10 * config.reprocessing_asteroid())) +
            (dazzling_spodumain                  * (48000f64 * 1.15 * config.reprocessing_asteroid())) +
            (compressed_spodumain                * (48000f64 * 1.00 * config.reprocessing_asteroid())) +
            (compressed_bright_spodumain         * (48000f64 * 1.05 * config.reprocessing_asteroid())) +
            (compressed_gleaming_spodumain       * (48000f64 * 1.10 * config.reprocessing_asteroid())) +
            (compressed_dazzling_spodumain       * (48000f64 * 1.15 * config.reprocessing_asteroid())) +

            (talassonite                         * (40000f64 * 1.00 * config.reprocessing_asteroid())) +
            (abyssal_talassonite                 * (40000f64 * 1.05 * config.reprocessing_asteroid())) +
            (hadal_talassonite                   * (40000f64 * 1.10 * config.reprocessing_asteroid())) +
            (compressed_talassonite              * (40000f64 * 1.00 * config.reprocessing_asteroid())) +
            (compressed_abyssal_talassonite      * (40000f64 * 1.05 * config.reprocessing_asteroid())) +
            (compressed_hadal_talassonite        * (40000f64 * 1.10 * config.reprocessing_asteroid())) +

            (ueganite                            * (800f64 * 1.00 * config.reprocessing_asteroid())) +
            (foggy_ueganite                      * (800f64 * 1.05 * config.reprocessing_asteroid())) +
            (overcast_ueganite                   * (800f64 * 1.10 * config.reprocessing_asteroid())) +
            (stormy_ueganite                     * (800f64 * 1.15 * config.reprocessing_asteroid())) +
            (compressed_ueganite                 * (800f64 * 1.00 * config.reprocessing_asteroid())) +
            (compressed_foggy_ueganite           * (800f64 * 1.05 * config.reprocessing_asteroid())) +
            (compressed_overcast_ueganite        * (800f64 * 1.10 * config.reprocessing_asteroid())) +
            (compressed_stormy_ueganite          * (800f64 * 1.15 * config.reprocessing_asteroid())) +

            (veldspar                            * (400f64 * 1.00 * config.reprocessing_asteroid())) +
            (concentrated_veldspar               * (400f64 * 1.05 * config.reprocessing_asteroid())) +
            (dense_veldspar                      * (400f64 * 1.10 * config.reprocessing_asteroid())) +
            (stable_veldspar                     * (400f64 * 1.15 * config.reprocessing_asteroid())) +
            (compressed_veldspar                 * (400f64 * 1.00 * config.reprocessing_asteroid())) +
            (compressed_concentrated_veldspar    * (400f64 * 1.05 * config.reprocessing_asteroid())) +
            (compressed_dense_veldspar           * (400f64 * 1.10 * config.reprocessing_asteroid())) +
            (compressed_stable_veldspar          * (400f64 * 1.15 * config.reprocessing_asteroid())) +

            tritanium >= config.want_mineral(Mineral::Tritanium)
        ).set_name("Tritanium".into()))
        .with(constraint!(
            (arkonor                             * (3200f64 * 1.00 * config.reprocessing_asteroid())) +
            (crimson_arkonor                     * (3200f64 * 1.05 * config.reprocessing_asteroid())) +
            (prime_arkonor                       * (3200f64 * 1.10 * config.reprocessing_asteroid())) +
            (flawless_arkonor                    * (3200f64 * 1.15 * config.reprocessing_asteroid())) +
            (compressed_arkonor                  * (3200f64 * 1.00 * config.reprocessing_asteroid())) +
            (compressed_crimson_arkonor          * (3200f64 * 1.05 * config.reprocessing_asteroid())) +
            (compressed_prime_arkonor            * (3200f64 * 1.10 * config.reprocessing_asteroid())) +
            (compressed_flawless_arkonor         * (3200f64 * 1.15 * config.reprocessing_asteroid())) +

            (bistot                              * (3200f64 * 1.00 * config.reprocessing_asteroid())) +
            (triclinic_bistot                    * (3200f64 * 1.05 * config.reprocessing_asteroid())) +
            (monoclinic_bistot                   * (3200f64 * 1.10 * config.reprocessing_asteroid())) +
            (cubic_bistot                        * (3200f64 * 1.15 * config.reprocessing_asteroid())) +
            (compressed_bistot                   * (3200f64 * 1.00 * config.reprocessing_asteroid())) +
            (compressed_triclinic_bistot         * (3200f64 * 1.05 * config.reprocessing_asteroid())) +
            (compressed_monoclinic_bistot        * (3200f64 * 1.10 * config.reprocessing_asteroid())) +
            (compressed_cubic_bistot             * (3200f64 * 1.15 * config.reprocessing_asteroid())) +

            (crokite                             * (800f64 * 1.00 * config.reprocessing_asteroid())) +
            (sharp_crokite                       * (800f64 * 1.05 * config.reprocessing_asteroid())) +
            (crystalline_crokite                 * (800f64 * 1.10 * config.reprocessing_asteroid())) +
            (pellucid_crokite                    * (800f64 * 1.15 * config.reprocessing_asteroid())) +
            (compressed_crokite                  * (800f64 * 1.00 * config.reprocessing_asteroid())) +
            (compressed_sharp_crokite            * (800f64 * 1.05 * config.reprocessing_asteroid())) +
            (compressed_crystalline_crokite      * (800f64 * 1.10 * config.reprocessing_asteroid())) +
            (compressed_pellucid_crokite         * (800f64 * 1.15 * config.reprocessing_asteroid())) +

            (gneiss                              * (2000f64 * 1.00 * config.reprocessing_asteroid())) +
            (iridescent_gneiss                   * (2000f64 * 1.05 * config.reprocessing_asteroid())) +
            (prismatic_gneiss                    * (2000f64 * 1.10 * config.reprocessing_asteroid())) +
            (brilliant_gneiss                    * (2000f64 * 1.15 * config.reprocessing_asteroid())) +
            (compressed_gneiss                   * (2000f64 * 1.00 * config.reprocessing_asteroid())) +
            (compressed_iridescent_gneiss        * (2000f64 * 1.05 * config.reprocessing_asteroid())) +
            (compressed_prismatic_gneiss         * (2000f64 * 1.10 * config.reprocessing_asteroid())) +
            (compressed_brilliant_gneiss         * (2000f64 * 1.15 * config.reprocessing_asteroid())) +

            (hedbergite                          * (450f64 * 1.00 * config.reprocessing_asteroid())) +
            (vitric_hedbergite                   * (450f64 * 1.05 * config.reprocessing_asteroid())) +
            (glazed_hedbergite                   * (450f64 * 1.10 * config.reprocessing_asteroid())) +
            (lustrous_hedbergite                 * (450f64 * 1.15 * config.reprocessing_asteroid())) +
            (compressed_hedbergite               * (450f64 * 1.00 * config.reprocessing_asteroid())) +
            (compressed_vitric_hedbergite        * (450f64 * 1.05 * config.reprocessing_asteroid())) +
            (compressed_glazed_hedbergite        * (450f64 * 1.10 * config.reprocessing_asteroid())) +
            (compressed_lustrous_hedbergite      * (450f64 * 1.15 * config.reprocessing_asteroid())) +

            (kylixium                            * (200f64 * 1.00 * config.reprocessing_asteroid())) +
            (kaolin_kylixium                     * (200f64 * 1.05 * config.reprocessing_asteroid())) +
            (argil_kylixium                      * (200f64 * 1.10 * config.reprocessing_asteroid())) +
            (adobe_kylixium                      * (200f64 * 1.15 * config.reprocessing_asteroid())) +
            (compressed_kylixium                 * (200f64 * 1.00 * config.reprocessing_asteroid())) +
            (compressed_kaolin_kylixium          * (200f64 * 1.05 * config.reprocessing_asteroid())) +
            (compressed_argil_kylixium           * (200f64 * 1.10 * config.reprocessing_asteroid())) +
            (compressed_adobe_kylixium           * (200f64 * 1.15 * config.reprocessing_asteroid())) +

            (mordunium                           * (84f64 * 1.00 * config.reprocessing_asteroid())) +
            (plum_mordunium                      * (84f64 * 1.05 * config.reprocessing_asteroid())) +
            (prize_mordunium                     * (84f64 * 1.10 * config.reprocessing_asteroid())) +
            (plunder_mordunium                   * (84f64 * 1.15 * config.reprocessing_asteroid())) +
            (compressed_mordunium                * (84f64 * 1.00 * config.reprocessing_asteroid())) +
            (compressed_plum_mordunium           * (84f64 * 1.05 * config.reprocessing_asteroid())) +
            (compressed_prize_mordunium          * (84f64 * 1.10 * config.reprocessing_asteroid())) +
            (compressed_plunder_mordunium        * (84f64 * 1.15 * config.reprocessing_asteroid())) +

            (nocxite                             * (150f64 * 1.00 * config.reprocessing_asteroid())) +
            (fragrant_nocxite                    * (150f64 * 1.05 * config.reprocessing_asteroid())) +
            (intoxicating_nocxite                * (150f64 * 1.10 * config.reprocessing_asteroid())) +
            (ambrosial_nocxite                   * (150f64 * 1.15 * config.reprocessing_asteroid())) +
            (compressed_nocxite                  * (150f64 * 1.00 * config.reprocessing_asteroid())) +
            (compressed_fragrant_nocxite         * (150f64 * 1.05 * config.reprocessing_asteroid())) +
            (compressed_intoxicating_nocxite     * (150f64 * 1.10 * config.reprocessing_asteroid())) +
            (compressed_ambrosial_nocxite        * (150f64 * 1.15 * config.reprocessing_asteroid())) +

            (omber                               * (90f64 * 1.00 * config.reprocessing_asteroid())) +
            (silvery_omber                       * (90f64 * 1.05 * config.reprocessing_asteroid())) +
            (golden_omber                        * (90f64 * 1.10 * config.reprocessing_asteroid())) +
            (platinoid_omber                     * (90f64 * 1.15 * config.reprocessing_asteroid())) +
            (compressed_omber                    * (90f64 * 1.00 * config.reprocessing_asteroid())) +
            (compressed_silvery_omber            * (90f64 * 1.05 * config.reprocessing_asteroid())) +
            (compressed_golden_omber             * (90f64 * 1.10 * config.reprocessing_asteroid())) +
            (compressed_platinoid_omber          * (90f64 * 1.15 * config.reprocessing_asteroid())) +

            (pyroxeres                           * (90f64 * 1.00 * config.reprocessing_asteroid())) +
            (solid_pyroxeres                     * (90f64 * 1.05 * config.reprocessing_asteroid())) +
            (viscous_pyroxeres                   * (90f64 * 1.10 * config.reprocessing_asteroid())) +
            (opulent_pyroxeres                   * (90f64 * 1.15 * config.reprocessing_asteroid())) +
            (compressed_pyroxeres                * (90f64 * 1.00 * config.reprocessing_asteroid())) +
            (compressed_solid_pyroxeres          * (90f64 * 1.05 * config.reprocessing_asteroid())) +
            (compressed_viscous_pyroxeres        * (90f64 * 1.10 * config.reprocessing_asteroid())) +
            (compressed_opulent_pyroxeres        * (90f64 * 1.15 * config.reprocessing_asteroid())) +

            (scordite                            * (90f64 * 1.00 * config.reprocessing_asteroid())) +
            (condensed_scordite                  * (90f64 * 1.05 * config.reprocessing_asteroid())) +
            (massive_scordite                    * (90f64 * 1.10 * config.reprocessing_asteroid())) +
            (glossy_scordite                     * (90f64 * 1.15 * config.reprocessing_asteroid())) +
            (compressed_scordite                 * (90f64 * 1.00 * config.reprocessing_asteroid())) +
            (compressed_condensed_scordite       * (90f64 * 1.05 * config.reprocessing_asteroid())) +
            (compressed_massive_scordite         * (90f64 * 1.10 * config.reprocessing_asteroid())) +
            (compressed_glossy_scordite          * (90f64 * 1.15 * config.reprocessing_asteroid())) +

            (bitumens                            * (6000f64 * 1.00 * config.reprocessing_asteroid())) +
            (brimful_bitumens                    * (6000f64 * 1.15 * config.reprocessing_asteroid())) +
            (glistening_bitumens                 * (6000f64 * 2.00 * config.reprocessing_asteroid())) +
            (compressed_bitumens                 * (6000f64 * 1.00 * config.reprocessing_asteroid())) +
            (compressed_brimful_bitumens         * (6000f64 * 1.15 * config.reprocessing_asteroid())) +
            (compressed_glistering_bitumens      * (6000f64 * 2.00 * config.reprocessing_asteroid())) +

            (coesite                             * (2000f64 * 1.00 * config.reprocessing_asteroid())) +
            (brimful_coesite                     * (2000f64 * 1.15 * config.reprocessing_asteroid())) +
            (glistening_coesite                  * (2000f64 * 2.00 * config.reprocessing_asteroid())) +
            (compressed_coesite                  * (2000f64 * 1.00 * config.reprocessing_asteroid())) +
            (compressed_brimful_coesite          * (2000f64 * 1.15 * config.reprocessing_asteroid())) +
            (compressed_glistering_coesite       * (2000f64 * 2.00 * config.reprocessing_asteroid())) +

            (sylvite                             * (4000f64 * 1.00 * config.reprocessing_asteroid())) +
            (brimful_sylvite                     * (4000f64 * 1.15 * config.reprocessing_asteroid())) +
            (glistening_sylvite                  * (4000f64 * 2.00 * config.reprocessing_asteroid())) +
            (compressed_sylvite                  * (4000f64 * 1.00 * config.reprocessing_asteroid())) +
            (compressed_brimful_sylvite          * (4000f64 * 1.15 * config.reprocessing_asteroid())) +
            (compressed_glistering_sylvite       * (4000f64 * 2.00 * config.reprocessing_asteroid())) +

            (zeolites                            * (8000f64 * 1.00 * config.reprocessing_asteroid())) +
            (brimful_zeolites                    * (8000f64 * 1.15 * config.reprocessing_asteroid())) +
            (glistening_zeolites                 * (8000f64 * 2.00 * config.reprocessing_asteroid())) +
            (compressed_zeolites                 * (8000f64 * 1.00 * config.reprocessing_asteroid())) +
            (compressed_brimful_zeolites         * (8000f64 * 1.15 * config.reprocessing_asteroid())) +
            (compressed_glistering_zeolites      * (8000f64 * 2.00 * config.reprocessing_asteroid())) +

            pyerite >= config.want_mineral(Mineral::Pyerite)
        ).set_name("Pyerite".into()))
        .with(constraint!(
            (arkonor                             * (1200f64 * 1.00 * config.reprocessing_asteroid())) +
            (crimson_arkonor                     * (1200f64 * 1.05 * config.reprocessing_asteroid())) +
            (prime_arkonor                       * (1200f64 * 1.10 * config.reprocessing_asteroid())) +
            (flawless_arkonor                    * (1200f64 * 1.15 * config.reprocessing_asteroid())) +
            (compressed_arkonor                  * (1200f64 * 1.00 * config.reprocessing_asteroid())) +
            (compressed_crimson_arkonor          * (1200f64 * 1.05 * config.reprocessing_asteroid())) +
            (compressed_prime_arkonor            * (1200f64 * 1.10 * config.reprocessing_asteroid())) +
            (compressed_flawless_arkonor         * (1200f64 * 1.15 * config.reprocessing_asteroid())) +

            (bistot                              * (1200f64 * 1.00 * config.reprocessing_asteroid())) +
            (triclinic_bistot                    * (1200f64 * 1.05 * config.reprocessing_asteroid())) +
            (monoclinic_bistot                   * (1200f64 * 1.10 * config.reprocessing_asteroid())) +
            (cubic_bistot                        * (1200f64 * 1.15 * config.reprocessing_asteroid())) +
            (compressed_bistot                   * (1200f64 * 1.00 * config.reprocessing_asteroid())) +
            (compressed_triclinic_bistot         * (1200f64 * 1.05 * config.reprocessing_asteroid())) +
            (compressed_monoclinic_bistot        * (1200f64 * 1.10 * config.reprocessing_asteroid())) +
            (compressed_cubic_bistot             * (1200f64 * 1.15 * config.reprocessing_asteroid())) +

            (crokite                             * (2000f64 * 1.00 * config.reprocessing_asteroid())) +
            (sharp_crokite                       * (2000f64 * 1.05 * config.reprocessing_asteroid())) +
            (crystalline_crokite                 * (2000f64 * 1.10 * config.reprocessing_asteroid())) +
            (pellucid_crokite                    * (2000f64 * 1.15 * config.reprocessing_asteroid())) +
            (compressed_crokite                  * (2000f64 * 1.00 * config.reprocessing_asteroid())) +
            (compressed_sharp_crokite            * (2000f64 * 1.05 * config.reprocessing_asteroid())) +
            (compressed_crystalline_crokite      * (2000f64 * 1.10 * config.reprocessing_asteroid())) +
            (compressed_pellucid_crokite         * (2000f64 * 1.15 * config.reprocessing_asteroid())) +

            (dark_ochre                          * (1360f64 * 1.00 * config.reprocessing_asteroid())) +
            (onyx_ochre                          * (1360f64 * 1.05 * config.reprocessing_asteroid())) +
            (obsidian_ochre                      * (1360f64 * 1.10 * config.reprocessing_asteroid())) +
            (jet_ochre                           * (1360f64 * 1.15 * config.reprocessing_asteroid())) +
            (compressed_dark_ochre               * (1360f64 * 1.00 * config.reprocessing_asteroid())) +
            (compressed_onyx_ochre               * (1360f64 * 1.05 * config.reprocessing_asteroid())) +
            (compressed_obsidian_ochre           * (1360f64 * 1.10 * config.reprocessing_asteroid())) +
            (compressed_jet_ochre                * (1360f64 * 1.15 * config.reprocessing_asteroid())) +

            (gneiss                              * (1500f64 * 1.00 * config.reprocessing_asteroid())) +
            (iridescent_gneiss                   * (1500f64 * 1.05 * config.reprocessing_asteroid())) +
            (prismatic_gneiss                    * (1500f64 * 1.10 * config.reprocessing_asteroid())) +
            (brilliant_gneiss                    * (1500f64 * 1.15 * config.reprocessing_asteroid())) +
            (compressed_gneiss                   * (1500f64 * 1.00 * config.reprocessing_asteroid())) +
            (compressed_iridescent_gneiss        * (1500f64 * 1.05 * config.reprocessing_asteroid())) +
            (compressed_prismatic_gneiss         * (1500f64 * 1.10 * config.reprocessing_asteroid())) +
            (compressed_brilliant_gneiss         * (1500f64 * 1.15 * config.reprocessing_asteroid())) +

            (jaspet                              * (150f64 * 1.00 * config.reprocessing_asteroid())) +
            (pure_jaspet                         * (150f64 * 1.05 * config.reprocessing_asteroid())) +
            (pristine_jaspet                     * (150f64 * 1.10 * config.reprocessing_asteroid())) +
            (immaculate_jaspet                   * (150f64 * 1.15 * config.reprocessing_asteroid())) +
            (compressed_jaspet                   * (150f64 * 1.00 * config.reprocessing_asteroid())) +
            (compressed_pure_jaspet              * (150f64 * 1.05 * config.reprocessing_asteroid())) +
            (compressed_pristine_jaspet          * (150f64 * 1.10 * config.reprocessing_asteroid())) +
            (compressed_immaculate_jaspet        * (150f64 * 1.15 * config.reprocessing_asteroid())) +

            (kernite                             * (60f64 * 1.00 * config.reprocessing_asteroid())) +
            (luminous_kernite                    * (60f64 * 1.05 * config.reprocessing_asteroid())) +
            (fiery_kernite                       * (60f64 * 1.10 * config.reprocessing_asteroid())) +
            (resplendant_kernite                 * (60f64 * 1.15 * config.reprocessing_asteroid())) +
            (compressed_kernite                  * (60f64 * 1.00 * config.reprocessing_asteroid())) +
            (compressed_luminous_kernite         * (60f64 * 1.05 * config.reprocessing_asteroid())) +
            (compressed_fiery_kernite            * (60f64 * 1.10 * config.reprocessing_asteroid())) +
            (compressed_resplendant_kernite      * (60f64 * 1.15 * config.reprocessing_asteroid())) +

            (kylixium                            * (550f64 * 1.00 * config.reprocessing_asteroid())) +
            (kaolin_kylixium                     * (550f64 * 1.05 * config.reprocessing_asteroid())) +
            (argil_kylixium                      * (550f64 * 1.10 * config.reprocessing_asteroid())) +
            (adobe_kylixium                      * (550f64 * 1.15 * config.reprocessing_asteroid())) +
            (compressed_kylixium                 * (550f64 * 1.00 * config.reprocessing_asteroid())) +
            (compressed_kaolin_kylixium          * (550f64 * 1.05 * config.reprocessing_asteroid())) +
            (compressed_argil_kylixium           * (550f64 * 1.10 * config.reprocessing_asteroid())) +
            (compressed_adobe_kylixium           * (550f64 * 1.15 * config.reprocessing_asteroid())) +

            (plagioclase                         * (70f64 * 1.00 * config.reprocessing_asteroid())) +
            (azure_plagioclase                   * (70f64 * 1.05 * config.reprocessing_asteroid())) +
            (rich_plagioclase                    * (70f64 * 1.10 * config.reprocessing_asteroid())) +
            (sparkling_plagioclase               * (70f64 * 1.15 * config.reprocessing_asteroid())) +
            (compressed_plagioclase              * (70f64 * 1.00 * config.reprocessing_asteroid())) +
            (compressed_azure_plagioclase        * (70f64 * 1.05 * config.reprocessing_asteroid())) +
            (compressed_rich_plagioclase         * (70f64 * 1.10 * config.reprocessing_asteroid())) +
            (compressed_sparkling_plagioclase    * (70f64 * 1.15 * config.reprocessing_asteroid())) +

            (pyroxeres                           * (30f64 * 1.00 * config.reprocessing_asteroid())) +
            (solid_pyroxeres                     * (30f64 * 1.05 * config.reprocessing_asteroid())) +
            (viscous_pyroxeres                   * (30f64 * 1.10 * config.reprocessing_asteroid())) +
            (opulent_pyroxeres                   * (30f64 * 1.15 * config.reprocessing_asteroid())) +
            (compressed_pyroxeres                * (30f64 * 1.00 * config.reprocessing_asteroid())) +
            (compressed_solid_pyroxeres          * (30f64 * 1.05 * config.reprocessing_asteroid())) +
            (compressed_viscous_pyroxeres        * (30f64 * 1.10 * config.reprocessing_asteroid())) +
            (compressed_opulent_pyroxeres        * (30f64 * 1.15 * config.reprocessing_asteroid())) +

            (bitumens                            * (400f64 * 1.00 * config.reprocessing_asteroid())) +
            (brimful_bitumens                    * (400f64 * 1.15 * config.reprocessing_asteroid())) +
            (glistening_bitumens                 * (400f64 * 2.00 * config.reprocessing_asteroid())) +
            (compressed_bitumens                 * (400f64 * 1.00 * config.reprocessing_asteroid())) +
            (compressed_brimful_bitumens         * (400f64 * 1.15 * config.reprocessing_asteroid())) +
            (compressed_glistering_bitumens      * (400f64 * 2.00 * config.reprocessing_asteroid())) +

            (coesite                             * (400f64 * 1.00 * config.reprocessing_asteroid())) +
            (brimful_coesite                     * (400f64 * 1.15 * config.reprocessing_asteroid())) +
            (glistening_coesite                  * (400f64 * 2.00 * config.reprocessing_asteroid())) +
            (compressed_coesite                  * (400f64 * 1.00 * config.reprocessing_asteroid())) +
            (compressed_brimful_coesite          * (400f64 * 1.15 * config.reprocessing_asteroid())) +
            (compressed_glistering_coesite       * (400f64 * 2.00 * config.reprocessing_asteroid())) +

            (sylvite                             * (400f64 * 1.00 * config.reprocessing_asteroid())) +
            (brimful_sylvite                     * (400f64 * 1.15 * config.reprocessing_asteroid())) +
            (glistening_sylvite                  * (400f64 * 2.00 * config.reprocessing_asteroid())) +
            (compressed_sylvite                  * (400f64 * 1.00 * config.reprocessing_asteroid())) +
            (compressed_brimful_sylvite          * (400f64 * 1.15 * config.reprocessing_asteroid())) +
            (compressed_glistering_sylvite       * (400f64 * 2.00 * config.reprocessing_asteroid())) +

            (zeolites                            * (400f64 * 1.00 * config.reprocessing_asteroid())) +
            (brimful_zeolites                    * (400f64 * 1.15 * config.reprocessing_asteroid())) +
            (glistening_zeolites                 * (400f64 * 2.00 * config.reprocessing_asteroid())) +
            (compressed_zeolites                 * (400f64 * 1.00 * config.reprocessing_asteroid())) +
            (compressed_brimful_zeolites         * (400f64 * 1.15 * config.reprocessing_asteroid())) +
            (compressed_glistering_zeolites      * (400f64 * 2.00 * config.reprocessing_asteroid())) +

            mexallon >= config.want_mineral(Mineral::Mexallon)
        ).set_name("Mexallon".into()))
        .with(constraint!(
            (bezdnacine                          * (4800f64 * 1.00 * config.reprocessing_asteroid())) +
            (abyssal_bezdnacine                  * (4800f64 * 1.05 * config.reprocessing_asteroid())) +
            (hadal_bezdnacine                    * (4800f64 * 1.10 * config.reprocessing_asteroid())) +
            (compressed_bezdnacine               * (4800f64 * 1.00 * config.reprocessing_asteroid())) +
            (compressed_abyssal_bezdnacine       * (4800f64 * 1.05 * config.reprocessing_asteroid())) +
            (compressed_hadal_bezdnacine         * (4800f64 * 1.10 * config.reprocessing_asteroid())) +

            (dark_ochre                          * (1200f64 * 1.00 * config.reprocessing_asteroid())) +
            (onyx_ochre                          * (1200f64 * 1.05 * config.reprocessing_asteroid())) +
            (obsidian_ochre                      * (1200f64 * 1.10 * config.reprocessing_asteroid())) +
            (jet_ochre                           * (1200f64 * 1.15 * config.reprocessing_asteroid())) +
            (compressed_dark_ochre               * (1200f64 * 1.00 * config.reprocessing_asteroid())) +
            (compressed_onyx_ochre               * (1200f64 * 1.05 * config.reprocessing_asteroid())) +
            (compressed_obsidian_ochre           * (1200f64 * 1.10 * config.reprocessing_asteroid())) +
            (compressed_jet_ochre                * (1200f64 * 1.15 * config.reprocessing_asteroid())) +

            (gneiss                              * (800f64 * 1.00 * config.reprocessing_asteroid())) +
            (iridescent_gneiss                   * (800f64 * 1.05 * config.reprocessing_asteroid())) +
            (prismatic_gneiss                    * (800f64 * 1.10 * config.reprocessing_asteroid())) +
            (brilliant_gneiss                    * (800f64 * 1.15 * config.reprocessing_asteroid())) +
            (compressed_gneiss                   * (800f64 * 1.00 * config.reprocessing_asteroid())) +
            (compressed_iridescent_gneiss        * (800f64 * 1.05 * config.reprocessing_asteroid())) +
            (compressed_prismatic_gneiss         * (800f64 * 1.10 * config.reprocessing_asteroid())) +
            (compressed_brilliant_gneiss         * (800f64 * 1.15 * config.reprocessing_asteroid())) +

            (griemeer                            * (80f64 * 1.00 * config.reprocessing_asteroid())) +
            (clear_griemeer                      * (80f64 * 1.05 * config.reprocessing_asteroid())) +
            (inky_griemeer                       * (80f64 * 1.10 * config.reprocessing_asteroid())) +
            (opaque_griemeer                     * (80f64 * 1.15 * config.reprocessing_asteroid())) +
            (compressed_griemeer                 * (80f64 * 1.00 * config.reprocessing_asteroid())) +
            (compressed_clear_griemeer           * (80f64 * 1.05 * config.reprocessing_asteroid())) +
            (compressed_inky_griemeer            * (80f64 * 1.10 * config.reprocessing_asteroid())) +
            (compressed_opaque_griemeer          * (80f64 * 1.15 * config.reprocessing_asteroid())) +

            (hemorphite                          * (240f64 * 1.00 * config.reprocessing_asteroid())) +
            (vivid_hemorphite                    * (240f64 * 1.05 * config.reprocessing_asteroid())) +
            (radiant_hemorphite                  * (240f64 * 1.10 * config.reprocessing_asteroid())) +
            (scintillating_hemorphite            * (240f64 * 1.15 * config.reprocessing_asteroid())) +
            (compressed_hemorphite               * (240f64 * 1.00 * config.reprocessing_asteroid())) +
            (compressed_vivid_hemorphite         * (240f64 * 1.05 * config.reprocessing_asteroid())) +
            (compressed_radiant_hemorphite       * (240f64 * 1.10 * config.reprocessing_asteroid())) +
            (compressed_scintillating_hemorphite * (240f64 * 1.15 * config.reprocessing_asteroid())) +

            (hezorime                            * (120f64 * 1.00 * config.reprocessing_asteroid())) +
            (dull_hezorime                       * (120f64 * 1.05 * config.reprocessing_asteroid())) +
            (serrated_hezorime                   * (120f64 * 1.10 * config.reprocessing_asteroid())) +
            (sharp_hezorime                      * (120f64 * 1.15 * config.reprocessing_asteroid())) +
            (compressed_hezorime                 * (120f64 * 1.00 * config.reprocessing_asteroid())) +
            (compressed_dull_hezorime            * (120f64 * 1.05 * config.reprocessing_asteroid())) +
            (compressed_serrated_hezorime        * (120f64 * 1.10 * config.reprocessing_asteroid())) +
            (compressed_sharp_hezorime           * (120f64 * 1.15 * config.reprocessing_asteroid())) +

            (kernite                             * (120f64 * 1.00 * config.reprocessing_asteroid())) +
            (luminous_kernite                    * (120f64 * 1.05 * config.reprocessing_asteroid())) +
            (fiery_kernite                       * (120f64 * 1.10 * config.reprocessing_asteroid())) +
            (resplendant_kernite                 * (120f64 * 1.15 * config.reprocessing_asteroid())) +
            (compressed_kernite                  * (120f64 * 1.00 * config.reprocessing_asteroid())) +
            (compressed_luminous_kernite         * (120f64 * 1.05 * config.reprocessing_asteroid())) +
            (compressed_fiery_kernite            * (120f64 * 1.10 * config.reprocessing_asteroid())) +
            (compressed_resplendant_kernite      * (120f64 * 1.15 * config.reprocessing_asteroid())) +

            (omber                               * (75f64 * 1.00 * config.reprocessing_asteroid())) +
            (silvery_omber                       * (75f64 * 1.05 * config.reprocessing_asteroid())) +
            (golden_omber                        * (75f64 * 1.10 * config.reprocessing_asteroid())) +
            (platinoid_omber                     * (75f64 * 1.15 * config.reprocessing_asteroid())) +
            (compressed_omber                    * (75f64 * 1.00 * config.reprocessing_asteroid())) +
            (compressed_silvery_omber            * (75f64 * 1.05 * config.reprocessing_asteroid())) +
            (compressed_golden_omber             * (75f64 * 1.10 * config.reprocessing_asteroid())) +
            (compressed_platinoid_omber          * (75f64 * 1.15 * config.reprocessing_asteroid())) +

            (rakovene                            * (3200f64 * 1.00 * config.reprocessing_asteroid())) +
            (abyssal_rakovene                    * (3200f64 * 1.05 * config.reprocessing_asteroid())) +
            (hadal_rakovene                      * (3200f64 * 1.10 * config.reprocessing_asteroid())) +
            (compressed_rakovene                 * (3200f64 * 1.00 * config.reprocessing_asteroid())) +
            (compressed_abyssal_rakovene         * (3200f64 * 1.05 * config.reprocessing_asteroid())) +
            (compressed_hadal_rakovene           * (3200f64 * 1.10 * config.reprocessing_asteroid())) +

            (spodumain                           * (1000f64 * 1.00 * config.reprocessing_asteroid())) +
            (bright_spodumain                    * (1000f64 * 1.05 * config.reprocessing_asteroid())) +
            (gleaming_spodumain                  * (1000f64 * 1.10 * config.reprocessing_asteroid())) +
            (dazzling_spodumain                  * (1000f64 * 1.15 * config.reprocessing_asteroid())) +
            (compressed_spodumain                * (1000f64 * 1.00 * config.reprocessing_asteroid())) +
            (compressed_bright_spodumain         * (1000f64 * 1.05 * config.reprocessing_asteroid())) +
            (compressed_gleaming_spodumain       * (1000f64 * 1.10 * config.reprocessing_asteroid())) +
            (compressed_dazzling_spodumain       * (1000f64 * 1.15 * config.reprocessing_asteroid())) +

            (ytirium                             * (240f64 * 1.00 * config.reprocessing_asteroid())) +
            (bootleg_ytirium                     * (240f64 * 1.05 * config.reprocessing_asteroid())) +
            (firewater_ytirium                   * (240f64 * 1.10 * config.reprocessing_asteroid())) +
            (moonshine_ytirium                   * (240f64 * 1.15 * config.reprocessing_asteroid())) +
            (compressed_ytirium                  * (240f64 * 1.00 * config.reprocessing_asteroid())) +
            (compressed_bootleg_ytirium          * (240f64 * 1.05 * config.reprocessing_asteroid())) +
            (compressed_firewater_ytirium        * (240f64 * 1.10 * config.reprocessing_asteroid())) +
            (compressed_moonshine_ytirium        * (240f64 * 1.15 * config.reprocessing_asteroid())) +

            isogen >= config.want_mineral(Mineral::Isogen)
        ).set_name("Isogen".into()))
        .with(constraint!(
            (crokite                             * (800f64 * 1.00 * config.reprocessing_asteroid())) +
            (sharp_crokite                       * (800f64 * 1.05 * config.reprocessing_asteroid())) +
            (crystalline_crokite                 * (800f64 * 1.10 * config.reprocessing_asteroid())) +
            (pellucid_crokite                    * (800f64 * 1.15 * config.reprocessing_asteroid())) +
            (compressed_crokite                  * (800f64 * 1.00 * config.reprocessing_asteroid())) +
            (compressed_sharp_crokite            * (800f64 * 1.05 * config.reprocessing_asteroid())) +
            (compressed_crystalline_crokite      * (800f64 * 1.10 * config.reprocessing_asteroid())) +
            (compressed_pellucid_crokite         * (800f64 * 1.15 * config.reprocessing_asteroid())) +

            (dark_ochre                          * (320f64 * 1.00 * config.reprocessing_asteroid())) +
            (onyx_ochre                          * (320f64 * 1.05 * config.reprocessing_asteroid())) +
            (obsidian_ochre                      * (320f64 * 1.10 * config.reprocessing_asteroid())) +
            (jet_ochre                           * (320f64 * 1.15 * config.reprocessing_asteroid())) +
            (compressed_dark_ochre               * (320f64 * 1.00 * config.reprocessing_asteroid())) +
            (compressed_onyx_ochre               * (320f64 * 1.05 * config.reprocessing_asteroid())) +
            (compressed_obsidian_ochre           * (320f64 * 1.10 * config.reprocessing_asteroid())) +
            (compressed_jet_ochre                * (320f64 * 1.15 * config.reprocessing_asteroid())) +

            (hedbergite                          * (120f64 * 1.00 * config.reprocessing_asteroid())) +
            (vitric_hedbergite                   * (120f64 * 1.05 * config.reprocessing_asteroid())) +
            (glazed_hedbergite                   * (120f64 * 1.10 * config.reprocessing_asteroid())) +
            (lustrous_hedbergite                 * (120f64 * 1.15 * config.reprocessing_asteroid())) +
            (compressed_hedbergite               * (120f64 * 1.00 * config.reprocessing_asteroid())) +
            (compressed_vitric_hedbergite        * (120f64 * 1.05 * config.reprocessing_asteroid())) +
            (compressed_glazed_hedbergite        * (120f64 * 1.10 * config.reprocessing_asteroid())) +
            (compressed_lustrous_hedbergite      * (120f64 * 1.15 * config.reprocessing_asteroid())) +

            (hemorphite                          * (90f64 * 1.00 * config.reprocessing_asteroid())) +
            (vivid_hemorphite                    * (90f64 * 1.05 * config.reprocessing_asteroid())) +
            (radiant_hemorphite                  * (90f64 * 1.10 * config.reprocessing_asteroid())) +
            (scintillating_hemorphite            * (90f64 * 1.15 * config.reprocessing_asteroid())) +
            (compressed_hemorphite               * (90f64 * 1.00 * config.reprocessing_asteroid())) +
            (compressed_vivid_hemorphite         * (90f64 * 1.05 * config.reprocessing_asteroid())) +
            (compressed_radiant_hemorphite       * (90f64 * 1.10 * config.reprocessing_asteroid())) +
            (compressed_scintillating_hemorphite * (90f64 * 1.15 * config.reprocessing_asteroid())) +

            (jaspet                              * (50f64 * 1.00 * config.reprocessing_asteroid())) +
            (pure_jaspet                         * (50f64 * 1.05 * config.reprocessing_asteroid())) +
            (pristine_jaspet                     * (50f64 * 1.10 * config.reprocessing_asteroid())) +
            (immaculate_jaspet                   * (50f64 * 1.15 * config.reprocessing_asteroid())) +
            (compressed_jaspet                   * (50f64 * 1.00 * config.reprocessing_asteroid())) +
            (compressed_pure_jaspet              * (50f64 * 1.05 * config.reprocessing_asteroid())) +
            (compressed_pristine_jaspet          * (50f64 * 1.10 * config.reprocessing_asteroid())) +
            (compressed_immaculate_jaspet        * (50f64 * 1.15 * config.reprocessing_asteroid())) +

            (nocxite                             * (105f64 * 1.00 * config.reprocessing_asteroid())) +
            (fragrant_nocxite                    * (105f64 * 1.05 * config.reprocessing_asteroid())) +
            (intoxicating_nocxite                * (105f64 * 1.10 * config.reprocessing_asteroid())) +
            (ambrosial_nocxite                   * (105f64 * 1.15 * config.reprocessing_asteroid())) +
            (compressed_nocxite                  * (105f64 * 1.00 * config.reprocessing_asteroid())) +
            (compressed_fragrant_nocxite         * (105f64 * 1.05 * config.reprocessing_asteroid())) +
            (compressed_intoxicating_nocxite     * (105f64 * 1.10 * config.reprocessing_asteroid())) +
            (compressed_ambrosial_nocxite        * (105f64 * 1.15 * config.reprocessing_asteroid())) +

            (spodumain                           * (160f64 * 1.00 * config.reprocessing_asteroid())) +
            (bright_spodumain                    * (160f64 * 1.05 * config.reprocessing_asteroid())) +
            (gleaming_spodumain                  * (160f64 * 1.10 * config.reprocessing_asteroid())) +
            (dazzling_spodumain                  * (160f64 * 1.15 * config.reprocessing_asteroid())) +
            (compressed_spodumain                * (160f64 * 1.00 * config.reprocessing_asteroid())) +
            (compressed_bright_spodumain         * (160f64 * 1.05 * config.reprocessing_asteroid())) +
            (compressed_gleaming_spodumain       * (160f64 * 1.10 * config.reprocessing_asteroid())) +
            (compressed_dazzling_spodumain       * (160f64 * 1.15 * config.reprocessing_asteroid())) +

            (talassonite                         * (960f64 * 1.00 * config.reprocessing_asteroid())) +
            (abyssal_talassonite                 * (960f64 * 1.05 * config.reprocessing_asteroid())) +
            (hadal_talassonite                   * (960f64 * 1.10 * config.reprocessing_asteroid())) +
            (compressed_talassonite              * (960f64 * 1.00 * config.reprocessing_asteroid())) +
            (compressed_abyssal_talassonite      * (960f64 * 1.05 * config.reprocessing_asteroid())) +
            (compressed_hadal_talassonite        * (960f64 * 1.10 * config.reprocessing_asteroid())) +

            nocxium >= config.want_mineral(Mineral::Nocxium)
        ).set_name("Nocxium".into()))
        .with(constraint!(
            (bistot                              * (160f64 * 1.00 * config.reprocessing_asteroid())) +
            (triclinic_bistot                    * (160f64 * 1.05 * config.reprocessing_asteroid())) +
            (monoclinic_bistot                   * (160f64 * 1.10 * config.reprocessing_asteroid())) +
            (cubic_bistot                        * (160f64 * 1.15 * config.reprocessing_asteroid())) +
            (compressed_bistot                   * (160f64 * 1.00 * config.reprocessing_asteroid())) +
            (compressed_triclinic_bistot         * (160f64 * 1.05 * config.reprocessing_asteroid())) +
            (compressed_monoclinic_bistot        * (160f64 * 1.10 * config.reprocessing_asteroid())) +
            (compressed_cubic_bistot             * (160f64 * 1.15 * config.reprocessing_asteroid())) +

            (eifyrium                            * (266f64 * 1.00 * config.reprocessing_asteroid())) +
            (doped_eifyrium                      * (266f64 * 1.05 * config.reprocessing_asteroid())) +
            (boosted_eifyrium                    * (266f64 * 1.10 * config.reprocessing_asteroid())) +
            (augmented_eifyrium                  * (266f64 * 1.15 * config.reprocessing_asteroid())) +
            (compressed_eifyrium                 * (266f64 * 1.00 * config.reprocessing_asteroid())) +
            (compressed_doped_eifyrium           * (266f64 * 1.05 * config.reprocessing_asteroid())) +
            (compressed_boosted_eifyrium         * (266f64 * 1.10 * config.reprocessing_asteroid())) +
            (compressed_augmented_eifyrium       * (266f64 * 1.15 * config.reprocessing_asteroid())) +

            (hezorime                            * (60f64 * 1.00 * config.reprocessing_asteroid())) +
            (dull_hezorime                       * (60f64 * 1.05 * config.reprocessing_asteroid())) +
            (serrated_hezorime                   * (60f64 * 1.10 * config.reprocessing_asteroid())) +
            (sharp_hezorime                      * (60f64 * 1.15 * config.reprocessing_asteroid())) +
            (compressed_hezorime                 * (60f64 * 1.00 * config.reprocessing_asteroid())) +
            (compressed_dull_hezorime            * (60f64 * 1.05 * config.reprocessing_asteroid())) +
            (compressed_serrated_hezorime        * (60f64 * 1.10 * config.reprocessing_asteroid())) +
            (compressed_sharp_hezorime           * (60f64 * 1.15 * config.reprocessing_asteroid())) +

            (rakovene                            * (200f64 * 1.00 * config.reprocessing_asteroid())) +
            (abyssal_rakovene                    * (200f64 * 1.05 * config.reprocessing_asteroid())) +
            (hadal_rakovene                      * (200f64 * 1.10 * config.reprocessing_asteroid())) +
            (compressed_rakovene                 * (200f64 * 1.00 * config.reprocessing_asteroid())) +
            (compressed_abyssal_rakovene         * (200f64 * 1.05 * config.reprocessing_asteroid())) +
            (compressed_hadal_rakovene           * (200f64 * 1.10 * config.reprocessing_asteroid())) +

            (spodumain                           * (80f64 * 1.00 * config.reprocessing_asteroid())) +
            (bright_spodumain                    * (80f64 * 1.05 * config.reprocessing_asteroid())) +
            (gleaming_spodumain                  * (80f64 * 1.10 * config.reprocessing_asteroid())) +
            (dazzling_spodumain                  * (80f64 * 1.15 * config.reprocessing_asteroid())) +
            (compressed_spodumain                * (80f64 * 1.00 * config.reprocessing_asteroid())) +
            (compressed_bright_spodumain         * (80f64 * 1.05 * config.reprocessing_asteroid())) +
            (compressed_gleaming_spodumain       * (80f64 * 1.10 * config.reprocessing_asteroid())) +
            (compressed_dazzling_spodumain       * (80f64 * 1.15 * config.reprocessing_asteroid())) +

            zydrine >= config.want_mineral(Mineral::Zydrine)
        ).set_name("Zydrine".into()))
        .with(constraint!(
            (arkonor                             * (120f64 * 1.00 * config.reprocessing_asteroid())) +
            (crimson_arkonor                     * (120f64 * 1.05 * config.reprocessing_asteroid())) +
            (prime_arkonor                       * (120f64 * 1.10 * config.reprocessing_asteroid())) +
            (flawless_arkonor                    * (120f64 * 1.15 * config.reprocessing_asteroid())) +
            (compressed_arkonor                  * (120f64 * 1.00 * config.reprocessing_asteroid())) +
            (compressed_crimson_arkonor          * (120f64 * 1.05 * config.reprocessing_asteroid())) +
            (compressed_prime_arkonor            * (120f64 * 1.10 * config.reprocessing_asteroid())) +
            (compressed_flawless_arkonor         * (120f64 * 1.15 * config.reprocessing_asteroid())) +

            (bezdnacine                          * (128f64 * 1.00 * config.reprocessing_asteroid())) +
            (abyssal_bezdnacine                  * (128f64 * 1.05 * config.reprocessing_asteroid())) +
            (hadal_bezdnacine                    * (128f64 * 1.10 * config.reprocessing_asteroid())) +
            (compressed_bezdnacine               * (128f64 * 1.00 * config.reprocessing_asteroid())) +
            (compressed_abyssal_bezdnacine       * (128f64 * 1.05 * config.reprocessing_asteroid())) +
            (compressed_hadal_bezdnacine         * (128f64 * 1.10 * config.reprocessing_asteroid())) +

            (ducinium                            * (170f64 * 1.00 * config.reprocessing_asteroid())) +
            (noble_ducinium                      * (170f64 * 1.05 * config.reprocessing_asteroid())) +
            (royal_ducinium                      * (170f64 * 1.10 * config.reprocessing_asteroid())) +
            (imperial_ducinium                   * (170f64 * 1.15 * config.reprocessing_asteroid())) +
            (compressed_ducinium                 * (170f64 * 1.00 * config.reprocessing_asteroid())) +
            (compressed_noble_ducinium           * (170f64 * 1.05 * config.reprocessing_asteroid())) +
            (compressed_royal_ducinium           * (170f64 * 1.10 * config.reprocessing_asteroid())) +
            (compressed_imperial_ducinium        * (170f64 * 1.15 * config.reprocessing_asteroid())) +

            (spodumain                           * (40f64 * 1.00 * config.reprocessing_asteroid())) +
            (bright_spodumain                    * (40f64 * 1.05 * config.reprocessing_asteroid())) +
            (gleaming_spodumain                  * (40f64 * 1.10 * config.reprocessing_asteroid())) +
            (dazzling_spodumain                  * (40f64 * 1.15 * config.reprocessing_asteroid())) +
            (compressed_spodumain                * (40f64 * 1.00 * config.reprocessing_asteroid())) +
            (compressed_bright_spodumain         * (40f64 * 1.05 * config.reprocessing_asteroid())) +
            (compressed_gleaming_spodumain       * (40f64 * 1.10 * config.reprocessing_asteroid())) +
            (compressed_dazzling_spodumain       * (40f64 * 1.15 * config.reprocessing_asteroid())) +

            (talassonite                         * (32f64 * 1.00 * config.reprocessing_asteroid())) +
            (abyssal_talassonite                 * (32f64 * 1.05 * config.reprocessing_asteroid())) +
            (hadal_talassonite                   * (32f64 * 1.10 * config.reprocessing_asteroid())) +
            (compressed_talassonite              * (32f64 * 1.00 * config.reprocessing_asteroid())) +
            (compressed_abyssal_talassonite      * (32f64 * 1.05 * config.reprocessing_asteroid())) +
            (compressed_hadal_talassonite        * (32f64 * 1.10 * config.reprocessing_asteroid())) +

            (ueganite                            * (40f64 * 1.00 * config.reprocessing_asteroid())) +
            (foggy_ueganite                      * (40f64 * 1.05 * config.reprocessing_asteroid())) +
            (overcast_ueganite                   * (40f64 * 1.10 * config.reprocessing_asteroid())) +
            (stormy_ueganite                     * (40f64 * 1.15 * config.reprocessing_asteroid())) +
            (compressed_ueganite                 * (40f64 * 1.00 * config.reprocessing_asteroid())) +
            (compressed_foggy_ueganite           * (40f64 * 1.05 * config.reprocessing_asteroid())) +
            (compressed_overcast_ueganite        * (40f64 * 1.10 * config.reprocessing_asteroid())) +
            (compressed_stormy_ueganite          * (40f64 * 1.15 * config.reprocessing_asteroid())) +

            megacyte >= config.want_mineral(Mineral::Megacyte)
        ).set_name("Megacyte".into()))
        .with(constraint!(
            (mercoxit                            * (140f64 * 1.00 * config.reprocessing_asteroid())) +
            (magma_mercoxit                      * (140f64 * 1.05 * config.reprocessing_asteroid())) +
            (vitreous_mercoxit                   * (140f64 * 1.10 * config.reprocessing_asteroid())) +
            (compressed_mercoxit                 * (140f64 * 1.00 * config.reprocessing_asteroid())) +
            (compressed_magma_mercoxit           * (140f64 * 1.05 * config.reprocessing_asteroid())) +
            (compressed_vitreous_mercoxit        * (140f64 * 1.10 * config.reprocessing_asteroid())) +

            morphite >= config.want_mineral(Mineral::Morphite)
        ).set_name("Morphite".into()))
        .with(constraint!(
            (zeolites                            * (65f64 * 1.00 * config.reprocessing_asteroid())) +
            (brimful_zeolites                    * (65f64 * 1.15 * config.reprocessing_asteroid())) +
            (glistening_zeolites                 * (65f64 * 2.00 * config.reprocessing_asteroid())) +
            (compressed_zeolites                 * (65f64 * 1.00 * config.reprocessing_asteroid())) +
            (compressed_brimful_zeolites         * (65f64 * 1.15 * config.reprocessing_asteroid())) +
            (compressed_glistering_zeolites      * (65f64 * 2.00 * config.reprocessing_asteroid())) +

            (otavite                             * (10f64 * 1.00 * config.reprocessing_asteroid())) +
            (lavish_otavite                      * (10f64 * 1.15 * config.reprocessing_asteroid())) +
            (shimmering_otavite                  * (10f64 * 2.00 * config.reprocessing_asteroid())) +
            (compressed_otavite                  * (10f64 * 1.00 * config.reprocessing_asteroid())) +
            (compressed_lavish_otavite           * (10f64 * 1.15 * config.reprocessing_asteroid())) +
            (compressed_shimmering_otavite       * (10f64 * 2.00 * config.reprocessing_asteroid())) +

            (carnotite                           * (15f64 * 1.00 * config.reprocessing_asteroid())) +
            (replete_carnotite                   * (15f64 * 1.15 * config.reprocessing_asteroid())) +
            (glowing_carnotite                   * (15f64 * 2.00 * config.reprocessing_asteroid())) +
            (compressed_carnotite                * (15f64 * 1.00 * config.reprocessing_asteroid())) +
            (compressed_replete_carnotite        * (15f64 * 1.15 * config.reprocessing_asteroid())) +
            (compressed_glowing_carnotite        * (15f64 * 2.00 * config.reprocessing_asteroid())) +

            (xenotime                            * (20f64 * 1.00 * config.reprocessing_asteroid())) +
            (bountiful_xenotime                  * (20f64 * 1.15 * config.reprocessing_asteroid())) +
            (shining_xenotime                    * (20f64 * 2.00 * config.reprocessing_asteroid())) +
            (compressed_xenotime                 * (20f64 * 1.00 * config.reprocessing_asteroid())) +
            (compressed_bountiful_xenotime       * (20f64 * 1.15 * config.reprocessing_asteroid())) +
            (compressed_shining_xenotime         * (20f64 * 2.00 * config.reprocessing_asteroid())) +

            atmospheric_gases >= config.want_mineral(Mineral::AtmosphericGases)
        ).set_name("Atmospheric Gases".into()))
        .with(constraint!(
            (sylvite                             * (65f64 * 1.00 * config.reprocessing_asteroid())) +
            (brimful_sylvite                     * (65f64 * 1.15 * config.reprocessing_asteroid())) +
            (glistening_sylvite                  * (65f64 * 2.00 * config.reprocessing_asteroid())) +
            (compressed_sylvite                  * (65f64 * 1.00 * config.reprocessing_asteroid())) +
            (compressed_brimful_sylvite          * (65f64 * 1.15 * config.reprocessing_asteroid())) +
            (compressed_glistering_sylvite       * (65f64 * 2.00 * config.reprocessing_asteroid())) +

            (sperrylite                          * (10f64 * 1.00 * config.reprocessing_asteroid())) +
            (lavish_sperrylite                   * (10f64 * 1.15 * config.reprocessing_asteroid())) +
            (shimmering_sperrylite               * (10f64 * 2.00 * config.reprocessing_asteroid())) +
            (compressed_sperrylite               * (10f64 * 1.00 * config.reprocessing_asteroid())) +
            (compressed_lavish_sperrylite        * (10f64 * 1.15 * config.reprocessing_asteroid())) +
            (compressed_shimmering_sperrylite    * (10f64 * 2.00 * config.reprocessing_asteroid())) +

            (cinnabar                            * (15f64 * 1.00 * config.reprocessing_asteroid())) +
            (replete_cinnabar                    * (15f64 * 1.15 * config.reprocessing_asteroid())) +
            (glowing_cinnabar                    * (15f64 * 2.00 * config.reprocessing_asteroid())) +
            (compressed_cinnabar                 * (15f64 * 1.00 * config.reprocessing_asteroid())) +
            (compressed_replete_cinnabar         * (15f64 * 1.15 * config.reprocessing_asteroid())) +
            (compressed_glowing_cinnabar         * (15f64 * 2.00 * config.reprocessing_asteroid())) +

            (monazite                            * (20f64 * 1.00 * config.reprocessing_asteroid())) +
            (bountiful_monazite                  * (20f64 * 1.15 * config.reprocessing_asteroid())) +
            (shining_monazite                    * (20f64 * 2.00 * config.reprocessing_asteroid())) +
            (compressed_monazite                 * (20f64 * 1.00 * config.reprocessing_asteroid())) +
            (compressed_bountiful_monazite       * (20f64 * 1.15 * config.reprocessing_asteroid())) +
            (compressed_shining_monazite         * (20f64 * 2.00 * config.reprocessing_asteroid())) +

            evaporite_deposits >= config.want_mineral(Mineral::EvaporiteDeposits)
        ).set_name("Evaporite Deposits".into()))
        .with(constraint!(
            (bitumens                            * (65f64 * 1.00 * config.reprocessing_asteroid())) +
            (brimful_bitumens                    * (65f64 * 1.15 * config.reprocessing_asteroid())) +
            (glistening_bitumens                 * (65f64 * 2.00 * config.reprocessing_asteroid())) +
            (compressed_bitumens                 * (65f64 * 1.00 * config.reprocessing_asteroid())) +
            (compressed_brimful_bitumens         * (65f64 * 1.15 * config.reprocessing_asteroid())) +
            (compressed_glistering_bitumens      * (65f64 * 2.00 * config.reprocessing_asteroid())) +

            (chromite                            * (10f64 * 1.00 * config.reprocessing_asteroid())) +
            (lavish_chromite                     * (10f64 * 1.15 * config.reprocessing_asteroid())) +
            (shimmering_chromite                 * (10f64 * 2.00 * config.reprocessing_asteroid())) +
            (compressed_chromite                 * (10f64 * 1.00 * config.reprocessing_asteroid())) +
            (compressed_lavish_chromite          * (10f64 * 1.15 * config.reprocessing_asteroid())) +
            (compressed_shimmering_chromite      * (10f64 * 2.00 * config.reprocessing_asteroid())) +

            (pollucite                           * (15f64 * 1.00 * config.reprocessing_asteroid())) +
            (replete_pollucite                   * (15f64 * 1.15 * config.reprocessing_asteroid())) +
            (glowing_pollucite                   * (15f64 * 2.00 * config.reprocessing_asteroid())) +
            (compressed_pollucite                * (15f64 * 1.00 * config.reprocessing_asteroid())) +
            (compressed_replete_pollucite        * (15f64 * 1.15 * config.reprocessing_asteroid())) +
            (compressed_glowing_pollucite        * (15f64 * 2.00 * config.reprocessing_asteroid())) +

            (loparite                            * (20f64 * 1.00 * config.reprocessing_asteroid())) +
            (bountiful_loparite                  * (20f64 * 1.15 * config.reprocessing_asteroid())) +
            (shining_loparite                    * (20f64 * 2.00 * config.reprocessing_asteroid())) +
            (compressed_loparite                 * (20f64 * 1.00 * config.reprocessing_asteroid())) +
            (compressed_bountiful_loparite       * (20f64 * 1.15 * config.reprocessing_asteroid())) +
            (compressed_shining_loparite         * (20f64 * 2.00 * config.reprocessing_asteroid())) +

            hydrocarbons >= config.want_mineral(Mineral::Hydrocarbons)
        ).set_name("Hydrocarbons".into()))
        .with(constraint!(
            (coesite                             * (65f64 * 1.00 * config.reprocessing_asteroid())) +
            (brimful_coesite                     * (65f64 * 1.15 * config.reprocessing_asteroid())) +
            (glistening_coesite                  * (65f64 * 2.00 * config.reprocessing_asteroid())) +
            (compressed_coesite                  * (65f64 * 1.00 * config.reprocessing_asteroid())) +
            (compressed_brimful_coesite          * (65f64 * 1.15 * config.reprocessing_asteroid())) +
            (compressed_glistering_coesite       * (65f64 * 2.00 * config.reprocessing_asteroid())) +

            (vanadinite                          * (10f64 * 1.00 * config.reprocessing_asteroid())) +
            (lavish_vanadinite                   * (10f64 * 1.15 * config.reprocessing_asteroid())) +
            (shimmering_vanadinite               * (10f64 * 2.00 * config.reprocessing_asteroid())) +
            (compressed_vanadinite               * (10f64 * 1.00 * config.reprocessing_asteroid())) +
            (compressed_lavish_vanadinite        * (10f64 * 1.15 * config.reprocessing_asteroid())) +
            (compressed_shimmering_vanadinite    * (10f64 * 2.00 * config.reprocessing_asteroid())) +

            (zircon                              * (15f64 * 1.00 * config.reprocessing_asteroid())) +
            (replete_zircon                      * (15f64 * 1.15 * config.reprocessing_asteroid())) +
            (glowing_zircon                      * (15f64 * 2.00 * config.reprocessing_asteroid())) +
            (compressed_zircon                   * (15f64 * 1.00 * config.reprocessing_asteroid())) +
            (compressed_replete_zircon           * (15f64 * 1.15 * config.reprocessing_asteroid())) +
            (compressed_glowing_zircon           * (15f64 * 2.00 * config.reprocessing_asteroid())) +

            (ytterbite                           * (20f64 * 1.00 * config.reprocessing_asteroid())) +
            (bountiful_ytterbite                 * (20f64 * 1.15 * config.reprocessing_asteroid())) +
            (shining_ytterbite                   * (20f64 * 2.00 * config.reprocessing_asteroid())) +
            (compressed_ytterbite                * (20f64 * 1.00 * config.reprocessing_asteroid())) +
            (compressed_bountiful_ytterbite      * (20f64 * 1.15 * config.reprocessing_asteroid())) +
            (compressed_shining_ytterbite        * (20f64 * 2.00 * config.reprocessing_asteroid())) +

            silicates >= config.want_mineral(Mineral::Silicates)
        ).set_name("Silicates".into()))
        .with(constraint!(
            (cobaltite                           * (40f64 * 1.00 * config.reprocessing_asteroid())) +
            (copious_cobaltite                   * (40f64 * 1.15 * config.reprocessing_asteroid())) +
            (twinkling_cobaltite                 * (40f64 * 2.00 * config.reprocessing_asteroid())) +
            (compressed_cobaltite                * (40f64 * 1.00 * config.reprocessing_asteroid())) +
            (compressed_copious_cobaltite        * (40f64 * 1.15 * config.reprocessing_asteroid())) +
            (compressed_twinkling_cobaltite      * (40f64 * 2.00 * config.reprocessing_asteroid())) +

            (carnotite                           * (10f64 * 1.00 * config.reprocessing_asteroid())) +
            (replete_carnotite                   * (10f64 * 1.15 * config.reprocessing_asteroid())) +
            (glowing_carnotite                   * (10f64 * 2.00 * config.reprocessing_asteroid())) +
            (compressed_carnotite                * (10f64 * 1.00 * config.reprocessing_asteroid())) +
            (compressed_replete_carnotite        * (10f64 * 1.15 * config.reprocessing_asteroid())) +
            (compressed_glowing_carnotite        * (10f64 * 2.00 * config.reprocessing_asteroid())) +

            (xenotime                            * (20f64 * 1.00 * config.reprocessing_asteroid())) +
            (bountiful_xenotime                  * (20f64 * 1.15 * config.reprocessing_asteroid())) +
            (shining_xenotime                    * (20f64 * 2.00 * config.reprocessing_asteroid())) +
            (compressed_xenotime                 * (20f64 * 1.00 * config.reprocessing_asteroid())) +
            (compressed_bountiful_xenotime       * (20f64 * 1.15 * config.reprocessing_asteroid())) +
            (compressed_shining_xenotime         * (20f64 * 2.00 * config.reprocessing_asteroid())) +

            cobalt >= config.want_mineral(Mineral::Cobalt)
        ).set_name("Cobalt".into()))
        .with(constraint!(
            (euxenite                            * (40f64 * 1.00 * config.reprocessing_asteroid())) +
            (copious_euxenite                    * (40f64 * 1.15 * config.reprocessing_asteroid())) +
            (twinkling_euxenite                  * (40f64 * 2.00 * config.reprocessing_asteroid())) +
            (compressed_euxenite                 * (40f64 * 1.00 * config.reprocessing_asteroid())) +
            (compressed_copious_euxenite         * (40f64 * 1.15 * config.reprocessing_asteroid())) +
            (compressed_twinkling_euxenite       * (40f64 * 2.00 * config.reprocessing_asteroid())) +

            (pollucite                           * (10f64 * 1.00 * config.reprocessing_asteroid())) +
            (replete_pollucite                   * (10f64 * 1.15 * config.reprocessing_asteroid())) +
            (glowing_pollucite                   * (10f64 * 2.00 * config.reprocessing_asteroid())) +
            (compressed_pollucite                * (10f64 * 1.00 * config.reprocessing_asteroid())) +
            (compressed_replete_pollucite        * (10f64 * 1.15 * config.reprocessing_asteroid())) +
            (compressed_glowing_pollucite        * (10f64 * 2.00 * config.reprocessing_asteroid())) +

            (loparite                            * (20f64 * 1.00 * config.reprocessing_asteroid())) +
            (bountiful_loparite                  * (20f64 * 1.15 * config.reprocessing_asteroid())) +
            (shining_loparite                    * (20f64 * 2.00 * config.reprocessing_asteroid())) +
            (compressed_loparite                 * (20f64 * 1.00 * config.reprocessing_asteroid())) +
            (compressed_bountiful_loparite       * (20f64 * 1.15 * config.reprocessing_asteroid())) +
            (compressed_shining_loparite         * (20f64 * 2.00 * config.reprocessing_asteroid())) +

            scandium >= config.want_mineral(Mineral::Scandium)
        ).set_name("Scandium".into()))
        .with(constraint!(
            (titanite                            * (40f64 * 1.00 * config.reprocessing_asteroid())) +
            (copious_titanite                    * (40f64 * 1.15 * config.reprocessing_asteroid())) +
            (twinkling_titanite                  * (40f64 * 2.00 * config.reprocessing_asteroid())) +
            (compressed_titanite                 * (40f64 * 1.00 * config.reprocessing_asteroid())) +
            (compressed_copious_titanite         * (40f64 * 1.15 * config.reprocessing_asteroid())) +
            (compressed_twinkling_titanite       * (40f64 * 2.00 * config.reprocessing_asteroid())) +

            (zircon                              * (10f64 * 1.00 * config.reprocessing_asteroid())) +
            (replete_zircon                      * (10f64 * 1.15 * config.reprocessing_asteroid())) +
            (glowing_zircon                      * (10f64 * 2.00 * config.reprocessing_asteroid())) +
            (compressed_zircon                   * (10f64 * 1.00 * config.reprocessing_asteroid())) +
            (compressed_replete_zircon           * (10f64 * 1.15 * config.reprocessing_asteroid())) +
            (compressed_glowing_zircon           * (10f64 * 2.00 * config.reprocessing_asteroid())) +

            (ytterbite                           * (20f64 * 1.00 * config.reprocessing_asteroid())) +
            (bountiful_ytterbite                 * (20f64 * 1.15 * config.reprocessing_asteroid())) +
            (shining_ytterbite                   * (20f64 * 2.00 * config.reprocessing_asteroid())) +
            (compressed_ytterbite                * (20f64 * 1.00 * config.reprocessing_asteroid())) +
            (compressed_bountiful_ytterbite      * (20f64 * 1.15 * config.reprocessing_asteroid())) +
            (compressed_shining_ytterbite        * (20f64 * 2.00 * config.reprocessing_asteroid())) +

            titanium >= config.want_mineral(Mineral::Titanium)
        ).set_name("Titanium".into()))
        .with(constraint!(
            (scheelite                           * (40f64 * 1.00 * config.reprocessing_asteroid())) +
            (copious_scheelite                   * (40f64 * 1.15 * config.reprocessing_asteroid())) +
            (twinkling_scheelite                 * (40f64 * 2.00 * config.reprocessing_asteroid())) +
            (compressed_scheelite                * (40f64 * 1.00 * config.reprocessing_asteroid())) +
            (compressed_copious_scheelite        * (40f64 * 1.15 * config.reprocessing_asteroid())) +
            (compressed_twinkling_scheelite      * (40f64 * 2.00 * config.reprocessing_asteroid())) +

            (cinnabar                            * (10f64 * 1.00 * config.reprocessing_asteroid())) +
            (replete_cinnabar                    * (10f64 * 1.15 * config.reprocessing_asteroid())) +
            (glowing_cinnabar                    * (10f64 * 2.00 * config.reprocessing_asteroid())) +
            (compressed_cinnabar                 * (10f64 * 1.00 * config.reprocessing_asteroid())) +
            (compressed_replete_cinnabar         * (10f64 * 1.15 * config.reprocessing_asteroid())) +
            (compressed_glowing_cinnabar         * (10f64 * 2.00 * config.reprocessing_asteroid())) +

            (monazite                            * (20f64 * 1.00 * config.reprocessing_asteroid())) +
            (bountiful_monazite                  * (20f64 * 1.15 * config.reprocessing_asteroid())) +
            (shining_monazite                    * (20f64 * 2.00 * config.reprocessing_asteroid())) +
            (compressed_monazite                 * (20f64 * 1.00 * config.reprocessing_asteroid())) +
            (compressed_bountiful_monazite       * (20f64 * 1.15 * config.reprocessing_asteroid())) +
            (compressed_shining_monazite         * (20f64 * 2.00 * config.reprocessing_asteroid())) +

            tungsten >= config.want_mineral(Mineral::Tungsten)
        ).set_name("Tungsten".into()))
        .with(constraint!(
            (chromite                            * (40f64 * 1.00 * config.reprocessing_asteroid())) +
            (lavish_chromite                     * (40f64 * 1.15 * config.reprocessing_asteroid())) +
            (shimmering_chromite                 * (40f64 * 2.00 * config.reprocessing_asteroid())) +
            (compressed_chromite                 * (40f64 * 1.00 * config.reprocessing_asteroid())) +
            (compressed_lavish_chromite          * (40f64 * 1.15 * config.reprocessing_asteroid())) +
            (compressed_shimmering_chromite      * (40f64 * 2.00 * config.reprocessing_asteroid())) +

            (monazite                            * (10f64 * 1.00 * config.reprocessing_asteroid())) +
            (bountiful_monazite                  * (10f64 * 1.15 * config.reprocessing_asteroid())) +
            (shining_monazite                    * (10f64 * 2.00 * config.reprocessing_asteroid())) +
            (compressed_monazite                 * (10f64 * 1.00 * config.reprocessing_asteroid())) +
            (compressed_bountiful_monazite       * (10f64 * 1.15 * config.reprocessing_asteroid())) +
            (compressed_shining_monazite         * (10f64 * 2.00 * config.reprocessing_asteroid())) +

            chromium >= config.want_mineral(Mineral::Chromium)
        ).set_name("Chromium".into()))
        .with(constraint!(
            (otavite                             * (40f64 * 1.00 * config.reprocessing_asteroid())) +
            (lavish_otavite                      * (40f64 * 1.15 * config.reprocessing_asteroid())) +
            (shimmering_otavite                  * (40f64 * 2.00 * config.reprocessing_asteroid())) +
            (compressed_otavite                  * (40f64 * 1.00 * config.reprocessing_asteroid())) +
            (compressed_lavish_otavite           * (40f64 * 1.15 * config.reprocessing_asteroid())) +
            (compressed_shimmering_otavite       * (40f64 * 2.00 * config.reprocessing_asteroid())) +

            (ytterbite                           * (10f64 * 1.00 * config.reprocessing_asteroid())) +
            (bountiful_ytterbite                 * (10f64 * 1.15 * config.reprocessing_asteroid())) +
            (shining_ytterbite                   * (10f64 * 2.00 * config.reprocessing_asteroid())) +
            (compressed_ytterbite                * (10f64 * 1.00 * config.reprocessing_asteroid())) +
            (compressed_bountiful_ytterbite      * (10f64 * 1.15 * config.reprocessing_asteroid())) +
            (compressed_shining_ytterbite        * (10f64 * 2.00 * config.reprocessing_asteroid())) +

            cadmium >= config.want_mineral(Mineral::Cadmium)
        ).set_name("Cadmium".into()))
        .with(constraint!(
            (sperrylite                          * (40f64 * 1.00 * config.reprocessing_asteroid())) +
            (lavish_sperrylite                   * (40f64 * 1.15 * config.reprocessing_asteroid())) +
            (shimmering_sperrylite               * (40f64 * 2.00 * config.reprocessing_asteroid())) +
            (compressed_sperrylite               * (40f64 * 1.00 * config.reprocessing_asteroid())) +
            (compressed_lavish_sperrylite        * (40f64 * 1.15 * config.reprocessing_asteroid())) +
            (compressed_shimmering_sperrylite    * (40f64 * 2.00 * config.reprocessing_asteroid())) +

            (loparite                            * (10f64 * 1.00 * config.reprocessing_asteroid())) +
            (bountiful_loparite                  * (10f64 * 1.15 * config.reprocessing_asteroid())) +
            (shining_loparite                    * (10f64 * 2.00 * config.reprocessing_asteroid())) +
            (compressed_loparite                 * (10f64 * 1.00 * config.reprocessing_asteroid())) +
            (compressed_bountiful_loparite       * (10f64 * 1.15 * config.reprocessing_asteroid())) +
            (compressed_shining_loparite         * (10f64 * 2.00 * config.reprocessing_asteroid())) +

            platinum >= config.want_mineral(Mineral::Platinum)
        ).set_name("Platinum".into()))
        .with(constraint!(
            (vanadinite                          * (40f64 * 1.00 * config.reprocessing_asteroid())) +
            (lavish_vanadinite                   * (40f64 * 1.15 * config.reprocessing_asteroid())) +
            (shimmering_vanadinite               * (40f64 * 2.00 * config.reprocessing_asteroid())) +
            (compressed_vanadinite               * (40f64 * 1.00 * config.reprocessing_asteroid())) +
            (compressed_lavish_vanadinite        * (40f64 * 1.15 * config.reprocessing_asteroid())) +
            (compressed_shimmering_vanadinite    * (40f64 * 2.00 * config.reprocessing_asteroid())) +

            (xenotime                            * (10f64 * 1.00 * config.reprocessing_asteroid())) +
            (bountiful_xenotime                  * (10f64 * 1.15 * config.reprocessing_asteroid())) +
            (shining_xenotime                    * (10f64 * 2.00 * config.reprocessing_asteroid())) +
            (compressed_xenotime                 * (10f64 * 1.00 * config.reprocessing_asteroid())) +
            (compressed_bountiful_xenotime       * (10f64 * 1.15 * config.reprocessing_asteroid())) +
            (compressed_shining_xenotime         * (10f64 * 2.00 * config.reprocessing_asteroid())) +

            vanadium >= config.want_mineral(Mineral::Vanadium)
        ).set_name("Vanadium".into()))
        .with(constraint!(
            (carnotite                           * (50f64 * 1.00 * config.reprocessing_asteroid())) +
            (replete_carnotite                   * (50f64 * 1.15 * config.reprocessing_asteroid())) +
            (glowing_carnotite                   * (50f64 * 2.00 * config.reprocessing_asteroid())) +
            (compressed_carnotite                * (50f64 * 1.00 * config.reprocessing_asteroid())) +
            (compressed_replete_carnotite        * (50f64 * 1.15 * config.reprocessing_asteroid())) +
            (compressed_glowing_carnotite        * (50f64 * 2.00 * config.reprocessing_asteroid())) +

            technetium >= config.want_mineral(Mineral::Technetium)
        ).set_name("Technetium".into()))
        .with(constraint!(
            (cinnabar                            * (50f64 * 1.00 * config.reprocessing_asteroid())) +
            (replete_cinnabar                    * (50f64 * 1.15 * config.reprocessing_asteroid())) +
            (glowing_cinnabar                    * (50f64 * 2.00 * config.reprocessing_asteroid())) +
            (compressed_cinnabar                 * (50f64 * 1.00 * config.reprocessing_asteroid())) +
            (compressed_replete_cinnabar         * (50f64 * 1.15 * config.reprocessing_asteroid())) +
            (compressed_glowing_cinnabar         * (50f64 * 2.00 * config.reprocessing_asteroid())) +

            mercury >= config.want_mineral(Mineral::Mercury)
        ).set_name("Mercury".into()))
        .with(constraint!(
            (pollucite                           * (50f64 * 1.00 * config.reprocessing_asteroid())) +
            (replete_pollucite                   * (50f64 * 1.15 * config.reprocessing_asteroid())) +
            (glowing_pollucite                   * (50f64 * 2.00 * config.reprocessing_asteroid())) +
            (compressed_pollucite                * (50f64 * 1.00 * config.reprocessing_asteroid())) +
            (compressed_replete_pollucite        * (50f64 * 1.15 * config.reprocessing_asteroid())) +
            (compressed_glowing_pollucite        * (50f64 * 2.00 * config.reprocessing_asteroid())) +

            caesium >= config.want_mineral(Mineral::Caesium)
        ).set_name("Caesium".into()))
        .with(constraint!(
            (zircon                              * (50f64 * 1.00 * config.reprocessing_asteroid())) +
            (replete_zircon                      * (50f64 * 1.15 * config.reprocessing_asteroid())) +
            (glowing_zircon                      * (50f64 * 2.00 * config.reprocessing_asteroid())) +
            (compressed_zircon                   * (50f64 * 1.00 * config.reprocessing_asteroid())) +
            (compressed_replete_zircon           * (50f64 * 1.15 * config.reprocessing_asteroid())) +
            (compressed_glowing_zircon           * (50f64 * 2.00 * config.reprocessing_asteroid())) +

            hafnium >= config.want_mineral(Mineral::Hafnium)
        ).set_name("Hafnium".into()))
        .with(constraint!(
            (loparite                            * (22f64 * 1.00 * config.reprocessing_asteroid())) +
            (bountiful_loparite                  * (22f64 * 1.15 * config.reprocessing_asteroid())) +
            (shining_loparite                    * (22f64 * 2.00 * config.reprocessing_asteroid())) +
            (compressed_loparite                 * (22f64 * 1.00 * config.reprocessing_asteroid())) +
            (compressed_bountiful_loparite       * (22f64 * 1.15 * config.reprocessing_asteroid())) +
            (compressed_shining_loparite         * (22f64 * 2.00 * config.reprocessing_asteroid())) +

            promethium >= config.want_mineral(Mineral::Promethium)
        ).set_name("Promethium".into()))
        .with(constraint!(
            (monazite                            * (22f64 * 1.00 * config.reprocessing_asteroid())) +
            (bountiful_monazite                  * (22f64 * 1.15 * config.reprocessing_asteroid())) +
            (shining_monazite                    * (22f64 * 2.00 * config.reprocessing_asteroid())) +
            (compressed_monazite                 * (22f64 * 1.00 * config.reprocessing_asteroid())) +
            (compressed_bountiful_monazite       * (22f64 * 1.15 * config.reprocessing_asteroid())) +
            (compressed_shining_monazite         * (22f64 * 2.00 * config.reprocessing_asteroid())) +

            neodymium >= config.want_mineral(Mineral::Neodymium)
        ).set_name("Neodymium".into()))
        .with(constraint!(
            (xenotime                            * (22f64 * 1.00 * config.reprocessing_asteroid())) +
            (bountiful_xenotime                  * (22f64 * 1.15 * config.reprocessing_asteroid())) +
            (shining_xenotime                    * (22f64 * 2.00 * config.reprocessing_asteroid())) +
            (compressed_xenotime                 * (22f64 * 1.00 * config.reprocessing_asteroid())) +
            (compressed_bountiful_xenotime       * (22f64 * 1.15 * config.reprocessing_asteroid())) +
            (compressed_shining_xenotime         * (22f64 * 2.00 * config.reprocessing_asteroid())) +

            dysprosium >= config.want_mineral(Mineral::Dysprosium)
        ).set_name("Dysprosium".into()))
        .with(constraint!(
            (ytterbite                           * (22f64 * 1.00 * config.reprocessing_asteroid())) +
            (bountiful_ytterbite                 * (22f64 * 1.15 * config.reprocessing_asteroid())) +
            (shining_ytterbite                   * (22f64 * 2.00 * config.reprocessing_asteroid())) +
            (compressed_ytterbite                * (22f64 * 1.00 * config.reprocessing_asteroid())) +
            (compressed_bountiful_ytterbite      * (22f64 * 1.15 * config.reprocessing_asteroid())) +
            (compressed_shining_ytterbite        * (22f64 * 2.00 * config.reprocessing_asteroid())) +

            thulium >= config.want_mineral(Mineral::Thulium)
        ).set_name("Thulium".into()))
        .solve()
        .map_err(|_| Error::NoSolution)?;

    // add all asteroids to the result
    let mut result = HashMap::new();
    for (asteroid, var) in entries.iter() {
        let entry = solution.value(*var);
        if entry > 0f64 {
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
                Asteroid::Thulium => result.insert(asteroid.to_type_id(), entry.ceil()),
                _ => result.insert(asteroid.to_type_id(), entry.ceil() * 100f64)
            };
        }
    }

    Ok(result)
}

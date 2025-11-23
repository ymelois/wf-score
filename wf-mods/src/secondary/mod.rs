mod accelerated_isotope;
mod amalgam_barrel_diffusion;
mod anemic_agility;
mod augur_pact;
mod barrel_diffusion;
mod convulsion;
mod creeping_bullseye;
mod deep_freeze;
mod expel_grineer;
mod frostbite;
mod galvanized_crosshairs;
mod galvanized_diffusion;
mod galvanized_shot;
mod gunslinger;
mod heated_charge;
mod hollow_point;
mod hornet_strike;
mod ice_storm;
mod jolt;
mod lethal_torrent;
mod magnum_force;
mod pathogen_rounds;
mod pistol_gambit;
mod pistol_pestilence;
mod primed_convulsion;
mod primed_expel_grineer;
mod primed_heated_charge;
mod primed_pistol_gambit;
mod primed_target_cracker;
mod scorch;
mod secondary_riven;
mod sharpened_bullet;
mod target_cracker;

use std::sync::Arc;

use wf_stats::Modifier;

pub use self::accelerated_isotope::AcceleratedIsotope;
pub use self::amalgam_barrel_diffusion::AmalgamBarrelDiffusion;
pub use self::anemic_agility::AnemicAgility;
pub use self::augur_pact::AugurPact;
pub use self::barrel_diffusion::BarrelDiffusion;
pub use self::convulsion::Convulsion;
pub use self::creeping_bullseye::CreepingBullseye;
pub use self::deep_freeze::DeepFreeze;
pub use self::expel_grineer::ExpelGrineer;
pub use self::frostbite::Frostbite;
pub use self::galvanized_crosshairs::GalvanizedCrosshairs;
pub use self::galvanized_diffusion::GalvanizedDiffusion;
pub use self::galvanized_shot::GalvanizedShot;
pub use self::gunslinger::Gunslinger;
pub use self::heated_charge::HeatedCharge;
pub use self::hollow_point::HollowPoint;
pub use self::hornet_strike::HornetStrike;
pub use self::ice_storm::IceStorm;
pub use self::jolt::Jolt;
pub use self::lethal_torrent::LethalTorrent;
pub use self::magnum_force::MagnumForce;
pub use self::pathogen_rounds::PathogenRounds;
pub use self::pistol_gambit::PistolGambit;
pub use self::pistol_pestilence::PistolPestilence;
pub use self::primed_convulsion::PrimedConvulsion;
pub use self::primed_expel_grineer::PrimedExpelGrineer;
pub use self::primed_heated_charge::PrimedHeatedCharge;
pub use self::primed_pistol_gambit::PrimedPistolGambit;
pub use self::primed_target_cracker::PrimedTargetCracker;
pub use self::scorch::Scorch;
pub use self::secondary_riven::SecondaryRiven;
pub use self::sharpened_bullet::SharpenedBullet;
pub use self::target_cracker::TargetCracker;

#[derive(Debug, Clone, PartialEq)]
pub enum SecondaryMod {
    AcceleratedIsotope,
    AmalgamBarrelDiffusion,
    CreepingBullseye,
    HollowPoint,
    MagnumForce,
    PrimedConvulsion,
    Scorch,
    AnemicAgility,
    DeepFreeze,
    HornetStrike,
    PrimedHeatedCharge,
    AugurPact,
    Frostbite,
    IceStorm,
    PathogenRounds,
    PrimedPistolGambit,
    SharpenedBullet,
    BarrelDiffusion,
    GalvanizedCrosshairs(u8),
    GalvanizedDiffusion(u8),
    GalvanizedShot(f32),
    Gunslinger,
    Jolt,
    PistolGambit,
    PrimedExpelGrineer,
    ExpelGrineer,
    Convulsion,
    HeatedCharge,
    LethalTorrent,
    PistolPestilence,
    PrimedTargetCracker,
    TargetCracker,
    Riven(SecondaryRiven),
}

impl From<SecondaryMod> for Arc<dyn Modifier> {
    fn from(value: SecondaryMod) -> Self {
        match value {
            SecondaryMod::AcceleratedIsotope => Arc::new(AcceleratedIsotope),
            SecondaryMod::AmalgamBarrelDiffusion => Arc::new(AmalgamBarrelDiffusion),
            SecondaryMod::CreepingBullseye => Arc::new(CreepingBullseye),
            SecondaryMod::HollowPoint => Arc::new(HollowPoint),
            SecondaryMod::MagnumForce => Arc::new(MagnumForce),
            SecondaryMod::PrimedConvulsion => Arc::new(PrimedConvulsion),
            SecondaryMod::Scorch => Arc::new(Scorch),
            SecondaryMod::AnemicAgility => Arc::new(AnemicAgility),
            SecondaryMod::DeepFreeze => Arc::new(DeepFreeze),
            SecondaryMod::HornetStrike => Arc::new(HornetStrike),
            SecondaryMod::PrimedHeatedCharge => Arc::new(PrimedHeatedCharge),
            SecondaryMod::AugurPact => Arc::new(AugurPact),
            SecondaryMod::Frostbite => Arc::new(Frostbite),
            SecondaryMod::IceStorm => Arc::new(IceStorm),
            SecondaryMod::PathogenRounds => Arc::new(PathogenRounds),
            SecondaryMod::PrimedPistolGambit => Arc::new(PrimedPistolGambit),
            SecondaryMod::SharpenedBullet => Arc::new(SharpenedBullet),
            SecondaryMod::BarrelDiffusion => Arc::new(BarrelDiffusion),
            SecondaryMod::GalvanizedCrosshairs(stacks) => Arc::new(GalvanizedCrosshairs {
                stacks,
            }),
            SecondaryMod::GalvanizedDiffusion(stacks) => Arc::new(GalvanizedDiffusion {
                stacks,
            }),
            SecondaryMod::GalvanizedShot(threshold) => Arc::new(GalvanizedShot {
                threshold,
            }),
            SecondaryMod::Gunslinger => Arc::new(Gunslinger),
            SecondaryMod::Jolt => Arc::new(Jolt),
            SecondaryMod::PistolGambit => Arc::new(PistolGambit),
            SecondaryMod::PrimedExpelGrineer => Arc::new(PrimedExpelGrineer),
            SecondaryMod::ExpelGrineer => Arc::new(ExpelGrineer),
            SecondaryMod::Convulsion => Arc::new(Convulsion),
            SecondaryMod::HeatedCharge => Arc::new(HeatedCharge),
            SecondaryMod::LethalTorrent => Arc::new(LethalTorrent),
            SecondaryMod::PistolPestilence => Arc::new(PistolPestilence),
            SecondaryMod::PrimedTargetCracker => Arc::new(PrimedTargetCracker),
            SecondaryMod::TargetCracker => Arc::new(TargetCracker),
            SecondaryMod::Riven(riven) => Arc::new(riven),
        }
    }
}

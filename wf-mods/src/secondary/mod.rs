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

impl Into<Arc<dyn Modifier>> for SecondaryMod {
    fn into(self) -> Arc<dyn Modifier> {
        match self {
            Self::AcceleratedIsotope => Arc::new(AcceleratedIsotope),
            Self::AmalgamBarrelDiffusion => Arc::new(AmalgamBarrelDiffusion),
            Self::CreepingBullseye => Arc::new(CreepingBullseye),
            Self::HollowPoint => Arc::new(HollowPoint),
            Self::MagnumForce => Arc::new(MagnumForce),
            Self::PrimedConvulsion => Arc::new(PrimedConvulsion),
            Self::Scorch => Arc::new(Scorch),
            Self::AnemicAgility => Arc::new(AnemicAgility),
            Self::DeepFreeze => Arc::new(DeepFreeze),
            Self::HornetStrike => Arc::new(HornetStrike),
            Self::PrimedHeatedCharge => Arc::new(PrimedHeatedCharge),
            Self::AugurPact => Arc::new(AugurPact),
            Self::Frostbite => Arc::new(Frostbite),
            Self::IceStorm => Arc::new(IceStorm),
            Self::PathogenRounds => Arc::new(PathogenRounds),
            Self::PrimedPistolGambit => Arc::new(PrimedPistolGambit),
            Self::SharpenedBullet => Arc::new(SharpenedBullet),
            Self::BarrelDiffusion => Arc::new(BarrelDiffusion),
            Self::GalvanizedCrosshairs(stacks) => Arc::new(GalvanizedCrosshairs {
                stacks,
            }),
            Self::GalvanizedDiffusion(stacks) => Arc::new(GalvanizedDiffusion {
                stacks,
            }),
            Self::GalvanizedShot(threshold) => Arc::new(GalvanizedShot {
                threshold,
            }),
            Self::Gunslinger => Arc::new(Gunslinger),
            Self::Jolt => Arc::new(Jolt),
            Self::PistolGambit => Arc::new(PistolGambit),
            Self::PrimedExpelGrineer => Arc::new(PrimedExpelGrineer),
            Self::ExpelGrineer => Arc::new(ExpelGrineer),
            Self::Convulsion => Arc::new(Convulsion),
            Self::HeatedCharge => Arc::new(HeatedCharge),
            Self::LethalTorrent => Arc::new(LethalTorrent),
            Self::PistolPestilence => Arc::new(PistolPestilence),
            Self::PrimedTargetCracker => Arc::new(PrimedTargetCracker),
            Self::TargetCracker => Arc::new(TargetCracker),
            Self::Riven(riven) => Arc::new(riven),
        }
    }
}

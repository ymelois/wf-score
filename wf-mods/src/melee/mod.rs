mod berserker_fury;
mod blood_rush;
mod condition_overload;
mod fever_strike;
mod focus_energy;
mod focus_radon;
mod gladiator_might;
mod melee_riven;
mod molten_impact;
mod north_wind;
mod organ_shatter;
mod pressure_point;
mod primed_fever_strike;
mod primed_fury;
mod primed_pressure_point;
mod primed_smite_grineer;
mod sacrificial_steel;
mod shocking_touch;
mod smite_grineer;
mod vicious_frost;
mod virulent_scourge;
mod volcanic_edge;
mod voltaic_strike;
mod weeping_wounds;

use std::sync::Arc;

use wf_stats::Modifier;

pub use self::berserker_fury::BerserkerFury;
pub use self::blood_rush::BloodRush;
pub use self::condition_overload::ConditionOverload;
pub use self::fever_strike::FeverStrike;
pub use self::focus_energy::FocusEnergy;
pub use self::focus_radon::FocusRadon;
pub use self::gladiator_might::GladiatorMight;
pub use self::melee_riven::MeleeRiven;
pub use self::molten_impact::MoltenImpact;
pub use self::north_wind::NorthWind;
pub use self::organ_shatter::OrganShatter;
pub use self::pressure_point::PressurePoint;
pub use self::primed_fever_strike::PrimedFeverStrike;
pub use self::primed_fury::PrimedFury;
pub use self::primed_pressure_point::PrimedPressurePoint;
pub use self::primed_smite_grineer::PrimedSmiteGrineer;
pub use self::sacrificial_steel::SacrificialSteel;
pub use self::shocking_touch::ShockingTouch;
pub use self::smite_grineer::SmiteGrineer;
pub use self::vicious_frost::ViciousFrost;
pub use self::virulent_scourge::VirulentScourge;
pub use self::volcanic_edge::VolcanicEdge;
pub use self::voltaic_strike::VoltaicStrike;
pub use self::weeping_wounds::WeepingWounds;

#[derive(Debug, Clone, PartialEq)]
pub enum MeleeMod {
    BerserkerFury,

    /// The combo multiplier of the weapon
    BloodRush(u8),

    /// The % of the total damage that a status must have to count as a
    /// condition
    ConditionOverload(f32),
    FeverStrike,
    FocusEnergy,
    FocusRadon,

    /// The combo multiplier of the weapon
    GladiatorMight(u8),
    MoltenImpact,
    NorthWind,
    OrganShatter,
    PressurePoint,
    PrimedFeverStrike,
    PrimedFury,
    PrimedPressurePoint,
    PrimedSmiteGrineer,
    SacrificialSteel,
    ShockingTouch,
    SmiteGrineer,
    ViciousFrost,
    VirulentScourge,
    VolcanicEdge,
    VoltaicStrike,

    /// The combo multiplier of the weapon
    WeepingWounds(u8),

    Riven(MeleeRiven),
}

impl From<MeleeMod> for Arc<dyn Modifier> {
    fn from(value: MeleeMod) -> Self {
        match value {
            MeleeMod::BerserkerFury => Arc::new(BerserkerFury),
            MeleeMod::BloodRush(combo_multiplier) => Arc::new(BloodRush {
                combo_multiplier,
            }),
            MeleeMod::ConditionOverload(threshold) => Arc::new(ConditionOverload {
                threshold,
            }),
            MeleeMod::FeverStrike => Arc::new(FeverStrike),
            MeleeMod::FocusEnergy => Arc::new(FocusEnergy),
            MeleeMod::FocusRadon => Arc::new(FocusRadon),
            MeleeMod::GladiatorMight(combo_multiplier) => Arc::new(GladiatorMight {
                combo_multiplier,
            }),
            MeleeMod::MoltenImpact => Arc::new(MoltenImpact),
            MeleeMod::NorthWind => Arc::new(NorthWind),
            MeleeMod::OrganShatter => Arc::new(OrganShatter),
            MeleeMod::PressurePoint => Arc::new(PressurePoint),
            MeleeMod::PrimedFeverStrike => Arc::new(PrimedFeverStrike),
            MeleeMod::PrimedFury => Arc::new(PrimedFury),
            MeleeMod::PrimedPressurePoint => Arc::new(PrimedPressurePoint),
            MeleeMod::PrimedSmiteGrineer => Arc::new(PrimedSmiteGrineer),
            MeleeMod::SacrificialSteel => Arc::new(SacrificialSteel),
            MeleeMod::ShockingTouch => Arc::new(ShockingTouch),
            MeleeMod::SmiteGrineer => Arc::new(SmiteGrineer),
            MeleeMod::ViciousFrost => Arc::new(ViciousFrost),
            MeleeMod::VirulentScourge => Arc::new(VirulentScourge),
            MeleeMod::VolcanicEdge => Arc::new(VolcanicEdge),
            MeleeMod::VoltaicStrike => Arc::new(VoltaicStrike),
            MeleeMod::WeepingWounds(combo_multiplier) => Arc::new(WeepingWounds {
                combo_multiplier,
            }),
            MeleeMod::Riven(riven) => Arc::new(riven),
        }
    }
}

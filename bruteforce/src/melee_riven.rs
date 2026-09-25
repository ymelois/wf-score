use itertools::Itertools as _;
use wf_mods::melee::*;
use wf_stats::*;

const MELEE_RIVEN_BASE_BONUS_LIST: [MeleeRivenAttribute; 9] = [
    MeleeRivenAttribute::CriticalChance(1.80),
    MeleeRivenAttribute::CriticalMultiplier(0.9),
    MeleeRivenAttribute::Damage(1.647),
    MeleeRivenAttribute::Status(Status::new().cold(0.9)),
    MeleeRivenAttribute::Status(Status::new().electricity(0.9)),
    MeleeRivenAttribute::Status(Status::new().heat(0.9)),
    MeleeRivenAttribute::Status(Status::new().toxin(0.9)),
    MeleeRivenAttribute::StatusChance(0.9),
    MeleeRivenAttribute::AttackSpeed(0.549),
];

#[derive(Clone, Debug)]
pub enum MeleeRivenAttribute {
    CriticalChance(f32),
    CriticalMultiplier(f32),
    Damage(f32),
    Status(Status),
    StatusChance(f32),
    AttackSpeed(f32),
}

impl<'local> FromIterator<&'local MeleeRivenAttribute> for MeleeRiven {
    fn from_iter<T: IntoIterator<Item = &'local MeleeRivenAttribute>>(iter: T) -> Self {
        use MeleeRivenAttribute as Attrs;

        let mut critical_chance = 0.0;
        let mut critical_multiplier = 0.0;
        let mut damage = 0.0;
        let mut status_chance = 0.0;
        let mut attack_speed = 0.0;
        let mut status = Status::new();

        for attribute in iter {
            match attribute {
                Attrs::CriticalChance(value) => critical_chance = *value,
                Attrs::CriticalMultiplier(value) => critical_multiplier = *value,
                Attrs::Damage(value) => damage = *value,
                Attrs::Status(status2) => status += status2,
                Attrs::StatusChance(value) => status_chance = *value,
                Attrs::AttackSpeed(value) => attack_speed = *value,
            }
        }

        MeleeRiven {
            damage,
            critical_chance,
            critical_multiplier,
            status_chance,
            attack_speed,
            status,
        }
    }
}

/// Generate all possible riven combinations for a melee weapon
///
/// # Arguments
///
/// * `disposition` - The riven disposition of the weapon
/// * `attribute_count` - The number of attributes in the riven
/// * `has_negative` - Whether the riven has a negative attribute (used for
///   bonus calculation)
pub fn generate_melee_riven_combinations(
    disposition: f32,
    attribute_count: usize,
    has_negative: bool,
) -> impl Iterator<Item = MeleeMod> {
    let bonus = match (attribute_count, has_negative) {
        (2, false) => 0.99,
        (2, true) => 1.2375,
        (3, false) => 0.75,
        (3, true) => 0.9375,
        _ => panic!("Invalid attribute count"),
    };

    MELEE_RIVEN_BASE_BONUS_LIST
        .iter()
        .combinations(attribute_count)
        .map(|c| c.into_iter().collect())
        .map(move |riven: MeleeRiven| MeleeMod::Riven(riven * disposition * bonus))
}

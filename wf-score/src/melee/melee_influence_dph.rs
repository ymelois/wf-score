use wf_stats::{
    Melee,
    Weapon as _,
};

/// Calculate the average damage per hit of the melee influence arcane on a
/// melee weapon
///
/// # Arguments
///
/// * `melee` - The melee weapon
/// * `animation_time` - The animation time of the combo from the stance mod
/// * `combo_hits` - The number of hits in the combo
#[must_use]
pub fn melee_influence_dph(
    melee: &Melee,
    _animation_time: f32,
    _combo_hits: f32,
) -> f32 {
    let damage_bonus = melee.damage_bonus();
    let critical_chance = melee.critical_chance();
    let critical_multiplier = melee.critical_multiplier();
    let anti_faction = melee.anti_faction();
    let status_chance = melee.status_chance();
    let status = melee.status();
    let total_damage = status.sum();

    let electricity_damage = status.electricity;

    if electricity_damage == 0.0 {
        return 0.0;
    }

    // Chance per hit
    let melee_influence_base_chance = 0.2;
    let melee_influence_chance =
        electricity_damage / total_damage * status_chance * melee_influence_base_chance;
    let melee_influence_chance = melee_influence_chance.min(1.0); // Cap at 100%

    // Damage per hit
    let melee_influence_base_damage =
        total_damage - (status.impact + status.puncture + status.slash);
    let melee_influence_damage = melee_influence_base_damage
        * (1.0 + damage_bonus)
        * (1.0 + critical_chance * (critical_multiplier - 1.0))
        * (1.0 + anti_faction).powf(2.0);

    // Average damage per hit
    melee_influence_chance * melee_influence_damage
}

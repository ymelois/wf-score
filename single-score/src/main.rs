#![allow(
    dead_code,
    unused_imports,
    unused_mut,
    unused_variables,
    clippy::wildcard_imports,
    reason = "testing"
)]

use wf_mods::melee::*;
use wf_mods::secondary::*;
use wf_score::melee::*;
use wf_score::secondary::*;
use wf_stats::*;

fn main() {
    let mut dual_toxocyst = Secondary {
        // Incarnon Genesis 4th evolution: Commodore's Fortune
        // +20% base critical chance
        // Default critical chance: 0.05
        critical_chance: 0.25,
        critical_multiplier: 2.0,
        status_chance: 37.0,
        fire_rate: 1.0,
        multishot: 1.0,
        ammo_maximum: 72.0,
        magazine_size: 12.0,
        reload_time: 2.35,
        reload_delay: 0.0,
        status_list: vec![
            Status::impact(7.5),
            Status::puncture(60.0),
            Status::slash(7.5),
            // Frenzy buff
            Status::toxin(75.0),
        ],
        modifier_list: Vec::new(),
    };

    let riven_mod = SecondaryMod::Riven(SecondaryRiven {
        damage: 2.703,
        critical_chance: 1.994,
        critical_multiplier: 1.227,
        fire_rate: 0.0,
        ammo_maximum: 0.0,
        magazine_capacity: 0.0,
        multishot: 0.0,
        reload_speed: 0.0,
        status_chance: 0.0,
        status_list: vec![],
    });

    let build = vec![
        SecondaryMod::PrimedConvulsion,
        riven_mod,
        SecondaryMod::AcceleratedIsotope,
        SecondaryMod::HornetStrike,
        SecondaryMod::PrimedTargetCracker,
        SecondaryMod::GalvanizedDiffusion(4),
        SecondaryMod::GalvanizedCrosshairs(5),
    ];

    for modifier in build {
        dual_toxocyst.add_modifier(modifier.into());
    }

    let score = raw_damage(&dual_toxocyst);

    println!("Score: {score}");
}

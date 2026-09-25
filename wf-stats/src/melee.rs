use std::sync::Arc;

use derive_more::Debug;

use crate::modifier::{
    Modifier,
    WeaponModifiers,
};
use crate::status::Status;
use crate::weapon::Weapon;

#[derive(Debug, Default, Clone)]
pub struct Melee {
    pub critical_chance: f32,
    pub critical_multiplier: f32,
    pub status_chance: f32,
    pub attack_speed: f32,
    pub status: Status,
    #[debug(skip)]
    pub modifier_list: Vec<Arc<dyn Modifier>>,
}

impl WeaponModifiers for Melee {
    fn add_modifier(
        &mut self,
        modifier: Arc<dyn Modifier>,
    ) {
        self.modifier_list.push(modifier);
    }
}

impl Weapon for Melee {
    fn fire_rate(&self) -> f32 { 0.0 }

    fn ammo_maximum(&self) -> f32 { 0.0 }

    fn magazine_capacity(&self) -> f32 { 0.0 }

    fn multishot(&self) -> f32 { 0.0 }

    fn reload_speed(&self) -> f32 { 0.0 }

    fn reload_delay(&self) -> f32 { 0.0 }

    fn damage_bonus(&self) -> f32 {
        let mut damage_bonus = 0.0;
        for modifier in &self.modifier_list {
            damage_bonus += modifier.damage(self);
        }
        damage_bonus
    }

    fn anti_faction(&self) -> f32 {
        let mut anti_faction = 0.0;
        for modifier in &self.modifier_list {
            let modifier_anti_faction = modifier.anti_faction(self);
            if modifier_anti_faction > anti_faction {
                anti_faction = modifier_anti_faction;
            }
        }
        anti_faction
    }

    fn critical_chance(&self) -> f32 {
        let mut critical_chance = 0.0;
        for modifier in &self.modifier_list {
            critical_chance += modifier.critical_chance(self);
        }
        self.critical_chance * (1.0 + critical_chance)
    }

    fn critical_multiplier(&self) -> f32 {
        let mut critical_multiplier = 0.0;
        for modifier in &self.modifier_list {
            critical_multiplier += modifier.critical_multiplier(self);
        }
        self.critical_multiplier * (1.0 + critical_multiplier)
    }

    fn status_chance(&self) -> f32 {
        let mut status_chance = 0.0;
        for modifier in &self.modifier_list {
            status_chance += modifier.status_chance(self);
        }
        self.status_chance * (1.0 + status_chance)
    }

    fn attack_speed(&self) -> f32 {
        let mut attack_speed = 0.0;
        for modifier in &self.modifier_list {
            attack_speed += modifier.attack_speed(self);
        }
        self.attack_speed * (1.0 + attack_speed)
    }

    fn status(&self) -> Status {
        let mut status = Status::new();

        for modifier in &self.modifier_list {
            status.apply(&modifier.status(self), &self.status);
        }

        status += &self.status;
        status
    }

    fn modifier_list(&self) -> &Vec<Arc<dyn Modifier>> { &self.modifier_list }

    fn cost(
        &self,
        has_reactor: bool,
    ) -> u8 {
        let mut cost = 0;
        for modifier in &self.modifier_list {
            cost += if has_reactor {
                modifier.cost(self).div_ceil(2_u8)
            } else {
                modifier.cost(self)
            };
        }
        cost
    }
}

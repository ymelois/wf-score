use std::sync::Arc;

use derive_more::Debug;

use crate::modifier::{
    Modifier,
    WeaponModifiers,
};
use crate::status::Status;
use crate::weapon::Weapon;

#[derive(Debug, Clone)]
pub struct Secondary {
    pub critical_chance: f32,
    pub critical_multiplier: f32,
    pub status_chance: f32,
    pub fire_rate: f32,
    pub multishot: f32,
    pub ammo_maximum: f32,
    pub magazine_size: f32,
    pub reload_time: f32,
    pub reload_delay: f32,
    pub status: Status,
    #[debug(skip)]
    pub modifier_list: Vec<Arc<dyn Modifier>>,
}

impl WeaponModifiers for Secondary {
    fn add_modifier(
        &mut self,
        modifier: Arc<dyn Modifier>,
    ) {
        self.modifier_list.push(modifier);
    }
}

impl Weapon for Secondary {
    fn attack_speed(&self) -> f32 { 0.0 }

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

    fn fire_rate(&self) -> f32 {
        let mut fire_rate = 0.0;
        for modifier in &self.modifier_list {
            fire_rate += modifier.fire_rate(self);
        }
        self.fire_rate * (1.0 + fire_rate)
    }

    fn ammo_maximum(&self) -> f32 {
        let mut ammo_maximum = 0.0;
        for modifier in &self.modifier_list {
            ammo_maximum += modifier.ammo_maximum(self);
        }
        (self.ammo_maximum * (1.0 + ammo_maximum)).floor()
    }

    fn magazine_capacity(&self) -> f32 {
        let mut magazine_capacity = 0.0;
        for modifier in &self.modifier_list {
            magazine_capacity += modifier.magazine_capacity(self);
        }
        (self.magazine_size * (1.0 + magazine_capacity)).floor()
    }

    fn multishot(&self) -> f32 {
        let mut multishot = 0.0;
        for modifier in &self.modifier_list {
            multishot += modifier.multishot(self);
        }
        self.multishot * (1.0 + multishot)
    }

    fn reload_speed(&self) -> f32 {
        let mut reload_speed = 0.0;
        for modifier in &self.modifier_list {
            reload_speed += modifier.reload_speed(self);
        }
        self.reload_time / (1.0 + reload_speed)
    }

    fn reload_delay(&self) -> f32 { self.reload_delay }

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

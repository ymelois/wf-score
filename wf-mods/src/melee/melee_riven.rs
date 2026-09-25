use wf_stats::{
    Modifier,
    Status,
    Weapon,
};

#[derive(Debug, Clone, PartialEq)]
pub struct MeleeRiven {
    pub damage: f32,
    pub critical_chance: f32,
    pub critical_multiplier: f32,
    pub status_chance: f32,
    pub attack_speed: f32,
    pub status: Status,
}

impl Modifier for MeleeRiven {
    fn damage(
        &self,
        _context: &dyn Weapon,
    ) -> f32 {
        self.damage
    }

    fn critical_chance(
        &self,
        _context: &dyn Weapon,
    ) -> f32 {
        self.critical_chance
    }

    fn critical_multiplier(
        &self,
        _context: &dyn Weapon,
    ) -> f32 {
        self.critical_multiplier
    }

    fn status_chance(
        &self,
        _context: &dyn Weapon,
    ) -> f32 {
        self.status_chance
    }

    fn attack_speed(
        &self,
        _context: &dyn Weapon,
    ) -> f32 {
        self.attack_speed
    }

    fn status(
        &self,
        _context: &dyn Weapon,
    ) -> Status {
        self.status.clone()
    }
}

impl std::ops::Mul<f32> for MeleeRiven {
    type Output = Self;

    fn mul(
        self,
        rhs: f32,
    ) -> Self::Output {
        Self {
            damage: self.damage * rhs,
            critical_chance: self.critical_chance * rhs,
            critical_multiplier: self.critical_multiplier * rhs,
            status_chance: self.status_chance * rhs,
            attack_speed: self.attack_speed * rhs,
            status: self.status * rhs,
        }
    }
}

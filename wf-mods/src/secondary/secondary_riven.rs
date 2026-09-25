use wf_stats::{
    Modifier,
    Status,
    Weapon,
};

#[derive(Debug, Clone, PartialEq)]
pub struct SecondaryRiven {
    pub damage: f32,
    pub critical_chance: f32,
    pub critical_multiplier: f32,
    pub fire_rate: f32,
    pub ammo_maximum: f32,
    pub magazine_capacity: f32,
    pub multishot: f32,
    pub reload_speed: f32,
    pub status_chance: f32,
    pub status: Status,
}

impl Modifier for SecondaryRiven {
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

    fn fire_rate(
        &self,
        _context: &dyn Weapon,
    ) -> f32 {
        self.fire_rate
    }

    fn ammo_maximum(
        &self,
        _context: &dyn Weapon,
    ) -> f32 {
        self.ammo_maximum
    }

    fn magazine_capacity(
        &self,
        _context: &dyn Weapon,
    ) -> f32 {
        self.magazine_capacity
    }

    fn multishot(
        &self,
        _context: &dyn Weapon,
    ) -> f32 {
        self.multishot
    }

    fn reload_speed(
        &self,
        _context: &dyn Weapon,
    ) -> f32 {
        self.reload_speed
    }

    fn status_chance(
        &self,
        _context: &dyn Weapon,
    ) -> f32 {
        self.status_chance
    }

    fn status(
        &self,
        _context: &dyn Weapon,
    ) -> Status {
        self.status.clone()
    }

    fn cost(
        &self,
        _context: &dyn Weapon,
    ) -> u8 {
        18
    }
}

impl std::ops::Mul<f32> for SecondaryRiven {
    type Output = Self;

    fn mul(
        self,
        rhs: f32,
    ) -> Self::Output {
        Self {
            damage: self.damage * rhs,
            critical_chance: self.critical_chance * rhs,
            critical_multiplier: self.critical_multiplier * rhs,
            fire_rate: self.fire_rate * rhs,
            ammo_maximum: self.ammo_maximum * rhs,
            magazine_capacity: self.magazine_capacity * rhs,
            multishot: self.multishot * rhs,
            reload_speed: self.reload_speed * rhs,
            status_chance: self.status_chance * rhs,
            status: self.status * rhs,
        }
    }
}

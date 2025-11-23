use wf_stats::{
    Modifier,
    Weapon,
};

#[derive(Debug, Clone, PartialEq)]
pub struct GalvanizedCrosshairs {
    pub stacks: u8,
}

impl Modifier for GalvanizedCrosshairs {
    // When aiming, critical chance is increased by 120% + 40% per headshot kills
    // (up to 5 stacks)
    fn critical_chance(
        &self,
        _context: &dyn Weapon,
    ) -> f32 {
        1.2 + 0.4 * f32::from(self.stacks.min(5))
    }

    fn cost(
        &self,
        _context: &dyn Weapon,
    ) -> u8 {
        12
    }
}

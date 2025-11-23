use wf_stats::{
    Modifier,
    Weapon,
};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct BloodRush {
    pub combo_multiplier: u8,
}

impl Modifier for BloodRush {
    fn critical_chance(
        &self,
        _context: &dyn Weapon,
    ) -> f32 {
        0.4 * f32::from((self.combo_multiplier - 1).max(1))
    }
}

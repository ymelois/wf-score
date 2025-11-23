use wf_stats::{
    Modifier,
    Weapon,
};

#[derive(Default, Clone, Copy, PartialEq)]
pub struct WeepingWounds {
    pub combo_multiplier: u8,
}

impl Modifier for WeepingWounds {
    fn status_chance(
        &self,
        _context: &dyn Weapon,
    ) -> f32 {
        0.4 * f32::from(self.combo_multiplier.saturating_sub(1).max(1))
    }
}

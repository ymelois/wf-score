use wf_stats::{
    Modifier,
    Weapon,
};

#[derive(Default, Clone, Copy, PartialEq)]
pub struct GladiatorMight {
    pub combo_multiplier: u8,
}

impl Modifier for GladiatorMight {
    fn set(&self) -> Option<&str> { Some("Gladiator") }

    fn critical_chance(
        &self,
        context: &dyn Weapon,
    ) -> f32 {
        let mut set_count: u8 = 0;
        for modifier in context.modifier_list() {
            if modifier.set() == Some("Gladiator") {
                set_count += 1;
            }
        }

        let set_bonus = 1.0 * (f32::from(set_count) - 1.0); // Subtract 1 to account for the current mod

        0.1 * f32::from(self.combo_multiplier.saturating_sub(1).max(1)) * (1.0 + set_bonus)
    }

    fn critical_multiplier(
        &self,
        _context: &dyn Weapon,
    ) -> f32 {
        0.6
    }
}

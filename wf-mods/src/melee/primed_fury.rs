use wf_stats::{
    Modifier,
    Weapon,
};

#[derive(Default, Clone, Copy, PartialEq)]
pub struct PrimedFury;

impl Modifier for PrimedFury {
    fn attack_speed(
        &self,
        _context: &dyn Weapon,
    ) -> f32 {
        0.55
    }
}

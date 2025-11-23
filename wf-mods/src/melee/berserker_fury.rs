use wf_stats::{
    Modifier,
    Weapon,
};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct BerserkerFury;

impl Modifier for BerserkerFury {
    fn attack_speed(
        &self,
        _context: &dyn Weapon,
    ) -> f32 {
        0.7
    }
}

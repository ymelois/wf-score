use wf_stats::{
    Modifier,
    Weapon,
};

#[derive(Default, Clone, Copy, PartialEq)]
pub struct PressurePoint;

impl Modifier for PressurePoint {
    fn damage(
        &self,
        _context: &dyn Weapon,
    ) -> f32 {
        1.2
    }
}

use wf_stats::{
    Modifier,
    Weapon,
};

#[derive(Default, Clone, Copy, PartialEq)]
pub struct PrimedPressurePoint;

impl Modifier for PrimedPressurePoint {
    fn damage(
        &self,
        _context: &dyn Weapon,
    ) -> f32 {
        1.65
    }
}

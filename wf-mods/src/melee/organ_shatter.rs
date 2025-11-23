use wf_stats::{
    Modifier,
    Weapon,
};

#[derive(Default, Clone, Copy, PartialEq)]
pub struct OrganShatter;

impl Modifier for OrganShatter {
    fn critical_multiplier(
        &self,
        _context: &dyn Weapon,
    ) -> f32 {
        0.9
    }
}

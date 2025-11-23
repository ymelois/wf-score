use wf_stats::{
    Modifier,
    Weapon,
};

#[derive(Default, Clone, Copy, PartialEq)]
pub struct PrimedSmiteGrineer;

impl Modifier for PrimedSmiteGrineer {
    fn anti_faction(
        &self,
        _context: &dyn Weapon,
    ) -> f32 {
        0.55
    }
}

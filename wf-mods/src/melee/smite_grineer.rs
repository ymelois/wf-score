use wf_stats::{
    Modifier,
    Weapon,
};

#[derive(Default, Clone, Copy, PartialEq)]
pub struct SmiteGrineer;

impl Modifier for SmiteGrineer {
    fn anti_faction(
        &self,
        _context: &dyn Weapon,
    ) -> f32 {
        0.3
    }
}

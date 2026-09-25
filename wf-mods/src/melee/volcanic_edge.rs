use wf_stats::{
    Modifier,
    Status,
    Weapon,
};

#[derive(Default, Clone, Copy, PartialEq)]
pub struct VolcanicEdge;

impl Modifier for VolcanicEdge {
    fn status(
        &self,
        _context: &dyn Weapon,
    ) -> Status {
        Status::new().heat(0.6)
    }

    fn status_chance(
        &self,
        _context: &dyn Weapon,
    ) -> f32 {
        0.6
    }
}

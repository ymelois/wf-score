use wf_stats::{
    Modifier,
    Status,
    Weapon,
};

#[derive(Default, Clone, Copy, PartialEq)]
pub struct VolcanicEdge;

impl Modifier for VolcanicEdge {
    fn status_list(
        &self,
        _context: &dyn Weapon,
    ) -> Vec<Status> {
        vec![Status::heat(0.6)]
    }

    fn status_chance(
        &self,
        _context: &dyn Weapon,
    ) -> f32 {
        0.6
    }
}

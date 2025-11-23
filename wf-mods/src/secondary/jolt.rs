use wf_stats::{
    Modifier,
    Status,
    Weapon,
};

#[derive(Debug, Clone, PartialEq)]
pub struct Jolt;

impl Modifier for Jolt {
    fn status_chance(
        &self,
        _context: &dyn Weapon,
    ) -> f32 {
        0.6
    }

    fn status_list(
        &self,
        _context: &dyn Weapon,
    ) -> Vec<Status> {
        vec![Status::electricity(0.6)]
    }

    fn cost(
        &self,
        _context: &dyn Weapon,
    ) -> u8 {
        7
    }
}

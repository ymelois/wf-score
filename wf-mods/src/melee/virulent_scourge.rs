use wf_stats::{
    Modifier,
    Status,
    Weapon,
};

#[derive(Default, Clone, Copy, PartialEq)]
pub struct VirulentScourge;

impl Modifier for VirulentScourge {
    fn status(
        &self,
        _context: &dyn Weapon,
    ) -> Status {
        Status::new().toxin(0.6)
    }

    fn status_chance(
        &self,
        _context: &dyn Weapon,
    ) -> f32 {
        0.6
    }
}

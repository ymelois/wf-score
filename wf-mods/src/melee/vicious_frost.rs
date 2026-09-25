use wf_stats::{
    Modifier,
    Status,
    Weapon,
};

#[derive(Default, Clone, Copy, PartialEq)]
pub struct ViciousFrost;

impl Modifier for ViciousFrost {
    fn status(
        &self,
        _context: &dyn Weapon,
    ) -> Status {
        Status::new().cold(0.6)
    }

    fn status_chance(
        &self,
        _context: &dyn Weapon,
    ) -> f32 {
        0.6
    }
}

use wf_stats::{
    Modifier,
    Status,
    Weapon,
};

#[derive(Default, Clone, Copy, PartialEq)]
pub struct VoltaicStrike;

impl Modifier for VoltaicStrike {
    fn status(
        &self,
        _context: &dyn Weapon,
    ) -> Status {
        Status::new().electricity(0.6)
    }

    fn status_chance(
        &self,
        _context: &dyn Weapon,
    ) -> f32 {
        0.6
    }
}

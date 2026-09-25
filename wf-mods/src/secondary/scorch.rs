use wf_stats::{
    Modifier,
    Status,
    Weapon,
};

#[derive(Debug, Clone, PartialEq)]
pub struct Scorch;

impl Modifier for Scorch {
    fn status_chance(
        &self,
        _context: &dyn Weapon,
    ) -> f32 {
        0.6
    }

    fn status(
        &self,
        _context: &dyn Weapon,
    ) -> Status {
        Status::new().heat(0.6)
    }

    fn cost(
        &self,
        _context: &dyn Weapon,
    ) -> u8 {
        7
    }
}

use wf_stats::{
    Modifier,
    Status,
    Weapon,
};

#[derive(Debug, Clone, PartialEq)]
pub struct PistolPestilence;

impl Modifier for PistolPestilence {
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
        Status::new().toxin(0.6)
    }

    fn cost(
        &self,
        _context: &dyn Weapon,
    ) -> u8 {
        7
    }
}

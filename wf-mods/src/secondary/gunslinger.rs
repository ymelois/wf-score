use wf_stats::{
    Modifier,
    Weapon,
};

#[derive(Debug, Clone, PartialEq)]
pub struct Gunslinger;

impl Modifier for Gunslinger {
    fn fire_rate(
        &self,
        _context: &dyn Weapon,
    ) -> f32 {
        0.72
    }

    fn cost(
        &self,
        _context: &dyn Weapon,
    ) -> u8 {
        9
    }
}

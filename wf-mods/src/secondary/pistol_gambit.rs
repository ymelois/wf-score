use wf_stats::{
    Modifier,
    Weapon,
};

#[derive(Debug, Clone, PartialEq)]
pub struct PistolGambit;

impl Modifier for PistolGambit {
    fn critical_chance(
        &self,
        _context: &dyn Weapon,
    ) -> f32 {
        1.2
    }

    fn cost(
        &self,
        _context: &dyn Weapon,
    ) -> u8 {
        9
    }
}

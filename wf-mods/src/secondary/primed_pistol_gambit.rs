use wf_stats::{
    Modifier,
    Weapon,
};

#[derive(Debug, Clone, PartialEq)]
pub struct PrimedPistolGambit;

impl Modifier for PrimedPistolGambit {
    fn critical_chance(
        &self,
        _context: &dyn Weapon,
    ) -> f32 {
        1.87
    }

    fn cost(
        &self,
        _context: &dyn Weapon,
    ) -> u8 {
        12
    }
}

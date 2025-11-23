use wf_stats::{
    Modifier,
    Weapon,
};

#[derive(Debug, Clone, PartialEq)]
pub struct TargetCracker;

impl Modifier for TargetCracker {
    fn critical_multiplier(
        &self,
        _context: &dyn Weapon,
    ) -> f32 {
        0.6
    }

    fn cost(
        &self,
        _context: &dyn Weapon,
    ) -> u8 {
        9
    }
}

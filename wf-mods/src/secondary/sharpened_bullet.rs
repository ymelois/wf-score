use wf_stats::{
    Modifier,
    Weapon,
};

#[derive(Debug, Clone, PartialEq)]
pub struct SharpenedBullet;

impl Modifier for SharpenedBullet {
    fn critical_multiplier(
        &self,
        _context: &dyn Weapon,
    ) -> f32 {
        0.75
    }

    fn cost(
        &self,
        _context: &dyn Weapon,
    ) -> u8 {
        7
    }
}

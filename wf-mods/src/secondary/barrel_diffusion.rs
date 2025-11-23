use wf_stats::{
    Modifier,
    Weapon,
};

#[derive(Debug, Clone, PartialEq)]
pub struct BarrelDiffusion;

impl Modifier for BarrelDiffusion {
    fn multishot(
        &self,
        _context: &dyn Weapon,
    ) -> f32 {
        1.2
    }

    fn cost(
        &self,
        _context: &dyn Weapon,
    ) -> u8 {
        11
    }
}

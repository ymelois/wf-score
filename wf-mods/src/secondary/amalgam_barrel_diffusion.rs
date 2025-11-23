use wf_stats::{
    Modifier,
    Weapon,
};

#[derive(Debug, Clone, PartialEq)]
pub struct AmalgamBarrelDiffusion;

impl Modifier for AmalgamBarrelDiffusion {
    fn multishot(
        &self,
        _context: &dyn Weapon,
    ) -> f32 {
        1.1
    }

    fn cost(
        &self,
        _context: &dyn Weapon,
    ) -> u8 {
        15
    }
}

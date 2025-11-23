use wf_stats::{
    Modifier,
    Weapon,
};

#[derive(Debug, Clone, PartialEq)]
pub struct GalvanizedDiffusion {
    pub stacks: u8,
}

impl Modifier for GalvanizedDiffusion {
    fn multishot(
        &self,
        _context: &dyn Weapon,
    ) -> f32 {
        1.1 + 0.3 * f32::from(self.stacks.min(4))
    }

    fn cost(
        &self,
        _context: &dyn Weapon,
    ) -> u8 {
        14
    }
}

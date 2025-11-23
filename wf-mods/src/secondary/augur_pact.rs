use wf_stats::{
    Modifier,
    Weapon,
};

#[derive(Debug, Clone, PartialEq)]
pub struct AugurPact;

impl Modifier for AugurPact {
    fn damage(
        &self,
        _context: &dyn Weapon,
    ) -> f32 {
        0.9
    }

    fn cost(
        &self,
        _context: &dyn Weapon,
    ) -> u8 {
        7
    }
}

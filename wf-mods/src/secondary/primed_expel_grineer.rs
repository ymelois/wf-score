use wf_stats::{
    Modifier,
    Weapon,
};

#[derive(Default, Clone, Copy, PartialEq)]
pub struct PrimedExpelGrineer;

impl Modifier for PrimedExpelGrineer {
    fn anti_faction(
        &self,
        _context: &dyn Weapon,
    ) -> f32 {
        0.55
    }

    fn cost(
        &self,
        _context: &dyn Weapon,
    ) -> u8 {
        14
    }
}

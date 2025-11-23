use wf_stats::{
    Modifier,
    Weapon,
};

#[derive(Default, Clone, Copy, PartialEq)]
pub struct ExpelGrineer;

impl Modifier for ExpelGrineer {
    fn anti_faction(
        &self,
        _context: &dyn Weapon,
    ) -> f32 {
        0.3
    }

    fn cost(
        &self,
        _context: &dyn Weapon,
    ) -> u8 {
        9
    }
}

use wf_stats::{
    Modifier,
    Weapon,
};

#[derive(Debug, Clone, PartialEq)]
pub struct LethalTorrent;

impl Modifier for LethalTorrent {
    fn multishot(
        &self,
        _context: &dyn Weapon,
    ) -> f32 {
        0.6
    }

    fn fire_rate(
        &self,
        _context: &dyn Weapon,
    ) -> f32 {
        0.6
    }

    fn cost(
        &self,
        _context: &dyn Weapon,
    ) -> u8 {
        11
    }
}

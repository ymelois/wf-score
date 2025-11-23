use wf_stats::{
    Modifier,
    Status,
    Weapon,
};

#[derive(Debug, Clone, PartialEq)]
pub struct IceStorm;

impl Modifier for IceStorm {
    fn magazine_capacity(
        &self,
        _context: &dyn Weapon,
    ) -> f32 {
        0.4
    }

    fn status_list(
        &self,
        _context: &dyn Weapon,
    ) -> Vec<Status> {
        vec![Status::cold(0.4)]
    }

    fn cost(
        &self,
        _context: &dyn Weapon,
    ) -> u8 {
        9
    }
}

use wf_stats::{
    Modifier,
    Status,
    Weapon,
};

#[derive(Debug, Clone, PartialEq)]
pub struct AcceleratedIsotope;

impl Modifier for AcceleratedIsotope {
    fn fire_rate(
        &self,
        _context: &dyn Weapon,
    ) -> f32 {
        0.4
    }

    fn status_list(
        &self,
        _context: &dyn Weapon,
    ) -> Vec<Status> {
        vec![Status::radiation(0.6)]
    }

    fn cost(
        &self,
        _context: &dyn Weapon,
    ) -> u8 {
        7
    }
}

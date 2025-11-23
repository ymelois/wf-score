use wf_stats::{
    Modifier,
    Status,
    Weapon,
};

#[derive(Debug, Clone, PartialEq)]
pub struct HeatedCharge;

impl Modifier for HeatedCharge {
    fn status_list(
        &self,
        _context: &dyn Weapon,
    ) -> Vec<Status> {
        vec![Status::heat(0.9)]
    }

    fn cost(
        &self,
        _context: &dyn Weapon,
    ) -> u8 {
        11
    }
}

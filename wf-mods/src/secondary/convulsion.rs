use wf_stats::{
    Modifier,
    Status,
    Weapon,
};

#[derive(Debug, Clone, PartialEq)]
pub struct Convulsion;

impl Modifier for Convulsion {
    fn status_list(
        &self,
        _context: &dyn Weapon,
    ) -> Vec<Status> {
        vec![Status::electricity(0.9)]
    }

    fn cost(
        &self,
        _context: &dyn Weapon,
    ) -> u8 {
        11
    }
}

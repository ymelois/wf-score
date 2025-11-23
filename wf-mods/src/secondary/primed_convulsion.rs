use wf_stats::{
    Modifier,
    Status,
    Weapon,
};

#[derive(Debug, Clone, PartialEq)]
pub struct PrimedConvulsion;

impl Modifier for PrimedConvulsion {
    fn status_list(
        &self,
        _context: &dyn Weapon,
    ) -> Vec<Status> {
        vec![Status::electricity(1.65)]
    }

    fn cost(
        &self,
        _context: &dyn Weapon,
    ) -> u8 {
        16
    }
}

use wf_stats::{
    Modifier,
    Status,
    Weapon,
};

#[derive(Default, Clone, Copy, PartialEq)]
pub struct PrimedFeverStrike;

impl Modifier for PrimedFeverStrike {
    fn status_list(
        &self,
        _context: &dyn Weapon,
    ) -> Vec<Status> {
        vec![Status::toxin(1.65)]
    }
}

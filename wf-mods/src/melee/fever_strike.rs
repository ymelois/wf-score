use wf_stats::{
    Modifier,
    Status,
    Weapon,
};

#[derive(Default, Clone, Copy, PartialEq)]
pub struct FeverStrike;

impl Modifier for FeverStrike {
    fn status_list(
        &self,
        _context: &dyn Weapon,
    ) -> Vec<Status> {
        vec![Status::toxin(0.9)]
    }
}

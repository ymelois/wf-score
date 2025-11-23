use wf_stats::{
    Modifier,
    Status,
    Weapon,
};

#[derive(Default, Clone, Copy, PartialEq)]
pub struct MoltenImpact;

impl Modifier for MoltenImpact {
    fn status_list(
        &self,
        _context: &dyn Weapon,
    ) -> Vec<Status> {
        vec![Status::heat(0.9)]
    }
}

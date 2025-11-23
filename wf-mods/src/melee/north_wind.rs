use wf_stats::{
    Modifier,
    Status,
    Weapon,
};

#[derive(Default, Clone, Copy, PartialEq)]
pub struct NorthWind;

impl Modifier for NorthWind {
    fn status_list(
        &self,
        _context: &dyn Weapon,
    ) -> Vec<Status> {
        vec![Status::cold(0.9)]
    }
}

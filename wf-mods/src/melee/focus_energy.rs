use wf_stats::{
    Modifier,
    Status,
    Weapon,
};

#[derive(Default, Clone, Copy, PartialEq)]
pub struct FocusEnergy;

impl Modifier for FocusEnergy {
    fn status(
        &self,
        _context: &dyn Weapon,
    ) -> Status {
        Status::new().electricity(0.6)
    }
}

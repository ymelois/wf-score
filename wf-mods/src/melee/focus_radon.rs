use wf_stats::{
    Modifier,
    Status,
    Weapon,
};

#[derive(Default, Clone, Copy, PartialEq)]
pub struct FocusRadon;

impl Modifier for FocusRadon {
    fn status(
        &self,
        _context: &dyn Weapon,
    ) -> Status {
        Status::new().radiation(0.6)
    }
}

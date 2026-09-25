use wf_stats::{
    Modifier,
    Status,
    Weapon,
};

#[derive(Default, Clone, Copy, PartialEq)]
pub struct ShockingTouch;

impl Modifier for ShockingTouch {
    fn status(
        &self,
        _context: &dyn Weapon,
    ) -> Status {
        Status::new().electricity(0.9)
    }
}

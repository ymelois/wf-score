use wf_stats::{
    Modifier,
    Status,
    Weapon,
};

#[derive(Default, Clone, Copy, PartialEq)]
pub struct NorthWind;

impl Modifier for NorthWind {
    fn status(
        &self,
        _context: &dyn Weapon,
    ) -> Status {
        Status::new().cold(0.9)
    }
}

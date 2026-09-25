use wf_stats::{
    Modifier,
    Status,
    Weapon,
};

#[derive(Default, Clone, Copy, PartialEq)]
pub struct FeverStrike;

impl Modifier for FeverStrike {
    fn status(
        &self,
        _context: &dyn Weapon,
    ) -> Status {
        Status::new().toxin(0.9)
    }
}

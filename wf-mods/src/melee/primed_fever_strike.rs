use wf_stats::{
    Modifier,
    Status,
    Weapon,
};

#[derive(Default, Clone, Copy, PartialEq)]
pub struct PrimedFeverStrike;

impl Modifier for PrimedFeverStrike {
    fn status(
        &self,
        _context: &dyn Weapon,
    ) -> Status {
        Status::new().toxin(1.65)
    }
}

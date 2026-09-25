use wf_stats::{
    Modifier,
    Status,
    Weapon,
};

#[derive(Debug, Clone, PartialEq)]
pub struct PrimedConvulsion;

impl Modifier for PrimedConvulsion {
    fn status(
        &self,
        _context: &dyn Weapon,
    ) -> Status {
        Status::new().electricity(1.65)
    }

    fn cost(
        &self,
        _context: &dyn Weapon,
    ) -> u8 {
        16
    }
}

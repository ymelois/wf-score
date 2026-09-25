use wf_stats::{
    Modifier,
    Status,
    Weapon,
};

#[derive(Debug, Clone, PartialEq)]
pub struct PrimedHeatedCharge;

impl Modifier for PrimedHeatedCharge {
    fn status(
        &self,
        _context: &dyn Weapon,
    ) -> Status {
        Status::new().heat(1.65)
    }

    fn cost(
        &self,
        _context: &dyn Weapon,
    ) -> u8 {
        16
    }
}

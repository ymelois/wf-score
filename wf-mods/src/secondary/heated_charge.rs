use wf_stats::{
    Modifier,
    Status,
    Weapon,
};

#[derive(Debug, Clone, PartialEq)]
pub struct HeatedCharge;

impl Modifier for HeatedCharge {
    fn status(
        &self,
        _context: &dyn Weapon,
    ) -> Status {
        Status::new().heat(0.9)
    }

    fn cost(
        &self,
        _context: &dyn Weapon,
    ) -> u8 {
        11
    }
}

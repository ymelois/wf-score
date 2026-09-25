use wf_stats::{
    Modifier,
    Status,
    Weapon,
};

#[derive(Default, Clone, Copy, PartialEq)]
pub struct MoltenImpact;

impl Modifier for MoltenImpact {
    fn status(
        &self,
        _context: &dyn Weapon,
    ) -> Status {
        Status::new().heat(0.9)
    }
}

use wf_stats::{
    Modifier,
    Status,
    Weapon,
};

#[derive(Debug, Clone, PartialEq)]
pub struct PathogenRounds;

impl Modifier for PathogenRounds {
    fn status(
        &self,
        _context: &dyn Weapon,
    ) -> Status {
        Status::new().toxin(0.9)
    }

    fn cost(
        &self,
        _context: &dyn Weapon,
    ) -> u8 {
        11
    }
}

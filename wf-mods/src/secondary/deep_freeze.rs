use wf_stats::{
    Modifier,
    Status,
    Weapon,
};

#[derive(Debug, Clone, PartialEq)]
pub struct DeepFreeze;

impl Modifier for DeepFreeze {
    fn status(
        &self,
        _context: &dyn Weapon,
    ) -> Status {
        Status::new().cold(0.9)
    }

    fn cost(
        &self,
        _context: &dyn Weapon,
    ) -> u8 {
        11
    }
}

use wf_stats::{
    Modifier,
    StatusesImpl as _,
    Weapon,
};

#[derive(Debug, Clone, PartialEq)]
pub struct GalvanizedShot {
    pub threshold: f32,
}

impl Modifier for GalvanizedShot {
    fn damage(
        &self,
        context: &dyn Weapon,
    ) -> f32 {
        let status_list = context.status_list();
        let total_damage = status_list.damage();

        let mut status_count: u8 = 0;
        for status in status_list {
            if status.damage() / total_damage > self.threshold {
                status_count += 1;
            }
        }

        0.4 * f32::from(status_count.min(3))
    }

    fn status_chance(
        &self,
        _context: &dyn Weapon,
    ) -> f32 {
        0.8
    }

    fn cost(
        &self,
        _context: &dyn Weapon,
    ) -> u8 {
        12
    }
}

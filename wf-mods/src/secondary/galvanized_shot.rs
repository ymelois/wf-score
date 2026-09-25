use wf_stats::{
    Modifier,
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
        let status = context.status();
        let total_damage = status.sum();

        let mut status_count: u8 = 0;

        if status.impact / total_damage > self.threshold {
            status_count += 1;
        }
        if status.puncture / total_damage > self.threshold {
            status_count += 1;
        }
        if status.slash / total_damage > self.threshold {
            status_count += 1;
        }
        if status.cold / total_damage > self.threshold {
            status_count += 1;
        }
        if status.electricity / total_damage > self.threshold {
            status_count += 1;
        }
        if status.heat / total_damage > self.threshold {
            status_count += 1;
        }
        if status.toxin / total_damage > self.threshold {
            status_count += 1;
        }
        if status.blast / total_damage > self.threshold {
            status_count += 1;
        }
        if status.corrosive / total_damage > self.threshold {
            status_count += 1;
        }
        if status.gas / total_damage > self.threshold {
            status_count += 1;
        }
        if status.magnetic / total_damage > self.threshold {
            status_count += 1;
        }
        if status.radiation / total_damage > self.threshold {
            status_count += 1;
        }
        if status.viral / total_damage > self.threshold {
            status_count += 1;
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

use wf_stats::{
    Modifier,
    Weapon,
};

#[derive(Default, Clone, Copy, PartialEq)]
pub struct ConditionOverload {
    /// The % of the total damage that a status must have to count as a
    /// condition
    pub threshold: f32,
}

impl Modifier for ConditionOverload {
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

        0.8 * f32::from(status_count)
    }
}

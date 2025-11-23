mod physical;
mod primary_elemental;
mod secondary_elemental;

use std::collections::{
    HashSet,
    VecDeque,
};

pub use physical::Physical;
pub use primary_elemental::PrimaryElemental;
pub use secondary_elemental::SecondaryElemental;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Status {
    Physical(Physical),
    PrimaryElemental(PrimaryElemental),
    SecondaryElemental(SecondaryElemental),
}

impl Status {
    #[must_use]
    pub fn damage(&self) -> f32 {
        match self {
            Self::Physical(physical) => physical.damage(),
            Self::PrimaryElemental(elemental) => elemental.damage(),
            Self::SecondaryElemental(secondary) => secondary.damage(),
        }
    }

    pub fn set_damage(
        &mut self,
        damage: f32,
    ) {
        match self {
            Self::Physical(physical) => physical.set_damage(damage),
            Self::PrimaryElemental(elemental) => elemental.set_damage(damage),
            Self::SecondaryElemental(secondary) => secondary.set_damage(damage),
        }
    }

    #[must_use]
    pub const fn impact(impact: f32) -> Self { Self::Physical(Physical::Impact(impact)) }

    #[must_use]
    pub const fn puncture(puncture: f32) -> Self { Self::Physical(Physical::Puncture(puncture)) }

    #[must_use]
    pub const fn slash(slash: f32) -> Self { Self::Physical(Physical::Slash(slash)) }

    #[must_use]
    pub const fn cold(cold: f32) -> Self { Self::PrimaryElemental(PrimaryElemental::Cold(cold)) }

    #[must_use]
    pub const fn electricity(electricity: f32) -> Self {
        Self::PrimaryElemental(PrimaryElemental::Electricity(electricity))
    }

    #[must_use]
    pub const fn heat(heat: f32) -> Self { Self::PrimaryElemental(PrimaryElemental::Heat(heat)) }

    #[must_use]
    pub const fn toxin(toxin: f32) -> Self {
        Self::PrimaryElemental(PrimaryElemental::Toxin(toxin))
    }

    #[must_use]
    pub const fn blast(blast: f32) -> Self {
        Self::SecondaryElemental(SecondaryElemental::Blast(blast))
    }

    #[must_use]
    pub const fn corrosive(corrosive: f32) -> Self {
        Self::SecondaryElemental(SecondaryElemental::Corrosive(corrosive))
    }

    #[must_use]
    pub const fn gas(gas: f32) -> Self { Self::SecondaryElemental(SecondaryElemental::Gas(gas)) }

    #[must_use]
    pub const fn magnetic(magnetic: f32) -> Self {
        Self::SecondaryElemental(SecondaryElemental::Magnetic(magnetic))
    }

    #[must_use]
    pub const fn radiation(radiation: f32) -> Self {
        Self::SecondaryElemental(SecondaryElemental::Radiation(radiation))
    }

    #[must_use]
    pub const fn viral(viral: f32) -> Self {
        Self::SecondaryElemental(SecondaryElemental::Viral(viral))
    }
}

pub trait StatusesImpl {
    /// Merge similar statuses into one and elementals into secondary if
    /// possible.
    ///
    /// Cold + Electricity = Magnetic
    /// Cold + Heat = Blast
    /// Cold + Toxin = Viral
    /// Electricity + Heat = Radiation
    /// Electricity + Toxin = Corrosive
    /// Heat + Toxin = Gas
    fn merge(&self) -> Vec<Status>;

    /// Calculate the total damage of all statuses.
    fn damage(&self) -> f32;

    fn physical(&self) -> Vec<Status>;
    fn elemental(&self) -> Vec<Status>;
    fn secondary(&self) -> Vec<Status>;

    fn impact(&self) -> Option<Status>;
    fn puncture(&self) -> Option<Status>;
    fn slash(&self) -> Option<Status>;
    fn cold(&self) -> Option<Status>;
    fn electricity(&self) -> Option<Status>;
    fn heat(&self) -> Option<Status>;
    fn toxin(&self) -> Option<Status>;
    fn blast(&self) -> Option<Status>;
    fn corrosive(&self) -> Option<Status>;
    fn gas(&self) -> Option<Status>;
    fn magnetic(&self) -> Option<Status>;
    fn radiation(&self) -> Option<Status>;
    fn viral(&self) -> Option<Status>;
}

fn deduplicate_statuses(mut statuses: Vec<Status>) -> Vec<Status> {
    let mut merged_statuses = VecDeque::with_capacity(statuses.len());

    'pop: while let Some(status) = statuses.pop() {
        for status_mut in statuses.iter_mut().rev() {
            if status_mut == &status {
                status_mut.set_damage(status_mut.damage() + status.damage());
                continue 'pop;
            }
        }

        merged_statuses.push_front(status);
    }

    merged_statuses.into()
}

fn combine_elemental_statuses(statuses: &[Status]) -> Vec<Status> {
    let mut visited = HashSet::new();
    let mut merged_statuses = Vec::with_capacity(statuses.len());

    'outer: for (index, &status) in statuses.iter().enumerate() {
        if visited.contains(&index) {
            continue;
        }
        visited.insert(index);

        if !matches!(status, Status::PrimaryElemental(_)) {
            merged_statuses.push(status);
            continue;
        }

        let mut checked = Vec::new();

        let status_2 = 'inner: {
            for (index_2, status_2) in statuses.iter().enumerate().skip(index + 1) {
                visited.insert(index_2);

                if matches!(status_2, Status::PrimaryElemental(_)) {
                    break 'inner *status_2;
                }

                checked.push(*status_2);
            }

            merged_statuses.push(status);
            merged_statuses.append(&mut checked);

            break 'outer;
        };

        let mut status = status;

        if let Status::PrimaryElemental(elemental) = status
            && let Status::PrimaryElemental(elemental_2) = status_2
        {
            let new_element = match elemental {
                PrimaryElemental::Cold(_) => match elemental_2 {
                    PrimaryElemental::Cold(_) => Status::cold,
                    PrimaryElemental::Electricity(_) => Status::magnetic,
                    PrimaryElemental::Heat(_) => Status::blast,
                    PrimaryElemental::Toxin(_) => Status::viral,
                },
                PrimaryElemental::Electricity(_) => match elemental_2 {
                    PrimaryElemental::Cold(_) => Status::magnetic,
                    PrimaryElemental::Electricity(_) => Status::electricity,
                    PrimaryElemental::Heat(_) => Status::radiation,
                    PrimaryElemental::Toxin(_) => Status::corrosive,
                },
                PrimaryElemental::Heat(_) => match elemental_2 {
                    PrimaryElemental::Cold(_) => Status::blast,
                    PrimaryElemental::Electricity(_) => Status::radiation,
                    PrimaryElemental::Heat(_) => Status::heat,
                    PrimaryElemental::Toxin(_) => Status::gas,
                },
                PrimaryElemental::Toxin(_) => match elemental_2 {
                    PrimaryElemental::Cold(_) => Status::viral,
                    PrimaryElemental::Electricity(_) => Status::corrosive,
                    PrimaryElemental::Heat(_) => Status::gas,
                    PrimaryElemental::Toxin(_) => Status::toxin,
                },
            };

            status = new_element(status.damage() + status_2.damage());
        }

        merged_statuses.push(status);
        merged_statuses.append(&mut checked);
    }

    merged_statuses
}

impl StatusesImpl for Vec<Status> {
    fn merge(&self) -> Vec<Status> {
        let statuses = deduplicate_statuses(self.clone());
        let statuses = combine_elemental_statuses(&statuses);
        deduplicate_statuses(statuses)
    }

    fn damage(&self) -> f32 { self.iter().map(Status::damage).sum::<f32>() }

    fn physical(&self) -> Vec<Status> {
        let mut physicals = Vec::new();
        for status in self {
            if let Status::Physical(_) = status {
                physicals.push(*status);
            }
        }

        physicals
    }

    fn elemental(&self) -> Vec<Status> {
        let mut elementals = Vec::new();
        for status in self {
            if let Status::PrimaryElemental(_) = status {
                elementals.push(*status);
            }
        }

        elementals
    }

    fn secondary(&self) -> Vec<Status> {
        let mut secondaries = Vec::new();
        for status in self {
            if let Status::SecondaryElemental(_) = status {
                secondaries.push(*status);
            }
        }

        secondaries
    }

    fn impact(&self) -> Option<Status> {
        for status in self {
            if let Status::Physical(Physical::Impact(_)) = status {
                return Some(*status);
            }
        }

        None
    }

    fn puncture(&self) -> Option<Status> {
        for status in self {
            if let Status::Physical(Physical::Puncture(_)) = status {
                return Some(*status);
            }
        }

        None
    }

    fn slash(&self) -> Option<Status> {
        for status in self {
            if let Status::Physical(Physical::Slash(_)) = status {
                return Some(*status);
            }
        }

        None
    }

    fn cold(&self) -> Option<Status> {
        for status in self {
            if let Status::PrimaryElemental(PrimaryElemental::Cold(_)) = status {
                return Some(*status);
            }
        }

        None
    }

    fn electricity(&self) -> Option<Status> {
        for status in self {
            if let Status::PrimaryElemental(PrimaryElemental::Electricity(_)) = status {
                return Some(*status);
            }
        }

        None
    }

    fn heat(&self) -> Option<Status> {
        for status in self {
            if let Status::PrimaryElemental(PrimaryElemental::Heat(_)) = status {
                return Some(*status);
            }
        }

        None
    }

    fn toxin(&self) -> Option<Status> {
        for status in self {
            if let Status::PrimaryElemental(PrimaryElemental::Toxin(_)) = status {
                return Some(*status);
            }
        }

        None
    }

    fn blast(&self) -> Option<Status> {
        for status in self {
            if let Status::SecondaryElemental(SecondaryElemental::Blast(_)) = status {
                return Some(*status);
            }
        }

        None
    }

    fn corrosive(&self) -> Option<Status> {
        for status in self {
            if let Status::SecondaryElemental(SecondaryElemental::Corrosive(_)) = status {
                return Some(*status);
            }
        }

        None
    }

    fn gas(&self) -> Option<Status> {
        for status in self {
            if let Status::SecondaryElemental(SecondaryElemental::Gas(_)) = status {
                return Some(*status);
            }
        }

        None
    }

    fn magnetic(&self) -> Option<Status> {
        for status in self {
            if let Status::SecondaryElemental(SecondaryElemental::Magnetic(_)) = status {
                return Some(*status);
            }
        }

        None
    }

    fn radiation(&self) -> Option<Status> {
        for status in self {
            if let Status::SecondaryElemental(SecondaryElemental::Radiation(_)) = status {
                return Some(*status);
            }
        }

        None
    }

    fn viral(&self) -> Option<Status> {
        for status in self {
            if let Status::SecondaryElemental(SecondaryElemental::Viral(_)) = status {
                return Some(*status);
            }
        }

        None
    }
}

impl std::ops::Add<f32> for Status {
    type Output = Status;

    fn add(
        mut self,
        other: f32,
    ) -> Status {
        self.set_damage(self.damage() + other);
        self
    }
}

impl std::ops::AddAssign<f32> for Status {
    fn add_assign(
        &mut self,
        other: f32,
    ) {
        self.set_damage(self.damage() + other);
    }
}

impl std::ops::Mul<f32> for Status {
    type Output = Status;

    fn mul(
        mut self,
        other: f32,
    ) -> Status {
        self.set_damage(self.damage() * other);
        self
    }
}

impl std::ops::MulAssign<f32> for Status {
    fn mul_assign(
        &mut self,
        other: f32,
    ) {
        self.set_damage(self.damage() * other);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_deduplicate_statuses() {
        let statuses = vec![Status::impact(1.0), Status::impact(1.0)];
        assert_eq!(deduplicate_statuses(statuses), vec![Status::impact(2.0)]);

        let statuses = vec![Status::impact(1.0), Status::slash(1.0), Status::impact(1.0)];
        assert_eq!(deduplicate_statuses(statuses), vec![
            Status::impact(2.0),
            Status::slash(1.0),
        ]);

        let statuses = vec![
            Status::impact(1.0),
            Status::slash(1.0),
            Status::impact(1.0),
            Status::slash(1.0),
        ];
        assert_eq!(deduplicate_statuses(statuses), vec![
            Status::impact(2.0),
            Status::slash(2.0),
        ]);
    }

    #[test]
    fn test_combine_elemental_statuses() {
        let statuses = vec![Status::impact(1.0), Status::cold(1.0), Status::heat(1.0)];
        assert_eq!(combine_elemental_statuses(&statuses), vec![
            Status::impact(1.0),
            Status::blast(2.0),
        ]);

        let statuses = vec![Status::cold(1.0), Status::heat(1.0), Status::impact(1.0)];
        assert_eq!(combine_elemental_statuses(&statuses), vec![
            Status::blast(2.0),
            Status::impact(1.0),
        ]);

        let statuses = vec![Status::cold(1.0), Status::impact(1.0), Status::slash(1.0)];
        assert_eq!(combine_elemental_statuses(&statuses), vec![
            Status::cold(1.0),
            Status::impact(1.0),
            Status::slash(1.0),
        ]);
    }
}

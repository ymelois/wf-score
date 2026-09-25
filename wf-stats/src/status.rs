use core::{
    fmt,
    ops,
};

#[derive(Clone, PartialEq)]
pub struct Status {
    pub impact: f32,
    pub puncture: f32,
    pub slash: f32,
    pub cold: f32,
    pub electricity: f32,
    pub heat: f32,
    pub toxin: f32,
    convert_cold: Option<ConvertColdStatus>,
    convert_electricity: Option<ConvertElectricityStatus>,
    convert_heat: Option<ConvertHeatStatus>,
    convert_toxin: Option<ConvertToxinStatus>,
    pub blast: f32,
    pub corrosive: f32,
    pub gas: f32,
    pub magnetic: f32,
    pub radiation: f32,
    pub viral: f32,
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum ConvertColdStatus {
    Blast,
    Magnetic,
    Viral,
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum ConvertElectricityStatus {
    Corrosive,
    Magnetic,
    Radiation,
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum ConvertHeatStatus {
    Blast,
    Gas,
    Radiation,
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum ConvertToxinStatus {
    Corrosive,
    Gas,
    Viral,
}

impl fmt::Debug for Status {
    fn fmt(
        &self,
        f: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        let mut f = f.debug_struct("Status");
        if self.impact > 0.0 {
            f.field("impact", &self.impact);
        }
        if self.puncture > 0.0 {
            f.field("puncture", &self.puncture);
        }
        if self.slash > 0.0 {
            f.field("slash", &self.slash);
        }
        if self.cold > 0.0 {
            f.field("cold", &self.cold);
        }
        if self.electricity > 0.0 {
            f.field("electricity", &self.electricity);
        }
        if self.heat > 0.0 {
            f.field("heat", &self.heat);
        }
        if self.toxin > 0.0 {
            f.field("toxin", &self.toxin);
        }
        if self.blast > 0.0 {
            f.field("blast", &self.blast);
        }
        if self.corrosive > 0.0 {
            f.field("corrosive", &self.corrosive);
        }
        if self.gas > 0.0 {
            f.field("gas", &self.gas);
        }
        if self.magnetic > 0.0 {
            f.field("magnetic", &self.magnetic);
        }
        if self.radiation > 0.0 {
            f.field("radiation", &self.radiation);
        }
        if self.viral > 0.0 {
            f.field("viral", &self.viral);
        }
        f.finish_non_exhaustive()
    }
}

impl Default for Status {
    fn default() -> Self { Self::new() }
}

impl Status {
    #[must_use]
    pub const fn new() -> Self {
        Self {
            impact: 0.0,
            puncture: 0.0,
            slash: 0.0,
            cold: 0.0,
            electricity: 0.0,
            heat: 0.0,
            toxin: 0.0,
            convert_cold: None,
            convert_electricity: None,
            convert_heat: None,
            convert_toxin: None,
            blast: 0.0,
            corrosive: 0.0,
            gas: 0.0,
            magnetic: 0.0,
            radiation: 0.0,
            viral: 0.0,
        }
    }

    #[must_use]
    pub const fn sum(&self) -> f32 {
        self.impact
            + self.puncture
            + self.slash
            + self.cold
            + self.electricity
            + self.heat
            + self.toxin
            + self.blast
            + self.corrosive
            + self.gas
            + self.magnetic
            + self.radiation
            + self.viral
    }

    pub const fn apply(
        &mut self,
        other: &Self,
        base: &Self,
    ) -> &mut Self {
        let base_damage = base.sum();

        if other.impact > 0.0 {
            self.impact += base.impact * other.impact;
        }
        if other.puncture > 0.0 {
            self.puncture += base.puncture * other.puncture;
        }
        if other.slash > 0.0 {
            self.slash += base.slash * other.slash;
        }
        if other.cold > 0.0 {
            self.add_cold(other.cold * base_damage);
        }
        if other.electricity > 0.0 {
            self.add_electricity(other.electricity * base_damage);
        }
        if other.heat > 0.0 {
            self.add_heat(other.heat * base_damage);
        }
        if other.toxin > 0.0 {
            self.add_toxin(other.toxin * base_damage);
        }
        if other.blast > 0.0 {
            self.add_blast(other.blast * base_damage);
        }
        if other.corrosive > 0.0 {
            self.add_corrosive(other.corrosive * base_damage);
        }
        if other.gas > 0.0 {
            self.add_gas(other.gas * base_damage);
        }
        if other.magnetic > 0.0 {
            self.add_magnetic(other.magnetic * base_damage);
        }
        if other.radiation > 0.0 {
            self.add_radiation(other.radiation * base_damage);
        }
        if other.viral > 0.0 {
            self.add_viral(other.viral * base_damage);
        }

        self
    }

    #[must_use]
    pub const fn impact(
        mut self,
        value: f32,
    ) -> Self {
        self.impact = value;
        self
    }

    #[must_use]
    pub const fn puncture(
        mut self,
        value: f32,
    ) -> Self {
        self.puncture = value;
        self
    }

    #[must_use]
    pub const fn slash(
        mut self,
        value: f32,
    ) -> Self {
        self.slash = value;
        self
    }

    #[must_use]
    pub const fn cold(
        mut self,
        value: f32,
    ) -> Self {
        self.cold = value;
        self
    }

    #[must_use]
    pub const fn electricity(
        mut self,
        value: f32,
    ) -> Self {
        self.electricity = value;
        self
    }

    #[must_use]
    pub const fn heat(
        mut self,
        value: f32,
    ) -> Self {
        self.heat = value;
        self
    }

    #[must_use]
    pub const fn toxin(
        mut self,
        value: f32,
    ) -> Self {
        self.toxin = value;
        self
    }

    #[must_use]
    pub const fn blast(
        mut self,
        value: f32,
    ) -> Self {
        self.blast = value;
        self
    }

    #[must_use]
    pub const fn corrosive(
        mut self,
        value: f32,
    ) -> Self {
        self.corrosive = value;
        self
    }

    #[must_use]
    pub const fn gas(
        mut self,
        value: f32,
    ) -> Self {
        self.gas = value;
        self
    }

    #[must_use]
    pub const fn magnetic(
        mut self,
        value: f32,
    ) -> Self {
        self.magnetic = value;
        self
    }

    #[must_use]
    pub const fn radiation(
        mut self,
        value: f32,
    ) -> Self {
        self.radiation = value;
        self
    }

    #[must_use]
    pub const fn viral(
        mut self,
        value: f32,
    ) -> Self {
        self.viral = value;
        self
    }

    pub const fn add_impact(
        &mut self,
        bonus: f32,
    ) -> &mut Self {
        self.impact += bonus;
        self
    }

    pub const fn add_puncture(
        &mut self,
        bonus: f32,
    ) -> &mut Self {
        self.puncture += bonus;
        self
    }

    pub const fn add_slash(
        &mut self,
        bonus: f32,
    ) -> &mut Self {
        self.slash += bonus;
        self
    }

    pub const fn add_cold(
        &mut self,
        bonus: f32,
    ) -> &mut Self {
        match self.convert_cold {
            None => {
                self.cold += bonus;
                if self.cold <= 0.0 {
                    return self;
                }

                if self.convert_electricity.is_none() && self.electricity > 0.0 {
                    self.magnetic += self.cold + self.electricity;
                    self.cold = 0.0;
                    self.electricity = 0.0;
                    self.convert_cold = Some(ConvertColdStatus::Magnetic);
                    self.convert_electricity = Some(ConvertElectricityStatus::Magnetic);
                } else if self.convert_heat.is_none() && self.heat > 0.0 {
                    self.blast += self.cold + self.heat;
                    self.cold = 0.0;
                    self.heat = 0.0;
                    self.convert_cold = Some(ConvertColdStatus::Blast);
                    self.convert_heat = Some(ConvertHeatStatus::Blast);
                } else if self.convert_toxin.is_none() && self.toxin > 0.0 {
                    self.viral += self.cold + self.toxin;
                    self.cold = 0.0;
                    self.toxin = 0.0;
                    self.convert_cold = Some(ConvertColdStatus::Viral);
                    self.convert_toxin = Some(ConvertToxinStatus::Viral);
                }

                self
            }
            Some(ConvertColdStatus::Blast) => self.add_blast(bonus),
            Some(ConvertColdStatus::Magnetic) => self.add_magnetic(bonus),
            Some(ConvertColdStatus::Viral) => self.add_viral(bonus),
        }
    }

    pub const fn add_electricity(
        &mut self,
        bonus: f32,
    ) -> &mut Self {
        match self.convert_electricity {
            None => {
                self.electricity += bonus;
                if self.electricity <= 0.0 {
                    return self;
                }

                if self.convert_cold.is_none() && self.cold > 0.0 {
                    self.magnetic += self.electricity + self.cold;
                    self.electricity = 0.0;
                    self.cold = 0.0;
                    self.convert_electricity = Some(ConvertElectricityStatus::Magnetic);
                    self.convert_cold = Some(ConvertColdStatus::Magnetic);
                } else if self.convert_heat.is_none() && self.heat > 0.0 {
                    self.radiation += self.electricity + self.heat;
                    self.electricity = 0.0;
                    self.heat = 0.0;
                    self.convert_electricity = Some(ConvertElectricityStatus::Radiation);
                    self.convert_heat = Some(ConvertHeatStatus::Radiation);
                } else if self.convert_toxin.is_none() && self.toxin > 0.0 {
                    self.corrosive += self.electricity + self.toxin;
                    self.electricity = 0.0;
                    self.toxin = 0.0;
                    self.convert_electricity = Some(ConvertElectricityStatus::Corrosive);
                    self.convert_toxin = Some(ConvertToxinStatus::Corrosive);
                }

                self
            }
            Some(ConvertElectricityStatus::Corrosive) => self.add_corrosive(bonus),
            Some(ConvertElectricityStatus::Magnetic) => self.add_magnetic(bonus),
            Some(ConvertElectricityStatus::Radiation) => self.add_radiation(bonus),
        }
    }

    pub const fn add_heat(
        &mut self,
        bonus: f32,
    ) -> &mut Self {
        match self.convert_heat {
            None => {
                self.heat += bonus;
                if self.heat <= 0.0 {
                    return self;
                }

                if self.convert_cold.is_none() && self.cold > 0.0 {
                    self.blast += self.heat + self.cold;
                    self.heat = 0.0;
                    self.cold = 0.0;
                    self.convert_heat = Some(ConvertHeatStatus::Blast);
                    self.convert_cold = Some(ConvertColdStatus::Blast);
                } else if self.convert_electricity.is_none() && self.electricity > 0.0 {
                    self.radiation += self.heat + self.electricity;
                    self.heat = 0.0;
                    self.electricity = 0.0;
                    self.convert_heat = Some(ConvertHeatStatus::Radiation);
                    self.convert_electricity = Some(ConvertElectricityStatus::Radiation);
                } else if self.convert_toxin.is_none() && self.toxin > 0.0 {
                    self.gas += self.heat + self.toxin;
                    self.heat = 0.0;
                    self.toxin = 0.0;
                    self.convert_heat = Some(ConvertHeatStatus::Gas);
                    self.convert_toxin = Some(ConvertToxinStatus::Gas);
                }

                self
            }
            Some(ConvertHeatStatus::Blast) => self.add_blast(bonus),
            Some(ConvertHeatStatus::Gas) => self.add_gas(bonus),
            Some(ConvertHeatStatus::Radiation) => self.add_radiation(bonus),
        }
    }

    pub const fn add_toxin(
        &mut self,
        bonus: f32,
    ) -> &mut Self {
        match self.convert_toxin {
            None => {
                self.toxin += bonus;
                if self.toxin <= 0.0 {
                    return self;
                }

                if self.convert_cold.is_none() && self.cold > 0.0 {
                    self.viral += self.toxin + self.cold;
                    self.toxin = 0.0;
                    self.cold = 0.0;
                    self.convert_toxin = Some(ConvertToxinStatus::Viral);
                    self.convert_cold = Some(ConvertColdStatus::Viral);
                } else if self.convert_electricity.is_none() && self.electricity > 0.0 {
                    self.corrosive += self.toxin + self.electricity;
                    self.toxin = 0.0;
                    self.electricity = 0.0;
                    self.convert_toxin = Some(ConvertToxinStatus::Corrosive);
                    self.convert_electricity = Some(ConvertElectricityStatus::Corrosive);
                } else if self.convert_heat.is_none() && self.heat > 0.0 {
                    self.gas += self.toxin + self.heat;
                    self.toxin = 0.0;
                    self.heat = 0.0;
                    self.convert_toxin = Some(ConvertToxinStatus::Gas);
                    self.convert_heat = Some(ConvertHeatStatus::Gas);
                }

                self
            }
            Some(ConvertToxinStatus::Corrosive) => self.add_corrosive(bonus),
            Some(ConvertToxinStatus::Gas) => self.add_gas(bonus),
            Some(ConvertToxinStatus::Viral) => self.add_viral(bonus),
        }
    }

    pub const fn add_blast(
        &mut self,
        bonus: f32,
    ) -> &mut Self {
        self.blast += bonus;
        self
    }

    pub const fn add_corrosive(
        &mut self,
        bonus: f32,
    ) -> &mut Self {
        self.corrosive += bonus;
        self
    }

    pub const fn add_gas(
        &mut self,
        bonus: f32,
    ) -> &mut Self {
        self.gas += bonus;
        self
    }

    pub const fn add_magnetic(
        &mut self,
        bonus: f32,
    ) -> &mut Self {
        self.magnetic += bonus;
        self
    }

    pub const fn add_radiation(
        &mut self,
        bonus: f32,
    ) -> &mut Self {
        self.radiation += bonus;
        self
    }

    pub const fn add_viral(
        &mut self,
        bonus: f32,
    ) -> &mut Self {
        self.viral += bonus;
        self
    }
}

impl ops::AddAssign<&Status> for Status {
    fn add_assign(
        &mut self,
        rhs: &Status,
    ) {
        self.add_impact(rhs.impact)
            .add_puncture(rhs.puncture)
            .add_slash(rhs.slash)
            .add_cold(rhs.cold)
            .add_electricity(rhs.electricity)
            .add_heat(rhs.heat)
            .add_toxin(rhs.toxin)
            .add_blast(rhs.blast)
            .add_corrosive(rhs.corrosive)
            .add_gas(rhs.gas)
            .add_magnetic(rhs.magnetic)
            .add_radiation(rhs.radiation)
            .add_viral(rhs.viral);
    }
}

impl ops::Mul<f32> for Status {
    type Output = Status;

    fn mul(
        mut self,
        rhs: f32,
    ) -> Self::Output {
        self.impact *= rhs;
        self.puncture *= rhs;
        self.slash *= rhs;
        self.cold *= rhs;
        self.electricity *= rhs;
        self.heat *= rhs;
        self.toxin *= rhs;
        self.blast *= rhs;
        self.corrosive *= rhs;
        self.gas *= rhs;
        self.magnetic *= rhs;
        self.radiation *= rhs;
        self.viral *= rhs;
        self
    }
}

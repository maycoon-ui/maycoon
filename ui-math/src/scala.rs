use std::cmp::Ordering;
use std::ops::{Add, Div, Mul, Sub};

#[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
pub enum Scala {
    Fixed(f32),
    Percent(f32),
}

impl Scala {
    pub fn new(val: f32) -> Self {
        Self::Fixed(val)
    }

    pub fn percent(val: f32) -> Self {
        Self::Percent(val)
    }

    pub fn as_fixed(&self) -> Option<f32> {
        match self {
            Self::Fixed(val) => Some(*val),
            _ => None,
        }
    }
}

impl Default for Scala {
    fn default() -> Self {
        Self::Fixed(1.0)
    }
}

impl From<f32> for Scala {
    fn from(val: f32) -> Self {
        Self::Fixed(val)
    }
}

impl From<f64> for Scala {
    fn from(val: f64) -> Self {
        Self::Fixed(val as f32)
    }
}

impl Add for Scala {
    type Output = Scala;

    fn add(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Scala::Fixed(l), Scala::Fixed(r)) => Scala::Fixed(l + r),
            (Scala::Percent(l), Scala::Percent(r)) => Scala::Percent(l + r),
            _ => panic!("Cannot perform addition on `Scala::Fixed` and `Scala::Percent`"),
        }
    }
}

impl Sub for Scala {
    type Output = Scala;

    fn sub(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Scala::Fixed(l), Scala::Fixed(r)) => Scala::Fixed(l - r),
            (Scala::Percent(l), Scala::Percent(r)) => Scala::Percent(l - r),
            _ => panic!("Only `Scala::Fixed` can be subtracted from `Scala::Fixed`"),
        }
    }
}

impl Mul for Scala {
    type Output = Scala;

    fn mul(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Scala::Fixed(l), Scala::Fixed(r)) => Scala::Fixed(l * r),
            (Scala::Percent(l), Scala::Percent(r)) => Scala::Percent(l * r),
            _ => panic!("Only `Scala::Fixed` can be multiplied by `Scala::Fixed`"),
        }
    }
}

impl Div for Scala {
    type Output = Scala;

    fn div(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Scala::Fixed(l), Scala::Fixed(r)) => Scala::Fixed(l / r),
            (Scala::Percent(l), Scala::Percent(r)) => Scala::Percent(l / r),
            _ => panic!("Only `Scala::Fixed` can be divided by `Scala::Fixed`"),
        }
    }
}

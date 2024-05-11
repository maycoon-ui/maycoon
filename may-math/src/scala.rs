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

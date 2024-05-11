use crate::scala::Scala;

#[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
pub struct Point {
    pub x: Scala,
    pub y: Scala,
}

impl Point {
    pub fn new(x: impl Into<Scala>, y: impl Into<Scala>) -> Self {
        Self {
            x: x.into(),
            y: y.into(),
        }
    }
}

impl From<(Scala, Scala)> for Point {
    fn from((x, y): (Scala, Scala)) -> Self {
        Self { x, y }
    }
}

impl From<(f32, f32)> for Point {
    fn from((x, y): (f32, f32)) -> Self {
        Self {
            x: x.into(),
            y: y.into(),
        }
    }
}

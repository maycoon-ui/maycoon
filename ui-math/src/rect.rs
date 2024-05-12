use crate::point::Point;
use crate::scala::Scala;

#[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
pub struct Rect {
    pub x: Scala,
    pub y: Scala,
    pub width: Scala,
    pub height: Scala,
}

impl Rect {
    pub fn new<T: Into<Scala>>(x: T, y: T, width: T, height: T) -> Self {
        Self {
            x: x.into(),
            y: y.into(),
            width: width.into(),
            height: height.into(),
        }
    }

    pub fn contains(&self, point: &Point) -> bool {
        self.x <= point.x
            && self.y <= point.y
            && self.x + self.width <= point.x
            && self.y + self.height <= point.y
    }
}

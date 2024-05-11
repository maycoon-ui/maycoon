use crate::scala::Scala;

#[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
pub struct Rect {
    pub x: Scala,
    pub y: Scala,
    pub width: Scala,
    pub height: Scala,
}

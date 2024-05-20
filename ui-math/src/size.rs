use crate::scala::Scala;

#[derive(Copy, Clone, Debug, PartialEq, PartialOrd, Default)]
pub struct Size {
    pub width: Scala,
    pub height: Scala,
}

impl Size {
    pub fn new<T: Into<Scala>>(width: T, height: T) -> Self {
        Self {
            width: width.into(),
            height: height.into(),
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct UpdateMode {
    pub(crate) layout: bool,
    pub(crate) draw: bool,
    pub(crate) force: bool,
    pub(crate) eval: bool,
}

impl UpdateMode {
    pub fn full() -> Self {
        Self {
            layout: true,
            draw: true,
            force: false,
            eval: false,
        }
    }

    pub fn none() -> Self {
        Self {
            layout: false,
            draw: false,
            force: false,
            eval: false,
        }
    }

    pub fn update_draw(&mut self) {
        self.draw = true;
    }

    pub fn update_force(&mut self) {
        self.force = true;
    }

    pub fn update_eval(&mut self) {
        self.eval = true;
    }

    pub fn update_layout(&mut self) {
        self.layout = true;
    }
}

use bitflags::bitflags;

bitflags! {
    /// Update bitflags to define which part of the App should Update.
    ///
    /// Possible values:
    /// - **EVAL** - Re-evaluate the widget tree.
    /// - **DRAW** - Re-draw the widget tree.
    /// - **LAYOUT** - Re-layout the widget tree.
    /// - **FORCE** - Force the App to re-evaluate, re-draw and re-layout the widget tree.
    #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct Update: u8 {
        /// Re-evaluate the widget tree.
        const EVAL   = 0b00000001;
        /// Re-draw the widget tree.
        const DRAW   = 0b00000010;
        /// Re-layout the widget tree.
        const LAYOUT = 0b00000100;
        /// Force the App to re-evaluate, re-draw and re-layout the widget tree.
        const FORCE  = 0b00001000;
    }
}

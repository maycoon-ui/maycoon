use femtovg::{Paint, Path};

pub enum RenderCommand {
    None,
    Fill {
        path: Path,
        paint: Paint,
    },
    FillText {
        text: String,
        paint: Paint,
        x: f32,
        y: f32,
    },
}

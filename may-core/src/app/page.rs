use crate::app::context::AppContext;
use crate::widget::Widget;

pub trait Page {
    fn init(&self, ctx: &mut AppContext);
    fn render(&self, ctx: &mut AppContext) -> Box<dyn Widget>;
}

use crate::app::context::AppContext;
use crate::app::page::Page;
use crate::widget::interaction::{InteractionInfo};
use crate::widget::{PathMode, Sketch, Widget};
use femtovg::renderer::OpenGl;
use femtovg::Canvas;
use taffy::{Dimension, NodeId, Size, Style, TaffyTree};
use may_theme::theme::Theme;

pub fn render_sketches(sketches: Vec<Sketch>, canvas: &mut Canvas<OpenGl>) {
    for sketch in sketches {
        match sketch {
            Sketch::Path(path, paint, mode) => match mode {
                PathMode::Fill => canvas.fill_path(&path, &paint),
                PathMode::Stroke => canvas.stroke_path(&path, &paint),
            },

            Sketch::Text(text, pos, paint, mode) => {
                match mode {
                    PathMode::Fill => canvas.fill_text(pos.x, pos.y, &text, &paint),
                    PathMode::Stroke => canvas.stroke_text(pos.x, pos.y, &text, &paint),
                }
                .expect("Failed to render text");
            }
        }
    }
}
pub fn draw_root_widget(
    widget: &mut Box<dyn Widget>,
    size: (f32, f32),
    taffy: &mut TaffyTree,
    info: &mut InteractionInfo,
    antialiasing: bool,
    theme: &Box<dyn Theme>,
) -> Vec<Sketch> {
    let mut sketches: Vec<Sketch> = Vec::with_capacity(16);

    let window = taffy
        .new_leaf(Style {
            size: Size::<Dimension> {
                width: Dimension::Length(size.0),
                height: Dimension::Length(size.1),
            },
            ..Style::default()
        })
        .expect("Failed to create window layout");

    layout_widget(&widget, taffy, window);

    taffy
        .compute_layout(window, Size::max_content())
        .expect("Failed to compute layout");

    draw_widget(
        widget,
        taffy,
        window,
        0,
        info,
        antialiasing,
        &mut sketches,
        theme,
    );

    // cleanup interactions
    info.keys.clear();

    sketches
}

pub fn layout_widget(widget: &Box<dyn Widget>, taffy: &mut TaffyTree, parent: NodeId) {
    let node = taffy
        .new_leaf(widget.style().clone())
        .expect("Failed to create layout node");

    taffy
        .add_child(parent, node)
        .expect("Failed to add layout node to parent");

    for child in widget.children() {
        layout_widget(child, taffy, node);
    }
}

pub fn draw_widget(
    widget: &mut Box<dyn Widget>,
    taffy: &mut TaffyTree,
    parent: NodeId,
    child_index: usize,
    info: &mut InteractionInfo,
    antialiasing: bool,
    sketches: &mut Vec<Sketch>,
    theme: &Box<dyn Theme>,
) {
    let layout = taffy
        .layout(
            taffy
                .child_at_index(parent, child_index)
                .expect("Failed to get child node"),
        )
        .expect("Failed to get layout");

    sketches.append(&mut widget.render(layout, &theme, info));

    for (idx, child) in widget.children_mut().iter_mut().enumerate() {
        draw_widget(child, taffy, parent, idx, info, antialiasing, sketches, &theme);
    }
}

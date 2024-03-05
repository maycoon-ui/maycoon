use femtovg::renderer::OpenGl;
use femtovg::Canvas;
use taffy::{Dimension, NodeId, Size, Style, TaffyTree};

use may_theme::theme::Theme;

use crate::widget::interaction::InteractionInfo;
use crate::widget::update::UpdateMode;
use crate::widget::{PathMode, Sketch, Widget};

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

pub fn update_root_widget(
    widget: &mut Box<dyn Widget>,
    info: &mut InteractionInfo,
    update: &mut UpdateMode,
) {
    widget.update(info, update);

    for child in widget.children_mut() {
        update_root_widget(child, info, update);
    }
}

pub fn draw_root_widget(
    widget: &mut Box<dyn Widget>,
    size: (f32, f32),
    taffy: &mut TaffyTree,
    antialiasing: bool,
    theme: &Box<dyn Theme>,
    update: &mut UpdateMode,
) -> Vec<Sketch> {
    let mut sketches: Vec<Sketch> = Vec::with_capacity(32);

    let window = taffy
        .new_leaf(Style {
            size: Size::<Dimension> {
                width: Dimension::Length(size.0),
                height: Dimension::Length(size.1),
            },
            ..Style::default()
        })
        .expect("Failed to create window layout");

    if update.layout || update.force {
        layout_widget(&widget, taffy, window);

        taffy
            .compute_layout(window, Size::max_content())
            .expect("Failed to compute layout");
    }

    if update.draw || update.force {
        draw_widget(widget, taffy, window, 0, antialiasing, &mut sketches, theme);
    }

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

    sketches.append(&mut widget.render(layout, &theme));

    for (idx, child) in widget.children_mut().iter_mut().enumerate() {
        draw_widget(child, taffy, parent, idx, antialiasing, sketches, &theme);
    }
}

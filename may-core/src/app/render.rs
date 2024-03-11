use bitflags::Flags;
use femtovg::renderer::OpenGl;
use femtovg::Canvas;
use taffy::{NodeId, Size, TaffyTree};

use may_theme::theme::Theme;

use crate::widget::interaction::InteractionInfo;
use crate::widget::update::Update;
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

pub fn update_widget(
    widget: &mut Box<dyn Widget>,
    info: &InteractionInfo,
    taffy: &TaffyTree,
    parent: NodeId,
    child_index: usize,
    mut update: Update,
) -> Update {
    let node = taffy
        .child_at_index(parent, child_index)
        .expect("Failed to get child node");

    update.insert(widget.update(info, taffy.layout(node).expect("Failed to get layout")));

    for (idx, child) in widget.children_mut().iter_mut().enumerate() {
        update_widget(child, info, taffy, node, idx, update);
    }

    update
}

pub fn build_widget(
    widget: &mut Box<dyn Widget>,
    window: NodeId,
    taffy: &mut TaffyTree,
    antialiasing: bool,
    theme: &Box<dyn Theme>,
    update: &mut Update,
) -> Vec<Sketch> {
    let mut sketches: Vec<Sketch> = Vec::with_capacity(32);

    if update.contains(Update::LAYOUT) || update.contains(Update::FORCE) {
        taffy
            .set_children(window, &[])
            .expect("Failed to clear layout tree");

        layout_widget(&widget, taffy, window);

        taffy
            .compute_layout(window, Size::max_content())
            .expect("Failed to compute layout");
    }

    if update.contains(Update::DRAW) || update.contains(Update::FORCE) {
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

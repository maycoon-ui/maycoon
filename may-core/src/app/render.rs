use femtovg::Canvas;
use femtovg::renderer::OpenGl;
use taffy::{Dimension, NodeId, Size, Style, TaffyTree};
use crate::app::context::AppContext;
use crate::app::page::Page;
use crate::widget::interaction::{Action, InteractionInfo};
use crate::widget::{PathMode, Sketch, Widget};

pub fn render_sketches(sketches: Vec<Sketch>, canvas: &mut Canvas<OpenGl>) {
    for sketch in sketches {
        match sketch {
            Sketch::Path(path, paint, mode) => {
                match mode {
                    PathMode::Fill => canvas.fill_path(&path, &paint),
                    PathMode::Stroke => canvas.stroke_path(&path, &paint),
                }
            }

            Sketch::Text(text, pos, paint, mode) => {
                match mode {
                    PathMode::Fill => canvas.fill_text(pos.x, pos.y, &text, &paint),
                    PathMode::Stroke => canvas.stroke_text(pos.x, pos.y, &text, &paint),
                }.expect("Failed to render text");
            }
        }
    }
}
pub fn draw_root_widget(
    page: &mut Box<dyn Page>,
    ctx: &mut AppContext,
    size: (f32, f32),
    taffy: &mut TaffyTree,
    info: &mut InteractionInfo,
    antialiasing: bool,
) -> Vec<Sketch> {
    let mut widget = page.render(ctx);
    let mut sketches: Vec<Sketch> = Vec::with_capacity(16);

    let window = taffy.new_leaf(Style {
        size: Size::<Dimension> {
            width: Dimension::Length(size.0),
            height: Dimension::Length(size.1),
        },
        ..Style::default()
    }).expect("Failed to create window layout");

    layout_widget(&widget, taffy, window);

    taffy.compute_layout(
        window,
        Size::max_content()
    ).expect("Failed to compute layout");

    draw_widget(&mut widget, taffy, window, 0, info, antialiasing, &mut sketches);

    // cleanup interactions
    info.keys.clear();

    sketches
}

pub fn layout_widget(widget: &Box<dyn Widget>, taffy: &mut TaffyTree, parent: NodeId) {
    let node = taffy.new_leaf(
        widget.style().clone()
    ).expect("Failed to create layout node");

    taffy.add_child(
        parent,
        node
    ).expect("Failed to add layout node to parent");

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
) {
    let layout = taffy.layout(
        taffy.child_at_index(
            parent,
            child_index
        ).expect("Failed to get child node")
    ).expect("Failed to get layout");

    if widget.interactive() {
        let mut actions = Vec::new();

        if let Some(cursor) = info.cursor {
            if cursor.x as f32 >= layout.location.x
                && cursor.y as f32 >= layout.location.y
                && cursor.x as f32 <= layout.location.x + layout.size.width
                && cursor.y as f32 <= layout.location.y + layout.size.height {
                actions.push(Action::Hover);
            }
        }

        for key in &info.keys {
            actions.push(
                Action::Key(
                    &key.logical_key,
                    &key.state
                )
            );
        }

        actions.push(Action::Modifiers(
            &info.modifiers
        ));

        widget.interact(actions);
    }

    sketches.append(
        &mut widget.render(
            layout
        )
    );

    for (idx, child) in widget.children_mut().iter_mut().enumerate() {
        draw_widget(child, taffy, parent, idx, info, antialiasing, sketches);
    }
}

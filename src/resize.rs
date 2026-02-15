use egui::{Id, Rect, Sense, Vec2, ViewportCommand};

pub static RESIZE_BORDER_THICKNESS: f32 = 6.0;

pub fn resize_borders_ui(ctx: &egui::Context, ui: &mut egui::Ui, app_rect: Rect, enabled: bool) {
    if !enabled {
        return;
    }

    let t = RESIZE_BORDER_THICKNESS;
    let nw = Rect::from_min_size(app_rect.left_top(), Vec2::splat(t));
    let ne = Rect::from_min_size(app_rect.right_top() - Vec2::splat(t), Vec2::splat(t));
    let sw = Rect::from_min_size(app_rect.left_bottom() - Vec2::new(0.0, t), Vec2::splat(t));
    let se = Rect::from_min_size(app_rect.right_bottom() - Vec2::splat(t), Vec2::splat(t));

    let n = Rect::from_min_max(
        app_rect.left_top() + Vec2::new(t, 0.0),
        app_rect.right_top() + Vec2::new(-t, t),
    );
    let s = Rect::from_min_max(
        app_rect.left_bottom() + Vec2::new(t, -t),
        app_rect.right_bottom() + Vec2::new(-t, 0.0),
    );
    let w = Rect::from_min_max(
        app_rect.left_top() + Vec2::new(0.0, t),
        app_rect.left_bottom() + Vec2::new(t, -t),
    );
    let e = Rect::from_min_max(
        app_rect.right_top() + Vec2::new(-t, t),
        app_rect.right_bottom() + Vec2::new(0.0, -t),
    );

    handle_resize_rect(
        ctx,
        ui,
        nw,
        egui::ResizeDirection::NorthWest,
        egui::CursorIcon::ResizeNwSe,
    );
    handle_resize_rect(
        ctx,
        ui,
        ne,
        egui::ResizeDirection::NorthEast,
        egui::CursorIcon::ResizeNeSw,
    );
    handle_resize_rect(
        ctx,
        ui,
        sw,
        egui::ResizeDirection::SouthWest,
        egui::CursorIcon::ResizeNeSw,
    );
    handle_resize_rect(
        ctx,
        ui,
        se,
        egui::ResizeDirection::SouthEast,
        egui::CursorIcon::ResizeNwSe,
    );

    handle_resize_rect(
        ctx,
        ui,
        n,
        egui::ResizeDirection::North,
        egui::CursorIcon::ResizeVertical,
    );
    handle_resize_rect(
        ctx,
        ui,
        s,
        egui::ResizeDirection::South,
        egui::CursorIcon::ResizeVertical,
    );
    handle_resize_rect(
        ctx,
        ui,
        w,
        egui::ResizeDirection::West,
        egui::CursorIcon::ResizeHorizontal,
    );
    handle_resize_rect(
        ctx,
        ui,
        e,
        egui::ResizeDirection::East,
        egui::CursorIcon::ResizeHorizontal,
    );
}

fn handle_resize_rect(
    ctx: &egui::Context,
    ui: &mut egui::Ui,
    rect: Rect,
    dir: egui::ResizeDirection,
    cursor: egui::CursorIcon,
) {
    let id = Id::new(("custom_frame_resize", dir as u8));
    let resp = ui.interact(rect, id, Sense::click_and_drag());

    if resp.hovered() {
        ui.output_mut(|o| o.cursor_icon = cursor);
    }

    if resp.drag_started_by(egui::PointerButton::Primary) {
        ctx.send_viewport_cmd(ViewportCommand::BeginResize(dir));
    }
}

pub fn top_resize_hot_zones(app_rect: egui::Rect) -> [egui::Rect; 3] {
    let t = RESIZE_BORDER_THICKNESS;
    let nw = egui::Rect::from_min_size(app_rect.left_top(), egui::Vec2::splat(t));
    let ne = egui::Rect::from_min_size(
        app_rect.right_top() - egui::Vec2::splat(t),
        egui::Vec2::splat(t),
    );

    let n = egui::Rect::from_min_max(
        app_rect.left_top() + egui::vec2(t, 0.0),
        app_rect.right_top() + egui::vec2(-t, t),
    );

    [nw, n, ne]
}

pub fn pointer_in_any(ctx: &egui::Context, rects: &[egui::Rect]) -> bool {
    let p = ctx.input(|i| i.pointer.hover_pos());
    match p {
        Some(p) => rects.iter().any(|r| r.contains(p)),
        None => false,
    }
}

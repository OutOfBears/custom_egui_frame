use egui::{
    vec2, Align2, Color32, FontId, Id, Image, ImageSource, Rect, Response, Rounding, Sense, Ui,
    Vec2, ViewportCommand, Widget,
};

use super::title_bar_button::TitleBarButton;

pub struct TitleBar<'t> {
    title: Option<&'static str>,
    icon: Option<Image<'t>>,
    restore_icon: Image<'t>,
    maximize_icon: Image<'t>,
    minimize_icon: Image<'t>,
    close_icon: Image<'t>,
    maximize: bool,
    rect: Rect,
}

impl<'t> TitleBar<'t> {
    pub fn new(rect: Rect) -> Self {
        Self {
            title: None,
            icon: None,
            maximize: true,
            close_icon: Image::new(egui::include_image!("../assets/close.svg")),
            minimize_icon: Image::new(egui::include_image!("../assets/minimize.svg")),
            maximize_icon: Image::new(egui::include_image!("../assets/maximize.svg")),
            restore_icon: Image::new(egui::include_image!("../assets/restore.svg")),
            rect,
        }
    }

    pub fn with_title(mut self, title: &'static str) -> Self {
        self.title = Some(title);
        self
    }

    pub fn with_icon(mut self, icon: &'t ImageSource<'t>) -> Self {
        self.icon = Some(Image::new(icon.clone()));
        self
    }

    pub fn with_maximize(mut self, maximize: bool) -> Self {
        self.maximize = maximize;
        self
    }

    pub fn with_rect(mut self, rect: Rect) -> Self {
        self.rect = rect;
        self
    }

    fn paint_buttons(&self, ui: &mut Ui) {
        let button_size = Vec2::new(44.0, 30.0);
        let icon_size = Vec2::new(11.0, 22.0);

        let is_maximized = ui.input(|i| i.viewport().maximized.unwrap_or(false));

        let close_button = TitleBarButton::new(button_size)
            .with_color(Color32::from_rgba_unmultiplied(255, 0, 0, 100))
            .with_hover_color(Color32::from_rgba_unmultiplied(255, 0, 0, 255))
            .with_rounding(Rounding {
                ne: 5,
                ..Default::default()
            })
            .with_icon(&self.close_icon, icon_size)
            .with_hover_color(Color32::RED)
            .with_hover_text("Close the window");

        if close_button.ui(ui).clicked() {
            ui.ctx().send_viewport_cmd(ViewportCommand::Close);
        }

        if self.maximize {
            let restore_maximize_button = {
                if is_maximized {
                    TitleBarButton::new(button_size)
                        .with_icon(&self.restore_icon, icon_size)
                        .with_hover_text("Restore window")
                } else {
                    TitleBarButton::new(button_size)
                        .with_icon(&self.maximize_icon, icon_size)
                        .with_hover_text("Maximize window")
                }
            };

            if restore_maximize_button.ui(ui).clicked() {
                ui.ctx()
                    .send_viewport_cmd(ViewportCommand::Maximized(!is_maximized));
            }
        }

        let minimize_button = TitleBarButton::new(button_size)
            .with_icon(&self.minimize_icon, icon_size)
            .with_hover_text("Minimize the window");

        if minimize_button.ui(ui).clicked() {
            ui.ctx().send_viewport_cmd(ViewportCommand::Minimized(true));
        }
    }
}

impl<'t> Widget for TitleBar<'t> {
    fn ui(self, ui: &mut Ui) -> Response {
        let rect = self.rect;

        let painter = ui.painter();
        let response = ui.interact(rect, Id::new("title_bar"), Sense::click());

        if let Some(text) = self.title {
            painter.text(
                (rect.left_center().to_vec2() + vec2(30.0, 0.0)).to_pos2(),
                Align2::LEFT_CENTER,
                text,
                FontId::proportional(12.0),
                ui.style().visuals.text_color(),
            );
        }

        if let Some(icon) = &self.icon {
            // CHANGE ME!!!
            icon.paint_at(
                ui,
                Rect::from_center_size(rect.left_center() + vec2(16.0, 0.0), Vec2::splat(16.0)),
            );
        }

        if response.is_pointer_button_down_on() {
            ui.ctx().send_viewport_cmd(ViewportCommand::StartDrag);
        }

        ui.allocate_ui_at_rect(rect.shrink2(Vec2::new(0.0, 1.0)), |ui| {
            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                ui.visuals_mut().button_frame = false;
                ui.spacing_mut().item_spacing.x = 0.0;
                ui.add_space(1.0);

                self.paint_buttons(ui);
            });
        });

        response
    }
}

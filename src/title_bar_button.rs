use egui::{Button, Color32, Image, Rect, Response, Rounding, Ui, Vec2, Widget};

pub struct TitleBarButton<'t> {
    size: Vec2,
    color: Option<Color32>,
    hover_color: Option<Color32>,
    hover_text: Option<&'static str>,
    rounding: Option<Rounding>,
    icon: Option<&'t Image<'t>>,
    icon_color: Option<Color32>,
    icon_hover_color: Option<Color32>,
    icon_size: Option<Vec2>,
}

impl<'t> TitleBarButton<'t> {
    pub fn new(size: Vec2) -> Self {
        Self {
            size,
            color: None,
            hover_text: None,
            hover_color: None,
            rounding: None,
            icon: None,
            icon_size: None,
            icon_color: None,
            icon_hover_color: None,
        }
    }

    pub fn with_icon(mut self, icon: &'t Image<'t>, size: Vec2) -> Self {
        self.icon = Some(icon);
        self.icon_size = Some(size);
        self
    }

    pub fn with_icon_color(mut self, color: Color32) -> Self {
        self.icon_color = Some(color);
        self
    }

    pub fn with_icon_hover_color(mut self, color: Color32) -> Self {
        self.icon_hover_color = Some(color);
        self
    }

    pub fn with_color(mut self, color: Color32) -> Self {
        self.color = Some(color);
        self
    }

    pub fn with_hover_text(mut self, text: &'static str) -> Self {
        self.hover_text = Some(text);
        self
    }

    pub fn with_hover_color(mut self, color: Color32) -> Self {
        self.hover_color = Some(color);
        self
    }

    pub fn with_rounding(mut self, rounding: Rounding) -> Self {
        self.rounding = Some(rounding);
        self
    }
}

// let button_size = Vec2::new(44.0, 30.0);
// let image_size = Vec2::new(11.0, 22.0);
// let mut rounding = Rounding::same(0.0);

// let base_bg = styles.visuals.widgets.active.weak_bg_fill;
// styles.visuals.widgets.inactive.weak_bg_fill = Color32::TRANSPARENT;

// if color == Color32::RED {
// 	styles.visuals.widgets.hovered.weak_bg_fill =
// 		Color32::from_rgba_unmultiplied(255, 0, 0, 255);
// 	styles.visuals.widgets.active.weak_bg_fill =
// 		Color32::from_rgba_unmultiplied(255, 0, 0, 100);
// 	rounding.ne = 5.0;
// } else {
// 	styles.visuals.widgets.hovered.weak_bg_fill = Color32::from_white_alpha(2);
// 	styles.visuals.widgets.active.weak_bg_fill = Color32::from_white_alpha(10);
// }

impl<'t> Widget for TitleBarButton<'t> {
    fn ui(self, ui: &mut Ui) -> Response {
        let button_size = self.size;
        let color = self.color.unwrap_or(Color32::from_white_alpha(10));
        let hover_color = self.hover_color.unwrap_or(Color32::from_white_alpha(2));

        let orig_styles = ui.style().clone();

        ui.style_mut().visuals.widgets.inactive.weak_bg_fill = Color32::TRANSPARENT;
        ui.style_mut().visuals.widgets.active.weak_bg_fill = color;
        ui.style_mut().visuals.widgets.hovered.weak_bg_fill = hover_color;

        let button = Button::new("")
            .stroke(egui::Stroke::new(0.0, Color32::TRANSPARENT))
            .rounding(self.rounding.unwrap_or(0.0.into()))
            .min_size(button_size)
            .frame(true);

        let mut response = button.ui(ui);

        let icon = self.icon;
        let icon_size = self.icon_size.unwrap_or(button_size);
        let icon_color = self.icon_color.unwrap_or(ui.style().visuals.text_color());
        let icon_hover_color = self.icon_hover_color.unwrap_or(Color32::WHITE);

        if let Some(icon) = icon {
            let image = icon
                .clone()
                .max_size(icon_size)
                .tint(if response.hovered() {
                    icon_hover_color
                } else {
                    icon_color
                });

            let image_size = image.load_and_calc_size(ui, icon_size).unwrap_or(icon_size);

            image.paint_at(
                ui,
                Rect::from_center_size(response.rect.center(), image_size),
            );
        }

        ui.style_mut().visuals.widgets.active.weak_bg_fill =
            orig_styles.visuals.widgets.active.weak_bg_fill;
        ui.style_mut().visuals.widgets.hovered.weak_bg_fill =
            orig_styles.visuals.widgets.hovered.weak_bg_fill;

        if let Some(text) = self.hover_text {
            response = response.on_hover_text(text);
        }

        response
    }
}

use egui::{CentralPanel, Frame, ImageSource, Response, Stroke, Ui, Widget};

use super::title_bar::TitleBar;

pub struct Window<'t> {
    title: Option<&'static str>,
    icon: Option<ImageSource<'t>>,
    maximize: bool,
}

impl<'t> Window<'t> {
    pub fn new() -> Self {
        Self {
            title: None,
            icon: None,
            maximize: true,
        }
    }

    pub fn with_title(mut self, title: &'static str) -> Self {
        self.title = Some(title);
        self
    }

    pub fn with_icon(mut self, icon: ImageSource<'t>) -> Self {
        self.icon = Some(icon);
        self
    }

    pub fn with_maximize(mut self, maximize: bool) -> Self {
        self.maximize = maximize;
        self
    }

    pub fn show(&mut self, ctx: &egui::Context, contents: impl FnOnce(&mut Ui)) -> Response {
        let maximized = ctx.input(|o| o.viewport().maximized.unwrap_or(false));
        let stroke = ctx.style().visuals.widgets.noninteractive.bg_stroke;

        let panel_frame = Frame {
            fill: ctx.style().visuals.window_fill(),
            corner_radius: if maximized { 0.0.into() } else { 5.0.into() },
            stroke: Stroke::new(if maximized { 0.0 } else { 1.0 }, stroke.color),
            outer_margin: if maximized { 0.0.into() } else { 2.0.into() },
            ..Default::default()
        };

        let panel = CentralPanel::default().frame(panel_frame).show(ctx, |ui| {
            let app_rect = ui.max_rect();

            let title_bar_height = 32.0;
            let title_bar_rect = {
                let mut rect = app_rect;
                rect.max.y = rect.min.y + title_bar_height;
                rect
            };

            let content_rect = {
                let mut rect = app_rect;
                rect.min.y = title_bar_rect.max.y;
                rect
            }
            .shrink(4.0);

            let mut title_bar = TitleBar::new(title_bar_rect).with_maximize(self.maximize);

            if let Some(icon) = &self.icon {
                title_bar = title_bar.with_icon(icon);
            }

            if let Some(title) = self.title {
                title_bar = title_bar.with_title(title);
            }

            title_bar.ui(ui);

            let mut child_contents = ui.child_ui(content_rect, *ui.layout(), None);
            contents(&mut child_contents);
        });

        panel.response
    }
}

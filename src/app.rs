use core::f32;

use egui::{epaint::RectShape, Color32, FontFamily, FontId, Pos2, Rect, Rounding, Shape, Stroke, TextEdit, TextureId, Vec2};
use serde::{Deserialize, Serialize};

use crate::component::{MadeWithEgui, SimpleTopMenuBar, TextEditSingleline, TransparentLabel};

/// We derive Deserialize/Serialize so we can persist app state on shutdown.
#[derive(Serialize, Deserialize)]
#[serde(default)] // if we add new fields, give them default values when deserializing old state
pub struct TemplateApp {
    // Example stuff:
    label: String,
    #[serde(skip)] // This how you opt-out of serialization of a field
    value: f32,

    was_single_line_focused: bool,
    signleline_text: String,
    signleline_hint_text: String,
}

impl Default for TemplateApp {
    fn default() -> Self {
        Self {
            label: "Hello World!".to_owned(),
            value: 2.7,

            was_single_line_focused: false,
            signleline_text: "".to_string(),
            signleline_hint_text: "Write here :)".to_owned(),
        }
    }
}

impl TemplateApp {
    /// Called once before the first frame.
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // This is also where you can customize the look and feel of egui using
        // `cc.egui_ctx.set_visuals` and `cc.egui_ctx.set_fonts`.

        // Load previous app state (if any).
        // Note that you must enable the `persistence` feature for this to work.
        if let Some(storage) = cc.storage {
            return eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default();
        }

        Default::default()
    }
}

impl eframe::App for TemplateApp {
    /// Called by the frame work to save state before shutdown.
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, eframe::APP_KEY, self);
    }

    /// Called each time the UI needs repainting, which may be many times per second.
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            SimpleTopMenuBar::ui(ui);
        });

        let font_size: f32 = 100.0;
        let char_limit: usize = 250;
        egui::CentralPanel::default().show(ctx, |ui| {
            // The central panel the region left after adding TopPanel's and SidePanel's
            ui.heading("eframe-porva-002");
            let mut singleline = TextEdit::singleline(&mut self.signleline_text)
                .hint_text(&self.signleline_hint_text)
                .font(FontId::new(font_size, FontFamily::default()))
                .cursor_at_end(true)
                .desired_width(f32::INFINITY)
                .char_limit(char_limit)
                .show(ui);

            // This attempts to fix a bug: if you click on the upper-half of the singleline 
            // It will put the cursor at the BEGINNING istead that at the END.
            if !self.was_single_line_focused && singleline.response.has_focus() {
                self.was_single_line_focused = true;
                TextEditSingleline::set_cursor_at_end(&mut singleline, ui);
            }
            else if self.was_single_line_focused && singleline.response.lost_focus() {
                self.was_single_line_focused = false;
            }
           
            let font_id = FontId::default(); // To generalize
            let size = Vec2::new(font_id.size *2.0, font_id.size * 1.0);
            let margin = 0.075*singleline.response.rect.size().y;
            let refs = Pos2::new(
                singleline.response.rect.max.x - margin,
                singleline.response.rect.min.y + margin,
            );
            let a = Pos2::new(
                refs.x - size.x,
                refs.y,
            );
            let b = Pos2::new(
                refs.x,
                refs.y + size.y,
            );

            let label = TransparentLabel::default(ui);
            label.ui(ui);

            let shape = Shape::Rect(RectShape { 
                rect: Rect::from_two_pos(a, b),
                rounding: Rounding::same(5.0),
                fill: Color32::from_rgb(125, 125, 125),
                stroke: Stroke::NONE,
                blur_width: 0.0, 
                fill_texture_id: TextureId::default(), 
                uv: Rect::ZERO,
            });
            ui.painter().add(shape);

            MadeWithEgui::ui(ui);
        });
    }
}



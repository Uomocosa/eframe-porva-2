use core::f32;

use egui::{epaint::{RectShape, TextShape}, Color32, FontFamily, FontId, Pos2, Rect, Rounding, Shape, Stroke, TextEdit, TextureId, Vec2};
use serde::{Deserialize, Serialize};

use crate::{component::{MadeWithEgui, SimpleTopMenuBar, TextEditSingleline}, utility::{get_galley_from_str, green_to_red_linear_gradient}};

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

    char_countdown_background_size: Vec2,
}

impl Default for TemplateApp {
    fn default() -> Self {
        Self {
            label: "Hello World!".to_owned(),
            value: 2.7,

            was_single_line_focused: false,
            signleline_text: "".to_string(),
            signleline_hint_text: "Write here :)".to_owned(),

            char_countdown_background_size: Vec2::new(30.0, 15.0),
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

            // This attempts to fix a bug: if you click at the top of the [singleline] "block"
            // It will put the cursor at the BEGINNING istead that at the END.
            if !self.was_single_line_focused && singleline.response.has_focus() {
                self.was_single_line_focused = true;
                TextEditSingleline::set_cursor_at_end(&mut singleline, ui);
            }
            else if self.was_single_line_focused && singleline.response.lost_focus() {
                self.was_single_line_focused = false;
            }

            let margin = 0.075*singleline.response.rect.size().y;
            let refs = Pos2::new(
                singleline.response.rect.max.x - margin,
                singleline.response.rect.min.y + margin,
            );
            let a = Pos2::new(
                refs.x - self.char_countdown_background_size.x,
                refs.y,
            );
            let b = Pos2::new(
                refs.x,
                refs.y + self.char_countdown_background_size.y,
            );

            let remaining_chars = char_limit - self.signleline_text.len();
            if remaining_chars <= 100 {
                let shape = Shape::Rect(RectShape{ 
                    rect: Rect::from_two_pos(a, b),
                    rounding: Rounding::same(5.0),
                    fill: Color32::from_rgb(125, 125, 125),
                    stroke: Stroke::NONE,
                    blur_width: 0.0, 
                    fill_texture_id: TextureId::default(), 
                    uv: Rect::ZERO,
                });
                
                

                let galley = get_galley_from_str(
                    remaining_chars.to_string(), 
                    egui::Align::Center, 
                    ui
                );
                // debug!(ui, "galley.rect.center() = {:?}", galley.rect.center());
                // debug!(ui, "galley.mesh_bounds.center() = {:?}", galley.mesh_bounds.center());
                let galley_center = (galley.rect.center() + galley.mesh_bounds.center().to_vec2()) / 2.0;
                let pos = Pos2::new(
                    a.x + (b.x-a.x)/2.0 - galley_center.x,
                    a.y + (b.y-a.y)/2.0 - galley_center.y,
                );
                let fallback_color = green_to_red_linear_gradient((remaining_chars as f32)/(char_limit as f32));
                let text = Shape::Text(TextShape{
                    pos,
                    galley,
                    underline: Stroke::NONE,
                    fallback_color,
                    override_text_color: None,
                    opacity_factor: 1.0,
                    angle: 0.0,
                });

                ui.painter().add(shape);
                ui.painter().add(text);
            }

            MadeWithEgui::ui(ui);
        });
    }
}



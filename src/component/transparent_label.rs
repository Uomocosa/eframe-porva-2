use std::sync::Arc;

use egui::{Color32, Galley, Label, Pos2, Response, Ui};

pub struct TransparentLabel{
    pub text: String,
    pub pos: Pos2,
    pub galley: Arc<Galley>,
    pub opacity: f32,
    pub fallback_color: Color32,
    pub response: Response,
}

impl TransparentLabel {
    pub fn default(ui: &mut Ui) -> Self {
        let text = "Label";
        let label = Label::new(text);
        let (pos, galley, response) = label.layout_in_ui(ui);
        let fallback_color = Color32::from_rgb(100, 100, 100);
        let opacity = 0.75;
        Self { 
            text: text.to_string(), 
            pos, 
            galley, 
            opacity,
            fallback_color,
            response, 
        }
    }

    pub fn create(text: String, fallback_color: Color32, opacity:f32, ui: &mut Ui) -> Self {
        let label = Label::new(text.clone());
        let (pos, galley, response) = label.layout_in_ui(ui);
        Self { 
            text, 
            pos, 
            galley, 
            opacity,
            fallback_color,
            response, 
        }
    }

    pub fn simple_text(text: String, ui: &mut Ui) -> Self {
        let label = Label::new(text.clone());
        let (pos, galley, response) = label.layout_in_ui(ui);
        let fallback_color = Color32::from_rgb(100, 100, 100);
        let opacity = 0.75;
        Self { 
            text, 
            pos, 
            galley, 
            opacity,
            fallback_color,
            response, 
        }
    }

    pub fn ui(&self, ui: &mut Ui) {
        let ui_builder = egui::UiBuilder::new();
        ui.scope_builder(ui_builder, |ui| {
            ui.multiply_opacity(self.opacity);
            let painter: &egui::Painter = ui.painter();
            painter.galley(self.pos, self.galley.clone(), self.fallback_color);
        });
    }
}
use egui::{text_edit::TextEditOutput, TextEdit};

pub fn show(text: &mut String, ui: &mut egui::Ui) -> TextEditOutput {
    TextEdit::singleline(text).show(ui)
}
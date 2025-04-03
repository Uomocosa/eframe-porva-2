use egui::text::CCursorRange;
use egui::{text_edit::TextEditOutput, TextEdit};

pub struct MyTextEditOutput;

impl MyTextEditOutput {
    pub fn show(text: &mut String, ui: &mut egui::Ui) -> TextEditOutput {
        TextEdit::singleline(text).show(ui)
    }

    pub fn set_cursor(widget: &mut TextEditOutput, cc_range: Option<CCursorRange>, ui: &mut egui::Ui) {
        let text_edit_id = widget.response.id;
        if let Some(mut state) = egui::TextEdit::load_state(ui.ctx(), text_edit_id) {
            state.cursor.set_char_range(cc_range);
            state.store(ui.ctx(), text_edit_id);
            widget.response.request_focus(); // give focus back to the `TextEdit`.
        }
    }

    pub fn get_cc_range(widget: TextEditOutput) -> Option<CCursorRange> {
        widget.state.cursor.char_range()
    }
}
use egui::text::{CCursor, CCursorRange};
use egui::TextBuffer;
use egui::{text_edit::TextEditOutput, TextEdit};

pub struct TextEditSingleline;

impl TextEditSingleline {
    pub fn text_edit(text: &mut dyn TextBuffer) -> TextEdit<'_> {
        TextEdit::singleline(text)
    }

    pub fn show(text: &mut dyn TextBuffer, ui: &mut egui::Ui) -> TextEditOutput {
        TextEdit::singleline(text).show(ui)
    }

    pub fn set_cursor(
        singleline: &mut TextEditOutput,
        cc_range: Option<CCursorRange>,
        ui: &mut egui::Ui,
    ) {
        let text_edit_id = singleline.response.id;
        if let Some(mut state) = egui::TextEdit::load_state(ui.ctx(), text_edit_id) {
            state.cursor.set_char_range(cc_range);
            state.store(ui.ctx(), text_edit_id);
            singleline.response.request_focus(); // give focus back to the `TextEdit`.
        }
    }

    pub fn set_cursor_at_end(singleline: &mut TextEditOutput, ui: &mut egui::Ui) {
        let text_edit_id = singleline.response.id;
        if let Some(mut state) = egui::TextEdit::load_state(ui.ctx(), text_edit_id) {
            let end = CCursor::new(singleline.galley.text().len());
            let cc_range = Some(CCursorRange::one(end));
            state.cursor.set_char_range(cc_range);
            state.store(ui.ctx(), text_edit_id);
            singleline.response.request_focus(); // give focus back to the `TextEdit`.
        }
    }

    pub fn get_cc_range(singleline: TextEditOutput) -> Option<CCursorRange> {
        singleline.state.cursor.char_range()
    }
}

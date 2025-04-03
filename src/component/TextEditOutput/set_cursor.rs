use egui::{text::CCursorRange, text_edit::TextEditOutput};

pub fn set_cursor(widget: &mut TextEditOutput, cc_range: Option<CCursorRange>, ui: &mut egui::Ui) {
    let text_edit_id = widget.response.id;
    if let Some(mut state) = egui::TextEdit::load_state(ui.ctx(), text_edit_id) {
        state.cursor.set_char_range(cc_range);
        state.store(ui.ctx(), text_edit_id);
        widget.response.request_focus(); // give focus back to the `TextEdit`.
    }
}
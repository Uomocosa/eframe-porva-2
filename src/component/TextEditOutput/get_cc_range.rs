use egui::{text::CCursorRange, text_edit::TextEditOutput};

pub fn get_cc_range(widget: TextEditOutput) -> Option<CCursorRange> {
    widget.state.cursor.char_range()
}
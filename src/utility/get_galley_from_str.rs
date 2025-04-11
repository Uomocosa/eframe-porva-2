use egui::{Align, FontSelection, Galley, WidgetText};
use std::sync::Arc;

pub fn get_galley_from_str(
    text: impl Into<WidgetText>,
    valign: Align,
    ui: &mut egui::Ui,
) -> Arc<Galley> {
    let text: WidgetText = text.into();
    let layout_job = text.into_layout_job(ui.style(), FontSelection::Default, valign);
    ui.fonts(|fonts| fonts.layout_job(layout_job))
}

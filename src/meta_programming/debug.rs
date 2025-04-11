#[macro_export]
macro_rules! debug {
    // ui: should be &mut ugui::Ui
    ($ui:expr, $($arg:tt)*) => {
        let text = format!($($arg)*);
        let debug_label = TransparentLabel::simple_text(text, $ui);
        debug_label.ui($ui);
    }
}
pub fn ui(ui: &mut egui::Ui) {
    ui.separator();

    ui.add(egui::github_link_file!(
        "https://github.com/emilk/eframe_template/blob/main/",
        "Source code."
    ));

    ui.with_layout(egui::Layout::bottom_up(egui::Align::LEFT), |ui| {
        ui.horizontal(|ui| {
            ui.spacing_mut().item_spacing.x = 0.0;
            ui.label("Powered by ");
            ui.hyperlink_to("egui", "https://github.com/emilk/egui");
            ui.label(" and ");
            ui.hyperlink_to(
                "eframe",
                "https://github.com/emilk/egui/tree/master/crates/eframe",
            );
            ui.label(".");
        });
        egui::warn_if_debug_build(ui);
    });
}
use egui::Ui;

pub fn ui(ui:&mut Ui) {
    egui::menu::bar(ui, |ui| {
        // NOTE: no File->Quit on web pages!
        let is_web = cfg!(target_arch = "wasm32");
        if !is_web {
            ui.menu_button("File", |ui| {
                if ui.button("Quit").clicked() {
                    ui.ctx().send_viewport_cmd(egui::ViewportCommand::Close);
                }
            });
            ui.add_space(16.0);
        }
        egui::widgets::global_theme_preference_buttons(ui);
    });
}
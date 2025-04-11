use egui::{epaint::RectShape, Color32, Pos2, Rect, Rounding, Stroke};

pub struct SimpleRect;

impl SimpleRect {
    pub fn shape() -> RectShape {
        let p1 = Pos2 { x: 0.0, y: 0.0 };
        let p2 = Pos2 { x: 100.0, y: 100.0 };
        let rect = Rect { min: p1, max: p2 };
        let rounding = Rounding {
            nw: 0.5,
            ne: 0.5,
            sw: 0.5,
            se: 0.5,
        };
        let fill_color = Color32::from_rgb(0, 0, 0);
        let stroke = Stroke::new(1.0, Color32::from_rgb(255, 255, 255));
        RectShape::new(rect, rounding, fill_color, stroke)
    }

    pub fn ui(ui: egui::Ui) {
        let painter = ui.painter();
        painter.add(SimpleRect::shape());
    }
}

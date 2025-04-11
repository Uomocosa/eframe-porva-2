use egui::Color32;

pub fn green_to_red_linear_gradient(x: f32) -> Color32 {
    // Clamp x to the valid range [0.0, 1.0] to avoid unexpected results
    let x_clamped = x.clamp(0.0, 1.0);

    // --- How the gradient is implemented (Linear Interpolation) ---
    // We want to transition between two colors:
    // Color A (Green): R=0, G=255, B=0  (associated with x = 1.0)
    // Color B (Red):   R=255, G=0, B=0  (associated with x = 0.0)
    let r = 255.0 * (1.0 - x_clamped);
    let g = 255.0 * x_clamped;
    let b = 0.0;
    Color32::from_rgb(r as u8, g as u8, b as u8)
}

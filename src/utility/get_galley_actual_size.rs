use std::sync::Arc;
use egui::{Galley, Vec2};

pub fn get_galley_actual_size(galley: &Arc<Galley>) -> Vec2 {
    galley.mesh_bounds.size()
}
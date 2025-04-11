#![warn(clippy::all, rust_2018_idioms)]

mod app;
pub mod component;
pub mod utility;
pub use app::TemplateApp;
pub mod meta_programming;
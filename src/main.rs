mod monitor;
mod data; 
mod operations; 
mod json_helper;
mod gui_interface;


pub use crate::monitor::Progress;
pub use crate::data::TodoItem;
pub use crate::gui_interface::TestUI;
pub use egui::Ui;
pub use eframe::egui;


fn main() {
    let app: TestUI = TestUI;
    let native_options = eframe::NativeOptions::default();
    eframe::run_native("Todo App", native_options, Box::new(|_cc| Box::new(app)));
    operations::run(); 
}

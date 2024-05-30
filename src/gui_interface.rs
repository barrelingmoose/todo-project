pub use egui::Ui;
pub use eframe::egui;

pub struct TestUI; 

impl eframe::App for TestUI{
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.label("Hello World!");
        });
    }
}

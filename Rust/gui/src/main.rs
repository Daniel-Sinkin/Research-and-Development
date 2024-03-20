#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

use eframe::egui;

fn main() -> Result<(), eframe::Error> {
    env_logger::init(); // Log to stderr (if you run with `RUST_LOG=debug`).
    let options = eframe::NativeOptions {
        initial_window_size: Some(egui::vec2(640.0, 480.0)),
        ..Default::default()
    };
    eframe::run_native("Tempi", options, Box::new(|_cc| Box::<MyApp>::default()))
}

struct MyApp {
    name: String,
    age: u32,
}

impl Default for MyApp {
    fn default() -> Self {
        Self {
            name: "Daniel".to_owned(),
            age: 26,
        }
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Time Tracking");
            ui.horizontal(|ui| {
                let name_label = ui.label("Username: ");
                ui.text_edit_singleline(&mut self.name)
                    .labelled_by(name_label.id);
            });
            ui.add(egui::Slider::new(&mut self.age, 0..=1024).text("age"));
            if ui.button("Double Age").clicked() {
                self.age *= 2;
            }
            ui.label(format!(
                "Good morning '{}', your age is {}",
                self.name, self.age
            ));
        });
    }
}

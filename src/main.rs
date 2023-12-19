use eframe::egui::{self, widgets::ImageSource, Vec2};
use egui_extras;
use std::borrow::Cow;

const WIDTH: f32 = 960.;
const HEIGHT: f32 = 540.;

struct Balagan;

impl eframe::App for Balagan {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui_extras::install_image_loaders(ctx);
        egui::CentralPanel::default().show(ctx, |ui| {
            let image = ImageSource::Uri(Cow::from("file:///home/vovchik/rust/balagan/test.jpg"));
            ui.image(image);
        });
    }
}

fn main() {
    let mut nat_opt = eframe::NativeOptions::default();
    nat_opt.viewport.inner_size = Some(Vec2::new(WIDTH, HEIGHT));
    nat_opt.viewport.resizable = Some(true);
    let _ = eframe::run_native(
        "balagan",
        nat_opt,
        Box::new(|_cc| Box::new(Balagan))
    );
}

use eframe::egui::{self, widgets::ImageSource, Vec2};
use egui_extras;
use std::borrow::Cow;

const WIDTH: f32 = 960.;
const HEIGHT: f32 = 540.;

#[derive(Default)]
struct MyEguiApp;

impl MyEguiApp {
    fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        Self::default()
    }
}

impl eframe::App for MyEguiApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui_extras::install_image_loaders(ctx);
        egui::CentralPanel::default().show(ctx, |ui| {
            let image = ImageSource::Uri(Cow::from("file:///home/vovchik/rust/balagan/test.jpg"));
            ui.image(image);
            // ui.heading("Hello World!");
            // ui.image(my_texture_id, [WIDTH, HEIGHT]);
        });
    }
}

fn main() {
    let mut native_options = eframe::NativeOptions::default();
    native_options.viewport.inner_size = Some(Vec2::new(WIDTH, HEIGHT));
    native_options.viewport.resizable = Some(true);
    let _ = eframe::run_native(
        "balagan",
        native_options,
        Box::new(|cc| Box::new(MyEguiApp::new(cc))),
    );
}

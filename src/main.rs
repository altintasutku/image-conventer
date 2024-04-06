#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release
use eframe::{egui, run_native, NativeOptions};
use egui::{CentralPanel, Color32, RichText, ViewportBuilder};
use image::ImageResult;

fn main() -> Result<(), eframe::Error> {
    run_native(
        "Image Conventer",
        NativeOptions {
            viewport: ViewportBuilder::default().with_inner_size([640.0, 240.0]),
            ..Default::default()
        },
        Box::new(|_cc| Box::<MyApp>::default()),
    )
}

#[derive(Default)]
struct MyApp {
    input: Option<String>,
    output: Option<String>,
    error: Option<String>,
    success: Option<String>,
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        CentralPanel::default().show(ctx, |ui| {
            ui.heading("Image Converter");

            if let Some(error) = &self.error {
                ui.label(RichText::new(error).color(Color32::RED));
            }

            if let Some(success) = &self.success {
                ui.label(RichText::new(success).color(Color32::GREEN));
            }

            ui.horizontal(|ui| {
                if ui.button("Open file").clicked() {
                    if let Some(path) = rfd::FileDialog::new().pick_file() {
                        self.input = Some(path.display().to_string());
                    }
                }

                if let Some(input_path) = &self.input {
                    ui.label("Input file:");
                    ui.monospace(input_path);
                }
            });

            ui.horizontal(|ui| {
                if ui.button("Select output folder").clicked() {
                    if let Some(path) = rfd::FileDialog::new().pick_folder() {
                        self.output = Some(format!("{}\\output.png", path.display().to_string()));
                    }
                }

                if let Some(output_path) = &self.output {
                    ui.label("Output folder:");
                    ui.monospace(output_path);
                }
            });

            let convert_button = ui.button("Convert");

            if convert_button.clicked() {
                if let Some(input) = &self.input {
                    if let Some(output) = &self.output {
                        if let Ok(()) = convert_image(input, output) {
                            self.success = Some("Convert is successful!".to_owned());
                            self.error = None;
                        } else {
                            self.error = Some("Something went wrong with saving!".to_owned());
                        }
                    } else {
                        self.error = Some("Error: Output path is not specified.".to_owned());
                    }
                } else {
                    self.error = Some("Error: Input path is not specified.".to_owned());
                }
            }
        });
    }
}

fn convert_image(input: &String, output: &String) -> ImageResult<()> {
    let img = image::open(input).expect("Failed to open image");

    let converted_img = img.to_rgba8();

    converted_img.save(output)
}

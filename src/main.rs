#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

use crate::egui::{Align, Layout};
use eframe::egui;
use poll_promise::Promise;

mod guis;
use guis::ares_gui;
use guis::lemmeknow_gui;

fn main() -> Result<(), eframe::Error> {
    // Log to stdout (if you run with `RUST_LOG=debug`).
    tracing_subscriber::fmt::init();

    let options = eframe::NativeOptions {
        initial_window_size: Some(egui::vec2(650.0, 500.0)),
        ..Default::default()
    };
    eframe::run_native("Lycus", options, Box::new(|_cc| Box::<Lycus>::default()))
}

#[derive(Default)]
pub struct Lycus {
    // general
    selected_tab: usize,
    settings_open: bool,
    // Ares GUI
    ciphertext: String,
    plaintext: String,
    path: String,
    timeout: u32,
    ares_promise: Option<Promise<(String, String)>>,
    // lemmeknow GUI
    text_input: String,
    lemmeknow_name: String,
    lemmeknow_description: String,
    lemmeknow_tags: String,
    lemmeknow_rarity: String,
    lemmeknow_promise: Option<Promise<Vec<lemmeknow::Match>>>,
}

impl Lycus {
    pub fn new() -> Self {
        Self {
            // general
            selected_tab: 0,
            settings_open: false,
            // Ares GUI
            ciphertext: String::new(),
            plaintext: String::new(),
            path: String::new(),
            timeout: 10,
            ares_promise: None,
            // lemmeknow GUI
            text_input: String::new(),
            lemmeknow_name: String::new(),
            lemmeknow_description: String::new(),
            lemmeknow_tags: String::new(),
            lemmeknow_rarity: String::new(),
            lemmeknow_promise: None,
        }
    }
}

impl eframe::App for Lycus {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.horizontal(|ui| {
                let mut selected_tab = self.selected_tab;

                // Ares tab
                if ui.selectable_label(selected_tab == 0, "Ares").clicked() {
                    selected_tab = 0;
                }

                // Lemmeknow tab
                if ui
                    .selectable_label(selected_tab == 1, "Lemmeknow")
                    .clicked()
                {
                    selected_tab = 1;
                }

                self.selected_tab = selected_tab;
            });

            // Show Ares tab contents if it's selected
            if self.selected_tab == 0 {
                // Add the settings cog button to the top-right corner of the window
                ui.horizontal(|ui| {
                    ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                        // blue button
                        // ui.style_mut().visuals.widgets.inactive.weak_bg_fill = Color32::BLUE;
                        if ui.add(egui::widgets::Button::new("⛭")).clicked() {
                            self.settings_open = true;
                        }
                    });
                });

                // Open a new window with settings
                if self.settings_open {
                    let settings_window = egui::Window::new("Settings")
                        .resizable(true)
                        .open(&mut self.settings_open);
                    settings_window.show(ui.ctx(), |ui| {
                        // The timeout threshold before Ares quits
                        // This is in seconds
                        ui.add(egui::Slider::new(&mut self.timeout, 0..=100).text("Timeout"));

                        ui.separator();
                    });
                }

                ui.separator();

                // call ares gui and pass the struct
                ares_gui::ares_gui(self, ui);
            }

            // Show Lemmeknow tab contents if it's selected
            if self.selected_tab == 1 {
                ui.separator();

                // call lemmeknow gui and pass the struct
                lemmeknow_gui::lemmeknow_gui(self, ui);
            }
        });
    }
}

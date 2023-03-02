#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

use std::fs::OpenOptions;
use std::io::prelude::*;
use std::time::{SystemTime, UNIX_EPOCH};

use eframe::egui;

fn main() -> Result<(), eframe::Error> {
    // Log to stdout (if you run with `RUST_LOG=debug`).
    tracing_subscriber::fmt::init();

    let options = eframe::NativeOptions {
        initial_window_size: Some(egui::vec2(320.0, 240.0)),
        ..Default::default()
    };
    eframe::run_native(
        "Personal Log",
        options,
        Box::new(|_cc| Box::new(MyApp::default())),
    )
}

struct MyApp {
    location: String,
    log: String,
    todo: Vec<String>,
}

impl Default for MyApp {
    fn default() -> Self {
        Self {
            location: "~/Documents/personal-log/personal-log.txt".to_owned(),
            log: "".to_owned(),
            todo: Vec::new(),
        }
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            let mut file = OpenOptions::new()
                .append(true)
                .open(self.location.to_owned())
                .unwrap();

            let start = SystemTime::now();
            let since_the_epoch = start
                .duration_since(UNIX_EPOCH)
                .expect("Time went backwards");

            ui.heading("Personal Log");
            ui.horizontal(|ui| {
                let name_label = ui.label("Log: ");
                let re = ui.text_edit_singleline(&mut self.log)
                    .labelled_by(name_label.id);

                if re.lost_focus() {
                    if let Err(e) = writeln!(file, "{} log:{}", since_the_epoch.as_millis(), self.log) {
                        eprintln!("Couldn't write to file: {}", e);
                    }
                    self.log = "".to_owned();
                }
            });
            ui.horizontal(|ui| {
                let todo_label = ui.label("Todo: ");
                let todo = "";
                let re = ui.text_edit_singleline(&mut todo.to_owned())
                    .labelled_by(todo_label.id);

                if re.lost_focus() {
                    if let Err(e) = writeln!(file, "{} todo:{}", since_the_epoch.as_millis(), todo) {
                        eprintln!("Couldn't write to file: {}", e);
                    }
                    self.todo.push(todo.to_owned())
                }
            });

            ui.horizontal(|ui| {
                if ui.button("idle").clicked() {
                    if let Err(e) = writeln!(file, "{} status:idle", since_the_epoch.as_millis()) {
                        eprintln!("Couldn't write to file: {}", e);
                    }
                };
                if ui.button("code").clicked() {
                    if let Err(e) = writeln!(file, "{} status:code", since_the_epoch.as_millis()) {
                        eprintln!("Couldn't write to file: {}", e);
                    }
                };
                if ui.button("meeting").clicked() {
                    if let Err(e) = writeln!(file, "{} status:meeting", since_the_epoch.as_millis()) {
                        eprintln!("Couldn't write to file: {}", e);
                    }
                };
            });
            ui.horizontal(|ui| {
                if ui.button("research").clicked() {
                    if let Err(e) = writeln!(file, "{} status:research", since_the_epoch.as_millis()) {
                        eprintln!("Couldn't write to file: {}", e);
                    }
                };
                if ui.button("adhoc").clicked() {
                    if let Err(e) = writeln!(file, "{} status:adhoc", since_the_epoch.as_millis()) {
                        eprintln!("Couldn't write to file: {}", e);
                    }
                };
                if ui.button("documenting").clicked() {
                    if let Err(e) = writeln!(file, "{} status:documenting", since_the_epoch.as_millis()) {
                        eprintln!("Couldn't write to file: {}", e);
                    }
                };
            });
        });
    }
}
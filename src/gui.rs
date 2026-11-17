//! Minimal egui-based GUI front-end (optional, `--features gui`).
#![cfg(feature = "gui")]

use eframe::egui;

pub struct BurnDiskApp {
    pub files: Vec<String>,
    pub disc: String,
}

impl Default for BurnDiskApp {
    fn default() -> Self {
        Self {
            files: Vec::new(),
            disc: "dvd4.7".into(),
        }
    }
}

impl eframe::App for BurnDiskApp {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        ui.heading("BurnDisk");
        ui.label("Rust disc burning & ISO creation");
        ui.separator();
        ui.label(format!("Disc type: {}", self.disc));
        ui.label(format!("Selected files: {}", self.files.len()));
        if ui.button("Enumerate drives").clicked() {
            if let Ok(ds) = crate::drives::enumerate_drives() {
                for d in ds {
                    self.files.push(d.id);
                }
            }
        }
    }
}

pub fn run_gui() -> eframe::Result<()> {
    let options = eframe::NativeOptions::default();
    eframe::run_native(
        "BurnDisk",
        options,
        Box::new(|_cc| Ok(Box::new(BurnDiskApp::default()))),
    )
}
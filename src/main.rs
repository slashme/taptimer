use eframe::egui;
//use egui::{Label, FontId};
use std::time::Instant;

struct TapApp {
    taps: Vec<f64>,          // tap timestamps (seconds)
    start: Option<Instant>,  // reference start time
    bpm: Option<f64>,
    ci_low: Option<f64>,
    ci_high: Option<f64>,
}

impl Default for TapApp {
    fn default() -> Self {
        Self {
            taps: Vec::new(),
            start: None,
            bpm: None,
            ci_low: None,
            ci_high: None,
        }
    }
}

impl TapApp {
    fn reset(&mut self) {
        *self = Self::default();
    }

    fn register_tap(&mut self) {
        let now = Instant::now();

        let t = match self.start {
            Some(start) => now.duration_since(start).as_secs_f64(),
            None => {
                self.start = Some(now);
                0.0
            }
        };

        self.taps.push(t);

        if self.taps.len() >= 2 {
            self.compute_stats();
        }
    }

    fn compute_stats(&mut self) {
        let intervals: Vec<f64> = self
            .taps
            .windows(2)
            .map(|w| w[1] - w[0])
            .collect();

        let n = intervals.len() as f64;
        let mean = intervals.iter().sum::<f64>() / n;

        let var = intervals
            .iter()
            .map(|x| (x - mean).powi(2))
            .sum::<f64>()
            / (n - 1.0).max(1.0);

        let std = var.sqrt();

        let bpm = 60.0 / mean;
        self.bpm = Some(bpm);

        if intervals.len() >= 2 {
            let se = std / n.sqrt();
            let ci_mean_low = mean - 1.96 * se;
            let ci_mean_high = mean + 1.96 * se;

            self.ci_low = Some(60.0 / ci_mean_high);
            self.ci_high = Some(60.0 / ci_mean_low);
        }
    }
}

impl eframe::App for TapApp {
    fn update(&mut self, ctx: &egui::Context, _: &mut eframe::Frame) {
        ctx.set_pixels_per_point(2.0);
        if ctx.input(|i| i.key_pressed(egui::Key::Space)) {
            self.register_tap();
        }

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.vertical(|ui| {
                if ui.button("Reset").clicked() {
                    self.reset();
                }

                ui.add_space(20.0);

                ui.label(format!(
                        "Lower 95% CI: {:.2}",
                        self.ci_low.unwrap_or(0.0)
                ));
                ui.label(egui::RichText::new(format!(
                            "BPM: {:.2}",
                            self.bpm.unwrap_or(0.0)
                ))
                    .color(egui::Color32::WHITE)
                );
                ui.label(format!(
                    "Upper 95% CI: {:.2}",
                    self.ci_high.unwrap_or(0.0)
                ));
            });
        });
    }
}

fn main() -> eframe::Result<()> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([300.0, 200.0]),
        ..Default::default()
    };
    eframe::run_native(
        "Tap BPM",
        options,
        Box::new(|_cc| Box::new(TapApp::default())),
    )
}

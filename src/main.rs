use std::io::{self};
use std::time::Instant;

struct TapApp {
    taps: Vec<f64>,         // tap timestamps (seconds)
    start: Option<Instant>, // reference start time
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
        let t = match &self.start {
            Some(start) => start.elapsed().as_secs_f64(),
            None => {
                self.start = Some(Instant::now());
                0.0
            }
        };

        self.taps.push(t);

        if self.taps.len() >= 2 {
            self.compute_stats();
        }
    }

    fn compute_stats(&mut self) {
        let intervals: Vec<f64> = self.taps.windows(2).map(|w| w[1] - w[0]).collect();

        let n = intervals.len() as f64;
        let mean = intervals.iter().sum::<f64>() / n;

        let var = intervals.iter().map(|x| (x - mean).powi(2)).sum::<f64>() / (n - 1.0).max(1.0);

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

    fn print_stats(&self) {
        if let Some(bpm) = self.bpm {
            print!("\nBPM: {:.2}", bpm);
            if let (Some(ci_low), Some(ci_high)) = (self.ci_low, self.ci_high) {
                println!(" ({:.2} - {:.2})", ci_low, ci_high);
            } else {
                println!();
            }
        }
    }
}

fn main() {
    let mut app = TapApp::default();
    let stdin = io::stdin();
    let mut input_buffer = String::new();

    println!("-------- BPM counter --------");
    println!("Press ENTER to tap.");
    println!("Type 'r' then ENTER to reset.");
    println!("Type 'q' then ENTER to quit.");
    println!("-----------------------------");

    loop {
        input_buffer.clear();
        stdin
            .read_line(&mut input_buffer)
            .expect("Read line failed");
        let command = input_buffer.trim().to_lowercase();

        match command.as_str() {
            command if command.starts_with("q") => {
                break;
            }
            command if command.starts_with("r") => {
                app.reset();
            }
            _ => {
                app.register_tap();
                app.print_stats();
            }
        }
    }
}

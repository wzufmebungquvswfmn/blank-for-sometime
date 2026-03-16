use eframe::egui;
use std::time::{Duration, Instant};
use rand::Rng;

fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([500.0, 450.0])
            .with_resizable(true)
            .with_title("Meditation Timer"),
        ..Default::default()
    };

    eframe::run_native(
        "Meditation Timer",
        options,
        Box::new(|_cc| Box::new(MeditationApp::default())),
    )
}

struct MeditationApp {
    state: AppState,
    duration_minutes: String,
    start_time: Option<Instant>,
    duration: Option<Duration>,
    
    // Display options
    show_countdown: bool,
    show_random_words: bool,
    animate_words: bool,  // Whether words should change positions (off by default)
    words_input: String,
    
    // Random words state
    word_displays: Vec<WordDisplay>,
    last_word_update: Option<Instant>,
}

#[derive(PartialEq, Clone, Copy)]
enum AppState {
    Input,
    Meditating,
    Finished,
}

struct WordDisplay {
    word: String,
    x: f32,
    y: f32,
    size: f32,
    rotation: f32,  // in radians
    color: egui::Color32,
}

impl Default for MeditationApp {
    fn default() -> Self {
        Self {
            state: AppState::Input,
            duration_minutes: String::from("15"),
            start_time: None,
            duration: None,
            show_countdown: true,
            show_random_words: false,
            animate_words: false,
            words_input: String::from("simplify, restatement, breathe, focus, calm, peace, clarity, mindfulness"),
            word_displays: Vec::new(),
            last_word_update: None,
        }
    }
}

impl eframe::App for MeditationApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        match self.state {
            AppState::Input => {
                self.show_input_screen(ctx);
            }
            AppState::Meditating => {
                self.show_meditation_screen(ctx);
            }
            AppState::Finished => {
                self.show_finished_screen(ctx);
            }
        }
    }
}

impl MeditationApp {
    fn show_input_screen(&mut self, ctx: &egui::Context) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.vertical_centered(|ui| {
                ui.add_space(30.0);
                ui.heading("Meditation Timer");
                ui.add_space(20.0);

                // Duration input
                ui.label("Enter duration in minutes:");
                ui.add_space(5.0);

                let text_edit = egui::TextEdit::singleline(&mut self.duration_minutes)
                    .desired_width(100.0)
                    .font(egui::TextStyle::Heading)
                    .horizontal_align(egui::Align::Center);
                ui.add(text_edit);

                ui.add_space(20.0);
                ui.separator();
                ui.add_space(15.0);

                // Display options
                ui.label(egui::RichText::new("Display Options:").strong());
                ui.add_space(10.0);

                ui.checkbox(&mut self.show_countdown, "Show countdown timer");
                ui.checkbox(&mut self.show_random_words, "Show random words");

                // Words input (only show when random words is enabled)
                if self.show_random_words {
                    ui.add_space(5.0);
                    ui.checkbox(&mut self.animate_words, "Animate words (change positions)");
                    
                    ui.add_space(5.0);
                    ui.label("Enter words (comma-separated):");
                    ui.add_space(5.0);
                    
                    egui::ScrollArea::vertical()
                        .max_height(80.0)
                        .show(ui, |ui| {
                            let text_edit = egui::TextEdit::multiline(&mut self.words_input)
                                .desired_width(f32::INFINITY)
                                .desired_rows(3);
                            ui.add(text_edit);
                        });
                }

                ui.add_space(20.0);
                ui.separator();
                ui.add_space(15.0);

                // Start button
                if ui.button("Start Meditation").clicked() {
                    if let Ok(minutes) = self.duration_minutes.parse::<u64>() {
                        self.duration = Some(Duration::from_secs(minutes * 60));
                        self.start_time = Some(Instant::now());
                        self.state = AppState::Meditating;
                        
                        // Initialize random words if enabled
                        if self.show_random_words {
                            self.initialize_word_displays();
                            self.last_word_update = Some(Instant::now());
                        }
                        
                        // Set fullscreen
                        ctx.send_viewport_cmd(egui::ViewportCommand::Fullscreen(true));
                    }
                }

                ui.add_space(15.0);
                ui.small("Press ESC during meditation to exit early");
            });
        });
    }

    fn initialize_word_displays(&mut self) {
        self.word_displays.clear();
        let words: Vec<&str> = self.words_input
            .split(',')
            .map(|s| s.trim())
            .filter(|s| !s.is_empty())
            .collect();
        
        if words.is_empty() {
            return;
        }

        let mut rng = rand::thread_rng();
        
        // Create word displays - one for each word with fixed position
        // When animate_words is off, all words are displayed at once in fixed positions
        // When animate_words is on, we show 5 words that change over time
        if self.animate_words {
            // Show 5 random words initially
            for _ in 0..5 {
                if let Some(word) = words.get(rng.gen_range(0..words.len())) {
                    self.word_displays.push(create_random_word_display(word.to_string(), &mut rng));
                }
            }
        } else {
            // Show all words with fixed positions
            for word in &words {
                self.word_displays.push(create_random_word_display(word.to_string(), &mut rng));
            }
        }
    }

    fn show_meditation_screen(&mut self, ctx: &egui::Context) {
        // Check for ESC key to exit early
        if ctx.input(|i| i.key_pressed(egui::Key::Escape)) {
            ctx.send_viewport_cmd(egui::ViewportCommand::Fullscreen(false));
            self.state = AppState::Finished;
            return;
        }

        // Check if time is up
        if let (Some(start), Some(duration)) = (self.start_time, self.duration) {
            let elapsed = start.elapsed();
            if elapsed >= duration {
                ctx.send_viewport_cmd(egui::ViewportCommand::Fullscreen(false));
                self.state = AppState::Finished;
                return;
            }
        }

        // Update random words periodically (only if animate_words is enabled)
        if self.show_random_words && self.animate_words {
            if let Some(last_update) = self.last_word_update {
                if last_update.elapsed() >= Duration::from_secs(3) {
                    self.update_word_displays();
                    self.last_word_update = Some(Instant::now());
                }
            }
        }

        // Show white blank screen with optional displays
        egui::CentralPanel::default().show(ctx, |ui| {
            // Fill entire screen with white
            let rect = ui.available_rect_before_wrap();
            ui.painter().rect_filled(rect, 0.0, egui::Color32::WHITE);

            // Draw random words if enabled
            if self.show_random_words {
                for word_display in &self.word_displays {
                    // Calculate position within the visible area
                    let screen_width = rect.width();
                    let screen_height = rect.height();
                    let x = rect.min.x + word_display.x * screen_width;
                    let y = rect.min.y + word_display.y * screen_height;
                    
                    let galley = ui.painter().layout_no_wrap(
                        word_display.word.clone(),
                        egui::FontId::proportional(word_display.size),
                        word_display.color,
                    );
                    
                    let text_pos = egui::Pos2::new(x, y);
                    
                    // Draw the text with rotation
                    ui.painter().add(egui::epaint::TextShape::new(
                        text_pos,
                        galley,
                        word_display.color,
                    ).with_angle(word_display.rotation));
                }
            }

            // Show remaining time if countdown is enabled
            if self.show_countdown {
                if let (Some(start), Some(duration)) = (self.start_time, self.duration) {
                    let elapsed = start.elapsed();
                    let remaining = if duration > elapsed {
                        duration - elapsed
                    } else {
                        Duration::ZERO
                    };
                    let remaining_secs = remaining.as_secs();
                    let mins = remaining_secs / 60;
                    let secs = remaining_secs % 60;
                    let time_text = format!("{:02}:{:02}", mins, secs);

                    ui.vertical(|ui| {
                        ui.with_layout(egui::Layout::bottom_up(egui::Align::Center), |ui| {
                            ui.add_space(20.0);
                            ui.label(
                                egui::RichText::new(time_text)
                                    .font(egui::FontId::proportional(40.0))
                                    .color(egui::Color32::from_rgb(200, 200, 200))
                            );
                            ui.add_space(10.0);
                            ui.label(
                                egui::RichText::new("Press ESC to exit")
                                    .color(egui::Color32::from_rgb(180, 180, 180))
                            );
                            ui.add_space(20.0);
                        });
                    });
                }
            }
        });

        // Request continuous repaint for timer update and animations
        ctx.request_repaint();
    }

    fn update_word_displays(&mut self) {
        let words: Vec<&str> = self.words_input
            .split(',')
            .map(|s| s.trim())
            .filter(|s| !s.is_empty())
            .collect();
        
        if words.is_empty() {
            return;
        }

        let mut rng = rand::thread_rng();
        
        // Remove a random word and add a new one
        if !self.word_displays.is_empty() {
            let remove_idx = rng.gen_range(0..self.word_displays.len());
            self.word_displays.remove(remove_idx);
        }
        
        if let Some(word) = words.get(rng.gen_range(0..words.len())) {
            self.word_displays.push(create_random_word_display(word.to_string(), &mut rng));
        }
    }

    fn show_finished_screen(&mut self, ctx: &egui::Context) {
        // Reset word displays
        self.word_displays.clear();
        
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.vertical_centered(|ui| {
                ui.add_space(60.0);
                ui.heading("✨ Meditation Complete ✨");
                ui.add_space(30.0);

                ui.label("Well done! You've completed your meditation session.");
                ui.add_space(20.0);

                if ui.button("Start Another Session").clicked() {
                    self.state = AppState::Input;
                    self.start_time = None;
                    self.duration = None;
                    self.last_word_update = None;
                }

                ui.add_space(10.0);

                if ui.button("Exit").clicked() {
                    ctx.send_viewport_cmd(egui::ViewportCommand::Close);
                }
            });
        });
    }
}

fn create_random_word_display(word: String, rng: &mut rand::rngs::ThreadRng) -> WordDisplay {
    // Random position (normalized 0.0 to 1.0)
    let x = rng.gen_range(0.1..0.9);
    let y = rng.gen_range(0.1..0.8);  // Leave some space at bottom for timer
    
    // Random size
    let size = rng.gen_range(20.0..60.0);
    
    // Random rotation (-45 to 45 degrees in radians)
    let rotation = rng.gen_range(-0.785..0.785);  // -45° to 45°
    
    // Random light gray color
    let gray = rng.gen_range(150..200);
    let color = egui::Color32::from_rgb(gray, gray, gray);
    
    WordDisplay {
        word,
        x,
        y,
        size,
        rotation,
        color,
    }
}

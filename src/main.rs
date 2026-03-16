use rand::Rng;
use std::time::{Duration, Instant};

// ─────────────────────────────────────────────────────────────────────────────
// GUI mode (desktop – Windows / Linux / macOS)
// Build with: cargo build --release
// ─────────────────────────────────────────────────────────────────────────────
#[cfg(feature = "gui")]
use eframe::egui;

#[cfg(feature = "gui")]
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

#[cfg(feature = "gui")]
struct MeditationApp {
    state: AppState,
    duration_minutes: String,
    start_time: Option<Instant>,
    duration: Option<Duration>,

    // Display options
    show_countdown: bool,
    show_random_words: bool,
    animate_words: bool,
    words_input: String,

    // Random words state
    word_displays: Vec<WordDisplay>,
    last_word_update: Option<Instant>,
}

#[cfg(feature = "gui")]
#[derive(PartialEq, Clone, Copy)]
enum AppState {
    Input,
    Meditating,
    Finished,
}

#[cfg(feature = "gui")]
struct WordDisplay {
    word: String,
    x: f32,
    y: f32,
    size: f32,
    rotation: f32,
    color: egui::Color32,
}

#[cfg(feature = "gui")]
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
            words_input: String::from(
                "simplify, restatement, breathe, focus, calm, peace, clarity, mindfulness",
            ),
            word_displays: Vec::new(),
            last_word_update: None,
        }
    }
}

#[cfg(feature = "gui")]
impl eframe::App for MeditationApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        match self.state {
            AppState::Input => self.show_input_screen(ctx),
            AppState::Meditating => self.show_meditation_screen(ctx),
            AppState::Finished => self.show_finished_screen(ctx),
        }
    }
}

#[cfg(feature = "gui")]
impl MeditationApp {
    fn show_input_screen(&mut self, ctx: &egui::Context) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.vertical_centered(|ui| {
                ui.add_space(30.0);
                ui.heading("Meditation Timer");
                ui.add_space(20.0);

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

                ui.label(egui::RichText::new("Display Options:").strong());
                ui.add_space(10.0);

                ui.checkbox(&mut self.show_countdown, "Show countdown timer");
                ui.checkbox(&mut self.show_random_words, "Show random words");

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

                if ui.button("Start Meditation").clicked() {
                    if let Ok(minutes) = self.duration_minutes.parse::<u64>() {
                        self.duration = Some(Duration::from_secs(minutes * 60));
                        self.start_time = Some(Instant::now());
                        self.state = AppState::Meditating;

                        if self.show_random_words {
                            self.initialize_word_displays();
                            self.last_word_update = Some(Instant::now());
                        }

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
        let words: Vec<&str> = self
            .words_input
            .split(',')
            .map(|s| s.trim())
            .filter(|s| !s.is_empty())
            .collect();

        if words.is_empty() {
            return;
        }

        let mut rng = rand::thread_rng();

        if self.animate_words {
            for _ in 0..5 {
                if let Some(word) = words.get(rng.gen_range(0..words.len())) {
                    self.word_displays
                        .push(gui_create_random_word(word.to_string(), &mut rng));
                }
            }
        } else {
            for word in &words {
                self.word_displays
                    .push(gui_create_random_word(word.to_string(), &mut rng));
            }
        }
    }

    fn show_meditation_screen(&mut self, ctx: &egui::Context) {
        if ctx.input(|i| i.key_pressed(egui::Key::Escape)) {
            ctx.send_viewport_cmd(egui::ViewportCommand::Fullscreen(false));
            self.state = AppState::Finished;
            return;
        }

        if let (Some(start), Some(duration)) = (self.start_time, self.duration) {
            let elapsed = start.elapsed();
            if elapsed >= duration {
                ctx.send_viewport_cmd(egui::ViewportCommand::Fullscreen(false));
                self.state = AppState::Finished;
                return;
            }
        }

        if self.show_random_words && self.animate_words {
            if let Some(last_update) = self.last_word_update {
                if last_update.elapsed() >= Duration::from_secs(3) {
                    self.update_word_displays();
                    self.last_word_update = Some(Instant::now());
                }
            }
        }

        egui::CentralPanel::default().show(ctx, |ui| {
            let rect = ui.available_rect_before_wrap();
            ui.painter()
                .rect_filled(rect, 0.0, egui::Color32::WHITE);

            if self.show_random_words {
                for wd in &self.word_displays {
                    let screen_width = rect.width();
                    let screen_height = rect.height();
                    let x = rect.min.x + wd.x * screen_width;
                    let y = rect.min.y + wd.y * screen_height;

                    let galley = ui.painter().layout_no_wrap(
                        wd.word.clone(),
                        egui::FontId::proportional(wd.size),
                        wd.color,
                    );

                    let text_pos = egui::Pos2::new(x, y);
                    ui.painter().add(
                        egui::epaint::TextShape::new(text_pos, galley, wd.color)
                            .with_angle(wd.rotation),
                    );
                }
            }

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
                                    .color(egui::Color32::from_rgb(200, 200, 200)),
                            );
                            ui.add_space(10.0);
                            ui.label(
                                egui::RichText::new("Press ESC to exit")
                                    .color(egui::Color32::from_rgb(180, 180, 180)),
                            );
                            ui.add_space(20.0);
                        });
                    });
                }
            }
        });

        ctx.request_repaint();
    }

    fn update_word_displays(&mut self) {
        let words: Vec<&str> = self
            .words_input
            .split(',')
            .map(|s| s.trim())
            .filter(|s| !s.is_empty())
            .collect();

        if words.is_empty() {
            return;
        }

        let mut rng = rand::thread_rng();

        if !self.word_displays.is_empty() {
            let remove_idx = rng.gen_range(0..self.word_displays.len());
            self.word_displays.remove(remove_idx);
        }

        if let Some(word) = words.get(rng.gen_range(0..words.len())) {
            self.word_displays
                .push(gui_create_random_word(word.to_string(), &mut rng));
        }
    }

    fn show_finished_screen(&mut self, ctx: &egui::Context) {
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

#[cfg(feature = "gui")]
fn gui_create_random_word(word: String, rng: &mut rand::rngs::ThreadRng) -> WordDisplay {
    let x = rng.gen_range(0.1..0.9);
    let y = rng.gen_range(0.1..0.8);
    let size = rng.gen_range(20.0..60.0);
    let rotation = rng.gen_range(-0.785..0.785);
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

// ─────────────────────────────────────────────────────────────────────────────
// TUI mode (terminal – works in Termux on Android, SSH sessions, etc.)
// Build with: cargo build --release --no-default-features --features tui
// Run with:   cargo run --no-default-features --features tui
// ─────────────────────────────────────────────────────────────────────────────
#[cfg(feature = "tui")]
use crossterm::{
    cursor,
    event::{self, Event, KeyCode},
    execute, queue,
    style::{Color, Print, SetBackgroundColor, SetForegroundColor},
    terminal::{self, ClearType},
};

#[cfg(feature = "tui")]
fn main() -> std::io::Result<()> {
    tui_run()
}

#[cfg(feature = "tui")]
struct TuiConfig {
    duration_minutes: u64,
    show_countdown: bool,
    show_random_words: bool,
    animate_words: bool,
    words: Vec<String>,
}

#[cfg(feature = "tui")]
struct TuiWord {
    word: String,
    col: u16,
    row: u16,
    gray: u8,
}

#[cfg(feature = "tui")]
fn tui_run() -> std::io::Result<()> {
    loop {
        let config = tui_gather_input()?;
        let completed = tui_meditation(&config)?;

        if completed {
            println!("\n✨ Meditation Complete ✨");
            println!("Well done! You've completed your meditation session.");
        } else {
            println!("\nMeditation ended early.");
        }

        print!("\nStart another session? [y/N]: ");
        use std::io::Write;
        std::io::stdout().flush()?;
        let mut answer = String::new();
        use std::io::BufRead;
        std::io::stdin().lock().read_line(&mut answer)?;
        if !answer.trim().eq_ignore_ascii_case("y") {
            break;
        }
    }
    Ok(())
}

#[cfg(feature = "tui")]
fn tui_gather_input() -> std::io::Result<TuiConfig> {
    use std::io::{BufRead, Write};

    println!("=== Meditation Timer ===\n");

    macro_rules! prompt {
        ($msg:expr) => {{
            print!($msg);
            std::io::stdout().flush()?;
            let mut buf = String::new();
            std::io::stdin().lock().read_line(&mut buf)?;
            buf.trim().to_string()
        }};
    }

    let raw = prompt!("Duration in minutes [15]: ");
    let duration_minutes = raw.parse::<u64>().unwrap_or(15);

    let raw = prompt!("Show countdown timer? [Y/n]: ");
    let show_countdown = !raw.eq_ignore_ascii_case("n");

    let raw = prompt!("Show random words? [y/N]: ");
    let show_random_words = raw.eq_ignore_ascii_case("y");

    let (animate_words, words) = if show_random_words {
        let raw = prompt!("Animate words (change positions every 3 s)? [y/N]: ");
        let animate = raw.eq_ignore_ascii_case("y");

        let raw = prompt!(
            "Words, comma-separated [breathe, focus, calm, peace, clarity]: "
        );
        let words: Vec<String> = if raw.is_empty() {
            ["breathe", "focus", "calm", "peace", "clarity", "simplify", "mindfulness"]
                .iter()
                .map(|s| s.to_string())
                .collect()
        } else {
            raw.split(',')
                .map(|s| s.trim().to_string())
                .filter(|s| !s.is_empty())
                .collect()
        };
        (animate, words)
    } else {
        (false, Vec::new())
    };

    println!("\nPress ENTER to begin...");
    std::io::stdout().flush()?;
    let mut buf = String::new();
    std::io::stdin().lock().read_line(&mut buf)?;

    Ok(TuiConfig {
        duration_minutes,
        show_countdown,
        show_random_words,
        animate_words,
        words,
    })
}

#[cfg(feature = "tui")]
fn tui_pick_word<'a>(words: &'a [String], rng: &mut rand::rngs::ThreadRng) -> &'a str {
    &words[rng.gen_range(0..words.len())]
}

#[cfg(feature = "tui")]
fn tui_init_words(config: &TuiConfig, cols: u16, rows: u16) -> Vec<TuiWord> {
    if config.words.is_empty() {
        return Vec::new();
    }
    let mut rng = rand::thread_rng();
    let count = if config.animate_words {
        5.min(config.words.len())
    } else {
        config.words.len()
    };
    (0..count)
        .map(|i| {
            let word = if config.animate_words {
                tui_pick_word(&config.words, &mut rng).to_string()
            } else {
                config.words[i].clone()
            };
            tui_make_word(word, cols, rows, &mut rng)
        })
        .collect()
}

#[cfg(feature = "tui")]
fn tui_make_word(word: String, cols: u16, rows: u16, rng: &mut rand::rngs::ThreadRng) -> TuiWord {
    // Reserve the bottom 4 rows for the countdown timer and hint line
    const TIMER_RESERVED_ROWS: u16 = 4;
    // Subtract the word length plus 1 column of right-edge padding
    const WORD_PADDING_COLS: u16 = 1;
    let usable_rows = rows.saturating_sub(TIMER_RESERVED_ROWS).max(1);
    let usable_cols = cols
        .saturating_sub(word.len() as u16 + WORD_PADDING_COLS)
        .max(1);
    let col = rng.gen_range(1..=usable_cols);
    let row = rng.gen_range(1..=usable_rows);
    let gray = rng.gen_range(100u8..180u8);
    TuiWord { word, col, row, gray }
}

#[cfg(feature = "tui")]
fn tui_render(
    stdout: &mut std::io::Stdout,
    config: &TuiConfig,
    words: &[TuiWord],
    start: Instant,
    total: Duration,
    cols: u16,
    rows: u16,
) -> std::io::Result<()> {
    use std::io::Write;

    // Fill the whole screen with white background
    queue!(
        stdout,
        terminal::Clear(ClearType::All),
        SetBackgroundColor(Color::White)
    )?;
    for r in 0..rows {
        queue!(
            stdout,
            cursor::MoveTo(0, r),
            Print(" ".repeat(cols as usize))
        )?;
    }

    // Draw random words
    if config.show_random_words {
        for w in words {
            queue!(
                stdout,
                cursor::MoveTo(w.col, w.row),
                SetForegroundColor(Color::Rgb {
                    r: w.gray,
                    g: w.gray,
                    b: w.gray,
                }),
                Print(&w.word)
            )?;
        }
    }

    // Draw countdown / hint at the bottom
    if config.show_countdown {
        let elapsed = start.elapsed();
        let remaining = if total > elapsed {
            total - elapsed
        } else {
            Duration::ZERO
        };
        let secs = remaining.as_secs();
        let time_text = format!("{:02}:{:02}", secs / 60, secs % 60);
        let hint = "ESC / q  to exit";

        let time_col = (cols / 2).saturating_sub((time_text.len() / 2) as u16);
        let hint_col = (cols / 2).saturating_sub((hint.len() / 2) as u16);

        queue!(
            stdout,
            cursor::MoveTo(time_col, rows.saturating_sub(3)),
            SetForegroundColor(Color::Rgb {
                r: 160,
                g: 160,
                b: 160,
            }),
            Print(&time_text),
            cursor::MoveTo(hint_col, rows.saturating_sub(2)),
            SetForegroundColor(Color::Rgb {
                r: 140,
                g: 140,
                b: 140,
            }),
            Print(hint)
        )?;
    }

    stdout.flush()
}

/// Returns `true` if the timer ran to completion, `false` if the user exited early.
#[cfg(feature = "tui")]
fn tui_meditation(config: &TuiConfig) -> std::io::Result<bool> {
    let mut stdout = std::io::stdout();

    terminal::enable_raw_mode()?;
    execute!(stdout, terminal::EnterAlternateScreen, cursor::Hide)?;

    let result = tui_meditation_loop(&mut stdout, config);

    // Always restore the terminal, regardless of errors
    let _ = execute!(stdout, terminal::LeaveAlternateScreen, cursor::Show);
    let _ = terminal::disable_raw_mode();

    result
}

#[cfg(feature = "tui")]
fn tui_meditation_loop(
    stdout: &mut std::io::Stdout,
    config: &TuiConfig,
) -> std::io::Result<bool> {
    let start = Instant::now();
    let total = Duration::from_secs(config.duration_minutes * 60);

    let (mut cols, mut rows) = terminal::size()?;
    let mut words = tui_init_words(config, cols, rows);
    let mut last_word_update = Instant::now();

    loop {
        let elapsed = start.elapsed();
        if elapsed >= total {
            tui_render(stdout, config, &words, start, total, cols, rows)?;
            return Ok(true);
        }

        tui_render(stdout, config, &words, start, total, cols, rows)?;

        // Non-blocking event poll (200 ms timeout keeps the timer smooth)
        if event::poll(Duration::from_millis(200))? {
            match event::read()? {
                Event::Key(key) => match key.code {
                    KeyCode::Esc | KeyCode::Char('q') => return Ok(false),
                    _ => {}
                },
                Event::Resize(new_cols, new_rows) => {
                    cols = new_cols;
                    rows = new_rows;
                    words = tui_init_words(config, cols, rows);
                    last_word_update = Instant::now();
                }
                _ => {}
            }
        }

        // Update animated words every 3 seconds
        if config.show_random_words
            && config.animate_words
            && last_word_update.elapsed() >= Duration::from_secs(3)
        {
            let mut rng = rand::thread_rng();
            if !words.is_empty() {
                let idx = rng.gen_range(0..words.len());
                words.remove(idx);
            }
            if !config.words.is_empty() {
                let word = tui_pick_word(&config.words, &mut rng).to_string();
                words.push(tui_make_word(word, cols, rows, &mut rng));
            }
            last_word_update = Instant::now();
        }
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// Compile-time guard: at least one feature must be enabled
// ─────────────────────────────────────────────────────────────────────────────
#[cfg(not(any(feature = "gui", feature = "tui")))]
compile_error!(
    "No UI feature selected. \
     Enable 'gui' (default, requires a display) or 'tui' (terminal, works in Termux)."
);

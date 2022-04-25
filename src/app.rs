mod board;
use eframe::{egui, epi};
use board::Board;

/// We derive Deserialize/Serialize so we can persist app state on shutdown.
#[cfg_attr(feature = "persistence", derive(serde::Deserialize, serde::Serialize))]
#[cfg_attr(feature = "persistence", serde(default))] // if we add new fields, give them default values when deserializing old state
pub struct App {
    board: Board,
    running: bool,
    filename: String,
    rect: Option<egui::Rect>,
}

impl Default for App {
    fn default() -> Self {
        let mut b = Board::new();
        b.generate_from_file("glider.txt");
        Self {
            board: b,
            running: false,
            filename: "".to_owned(),
            rect: None,
        }
    }
}

impl epi::App for App {
    fn name(&self) -> &str {
        "Game of Life"
    }

    /// Called once before the first frame.
    fn setup(
        &mut self,
        _ctx: &egui::Context,
        _frame: &epi::Frame,
        _storage: Option<&dyn epi::Storage>,
    ) {
        // Load previous app state (if any).
        // Note that you must enable the `persistence` feature for this to work.
        #[cfg(feature = "persistence")]
        if let Some(storage) = _storage {
            *self = epi::get_value(storage, epi::APP_KEY).unwrap_or_default()
        }
    }

    /// Called by the frame work to save state before shutdown.
    /// Note that you must enable the `persistence` feature for this to work.
    #[cfg(feature = "persistence")]
    fn save(&mut self, storage: &mut dyn epi::Storage) {
        epi::set_value(storage, epi::APP_KEY, self);
    }

    /// Called each time the UI needs repainting, which may be many times per second.
    /// Put your widgets into a `SidePanel`, `TopPanel`, `CentralPanel`, `Window` or `Area`.
    fn update(&mut self, ctx: &egui::Context, _frame: &epi::Frame) {
        ctx.request_repaint();
        egui::SidePanel::left("Menu").show(ctx, |ui| {
            ui.add(egui::Slider::new(&mut self.board.cell_size, 0.1..=50.0)
                   .step_by(0.1)
                   .orientation(egui::SliderOrientation::Horizontal)
                   .text("Cell Size"),
            );

            ui.add(egui::Slider::new(&mut self.board.fps, 1..=60)
                   .step_by(1.0)
                   .orientation(egui::SliderOrientation::Horizontal)
                   .text("FPS"),
            );
            self.board.update_speed();

            ui.add(egui::Slider::new(&mut self.board.x_axis, -1000..=1000)
                   .step_by(1.0)
                   .orientation(egui::SliderOrientation::Horizontal)
                   .text("X Axis"),
            );
            ui.add(egui::Slider::new(&mut self.board.y_axis, -1000..=1000)
                   .step_by(1.0)
                   .orientation(egui::SliderOrientation::Horizontal)
                   .text("Y Axis"),
            );
            ui.add(egui::Slider::new(&mut self.board.b_size, 10..=500)
                   .step_by(1.0)
                   .orientation(egui::SliderOrientation::Horizontal)
                   .text("Board Size"),
            );

            ui.horizontal(|ui| {
                if ui.add(egui::Button::new("Toggle")).clicked() {
                    self.running = !self.running;
                }
                if ui.add(egui::Button::new("Random")).clicked() {
                    self.board.generate_random();
                    self.board.center_cells(self.rect.unwrap());
                }
                if ui.add(egui::Button::new("Clean")).clicked() {
                    self.board.clean();
                }
            });

            ui.horizontal(|ui| {
                ui.add(egui::TextEdit::singleline(&mut self.filename)
                    .desired_width(150.0)
                );
                if ui.add(egui::Button::new("Load from file")).clicked() {
                    self.board.generate_from_file(&self.filename);
                    self.board.center_cells(self.rect.unwrap());
                }
            });
        });
        egui::CentralPanel::default().show(ctx, |ui| {
            let painter = egui::Painter::new(
                    ui.ctx().clone(),
                    ui.layer_id(),
                    ui.available_rect_before_wrap()
                );
            ui.expand_to_include_rect(painter.clip_rect());
            self.rect = Some(painter.clip_rect());
            let mut shapes = vec![egui::Shape::rect_filled(self.rect.unwrap(), egui::Rounding::none(), egui::Color32::WHITE)];
            self.board.generate_cells(&mut shapes, self.rect.unwrap());
            painter.extend(shapes);
            if self.running {
                self.board.update();
            }
        });
    }
}

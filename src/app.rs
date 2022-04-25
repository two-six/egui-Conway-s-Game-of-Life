mod board;
use eframe::{egui, epi};
use board::Board;

/// We derive Deserialize/Serialize so we can persist app state on shutdown.
#[cfg_attr(feature = "persistence", derive(serde::Deserialize, serde::Serialize))]
#[cfg_attr(feature = "persistence", serde(default))] // if we add new fields, give them default values when deserializing old state
pub struct App {
    board: Board,
    first_time: bool
}

impl Default for App {
    fn default() -> Self {
        let mut b = Board::new();
        b.generate_from_file("cap.txt");
        Self {
            board: b,
            first_time: true
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
            ui.label("<Options>");
        });
        egui::CentralPanel::default().show(ctx, |ui| {
            let painter = egui::Painter::new(
                    ui.ctx().clone(),
                    ui.layer_id(),
                    ui.available_rect_before_wrap()
                );
            ui.expand_to_include_rect(painter.clip_rect());
            let rect = painter.clip_rect();
            // println!("{},{}", rect.min.x as i32, rect.max.x as i32);
            let mut shapes = vec![egui::Shape::rect_filled(rect, egui::Rounding::none(), egui::Color32::WHITE)];
            if self.first_time {
                self.board.center_cells(rect);
                self.first_time = false;
            }
            self.board.generate_cells(&mut shapes, rect);
            painter.extend(shapes);
            self.board.update();
        });
    }
}

use std::collections::HashSet;
use eframe::egui::{Color32, Rect, Shape, vec2, Rounding};
use std::{time::Duration};
use std::fs;
use instant::Instant;
use rand::{Rng, thread_rng};

const NEIGHBOURHOOD: [(i32, i32); 8] = [
    (-1, -1),
    (-1, 0),
    (-1, 1),
    (1, -1),
    (1, 0),
    (1, 1),
    (0, -1),
    (0, 1),
];

#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub struct Pos(pub i32, pub i32);

pub struct Board {
    pub fps: u32,
    pub speed: u128,
    pub cell_size: f32,
    pub x_axis: i32,
    pub y_axis: i32,
    pub b_size: i32,
    cells: HashSet<Pos>,
    last_frame_time: Instant,
}

impl Board {
    pub fn new() -> Self {
        Self {
            fps: 10,
            speed: Board::fps_to_speed(10.0),
            cells: HashSet::new(),
            last_frame_time: Instant::now(),
            b_size: 75,
            cell_size: 0.0,
            x_axis: 0,
            y_axis: 0,
        }
    }

    pub fn neighbours(&self, p: &Pos) -> usize {
        let mut neighbours = 0;
        for step in NEIGHBOURHOOD {

            if self.cells.contains(&Pos(p.0+step.0, p.1+step.1)) {
                neighbours += 1;
            }
        }
        neighbours
    }

    pub fn generate_random(&mut self) {
        self.cells = HashSet::new();
        for x in 0..=self.b_size {
            for y in 0..=self.b_size {
                let ran = thread_rng().gen_range(1..=3);
                if ran == 1 {
                    self.cells.insert(Pos(x, y));
                }
            }
        }
    }

    pub fn clean(&mut self) {
        self.cells = HashSet::new();
    }

    pub fn fps_to_speed(fps: f32) -> u128 {
        Duration::new(0, (1000000000.0 / fps) as u32).as_millis()
    }

    pub fn update_speed(&mut self) {
        self.speed = Board::fps_to_speed(self.fps as f32);
    }

    pub fn update(&mut self) {
        let duration_since_last_frame = Instant::now().duration_since(self.last_frame_time);
        if duration_since_last_frame.as_millis().lt(&self.speed) {
            return;
        }
        let mut n_cells = HashSet::new();
        let mut checked = HashSet::new();
        for el in &self.cells {
            for step in NEIGHBOURHOOD {
                let xy = Pos(el.0+step.0, el.1+step.1);
                if !checked.contains(&xy) {
                    checked.insert(xy);
                    let n = self.neighbours(&xy);
                    if (n == 2 && self.cells.contains(&xy)) || n == 3 {
                        n_cells.insert(xy);
                    }
                }
            }
        }
        self.last_frame_time = Instant::now();
        self.cells = n_cells;
    }

    fn find_min(&self) -> (i32, i32) {
        let mut min_x = -1;
        let mut min_y = -1;
        for el in &self.cells {
            if min_x == -1 || el.0 < min_x {
                min_x = el.0;
            }
            if min_y == -1 || el.1 < min_y {
                min_y = el.1;
            }
        }
        (min_x, min_y)
    }

    fn find_max(&self) -> (i32, i32) {
        let mut max_x = -1;
        let mut max_y = -1;
        for el in &self.cells {
            if el.0 > max_x {
                max_x = el.0;
            }
            if el.1 > max_y {
                max_y = el.1;
            }
        }
        (max_x, max_y)
    }

    pub fn center_cells(&mut self, rect: Rect) {
        let (min_x, min_y) = self.find_min();
        let (max_x, max_y) = self.find_max();
        let mut elems_c = HashSet::new();
        if rect.max.x > rect.max.y {
            self.cell_size = ((rect.max.x-rect.min.x) as i32 / self.b_size) as f32;
        } else {
            self.cell_size = ((rect.max.y-rect.min.y) as i32 / self.b_size) as f32;
        }
        for el in &self.cells {
            elems_c.insert(Pos(self.b_size/2-(max_x-min_x)/2 + el.0, self.b_size/2-(max_y-min_y)/2 + el.1));
        }

        self.cells = elems_c;
    }

    pub fn generate_cells(&self, shapes: &mut Vec<Shape>, rect: Rect) {
        for c in &self.cells {
            shapes.push(Shape::rect_filled(
                Rect {
                    min: rect.min
                        + vec2(self.cell_size as f32 * c.0 as f32 - self.x_axis as f32, self.cell_size as f32 * c.1 as f32 - self.y_axis as f32),
                    max: rect.min
                        + vec2(self.cell_size as f32 * (c.0+1) as f32 - self.x_axis as f32, self.cell_size as f32 * (c.1+1) as f32 - self.y_axis as f32)
                },
                Rounding::none(),
                Color32::BLACK
            ));
        }
    }

    pub fn generate_from_file(&mut self, f: &str) {
        if fs::read_to_string(f).is_err() {
            println!("Error reading from file");
            return;
        };
        let contents = fs::read_to_string(f)
            .expect("Error reading from file");

        let mut x = HashSet::new();
        for (ind, l) in contents.split('\n').enumerate() {
           for (i, c) in l.chars().enumerate() {
               if c == '#' {
                   x.insert(Pos(i as i32, ind as i32));
               }
           }
        }
        self.cells = x;
    }
}

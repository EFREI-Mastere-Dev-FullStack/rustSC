use noise::{NoiseFn, Perlin};
use crate::utils::{get_char};

pub struct Map {
    data: Vec<Vec<char>>,
}

impl Map {
    pub fn new(width: usize, height: usize, seed: u32) -> Self {
        let perlin = Perlin::new(seed);
        let scale = 0.1;

        let mut map = vec![vec![' '; width]; height];

        for y in 0..height {
            for x in 0..width {
                let value = perlin.get([x as f64 * scale, y as f64 * scale, seed as f64]);
                map[y][x] = get_char(value);
            }
        }

        // Add a 7x7 ground place in the middle of the map
        let center_x = width / 2;
        let center_y = height / 2;
        let place_size = 7;
        let half_place = place_size / 2;

        for y in (center_y - half_place)..=(center_y + half_place) {
            for x in (center_x - half_place)..=(center_x + half_place) {
                if y < height && x < width {
                    map[y][x] = ' '; // Set as ground
                }
            }
        }

        Map { data: map }
    }

    pub fn get_cell(&self, x: usize, y: usize) -> Option<char> {
        if let Some(row) = self.data.get(y) {
            if let Some(&cell) = row.get(x) {
                return Some(cell);
            }
        }
        None
    }

    pub fn width(&self) -> usize {
        self.data[0].len()
    }

    pub fn height(&self) -> usize {
        self.data.len()
    }

    pub fn print_map(&self, robot_pos: (usize, usize)) {
        for (y, row) in self.data.iter().enumerate() {
            for (x, col) in row.iter().enumerate() {
                if (x, y) == robot_pos {
                    print!("R");
                } else {
                    print!("{}", col);
                }
            }
            println!();
        }
    }
}
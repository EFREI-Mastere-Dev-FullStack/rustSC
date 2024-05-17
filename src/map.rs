use noise::{NoiseFn, Perlin};
use crate::utils::{get_char, Terrain};
use rand::Rng;

pub struct Map {
    data: Vec<Vec<char>>,
}

impl Map {
    pub fn new(width: usize, height: usize, seed: u32) -> Self {
        let perlin = Perlin::new(seed);
        let scale = 0.1;
        let mut map = vec![vec![Terrain::Ground.to_char(); width]; height];

        for y in 0..height {
            for x in 0..width {
                let value = perlin.get([x as f64 * scale, y as f64 * scale, seed as f64]);
                map[y][x] = get_char(value);
            }
        }

        // Generate walls
        let mut rng = rand::thread_rng();

        // Generate resources
        let resource_density = 0.01;
        for _ in 0..(width as f64 * height as f64 * resource_density) as usize {
            let mut x;
            let mut y;
            loop {
                x = rng.gen_range(0..width);
                y = rng.gen_range(0..height);
                if not_near_a_wall_and_valid(width, height, x, y, &map) {
                    break;
                }
            }
            map[y][x] = Terrain::Resource.to_char();
        }

        // Generate energy
        let energy_density = 0.01;
        for _ in 0..(width as f64 * height as f64 * energy_density) as usize {
            let mut x;
            let mut y;
            loop {
                x = rng.gen_range(0..width);
                y = rng.gen_range(0..height);
                if not_near_a_wall_and_valid(width, height, x, y, &map) {
                    break;
                }
            }
            map[y][x] = Terrain::Energy.to_char();
        }

        let center_x = width / 2;
        let center_y = height / 2;
        let place_size = 7;
        let half_place = place_size / 2;

        for y in (center_y - half_place)..=(center_y + half_place) {
            for x in (center_x - half_place)..=(center_x + half_place) {
                if y < height && x < width {
                    map[y][x] = Terrain::Ground.to_char();
                }
            }
        }

        map[center_y][center_x] = '╔';
        map[center_y][center_x + 1] = '╗';
        map[center_y + 1][center_x] = '╚';
        map[center_y + 1][center_x + 1] = '╝';

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
                    print!("{}", Terrain::Robot.to_char());
                } else {
                    print!("{}", col);
                }
            }
            println!();
        }
    }
}

fn not_near_a_wall_and_valid(width: usize, height: usize, x: usize, y: usize, map: &Vec<Vec<char>>) -> bool {
    if !Terrain::Ground.is_char(Some(map[y][x])) {
        return false;
    }

    let mut surrounded_positions = vec![];
    if x > 0 { surrounded_positions.push((x - 1, y)); }
    if x < width - 1 { surrounded_positions.push((x + 1, y)); }
    if y > 0 { surrounded_positions.push((x, y - 1)); }
    if y < height - 1 { surrounded_positions.push((x, y + 1)); }
    if x > 0 && y > 0 { surrounded_positions.push((x - 1, y - 1)); }
    if x > 0 && y < height - 1 { surrounded_positions.push((x - 1, y + 1)); }
    if x < width - 1 && y > 0 { surrounded_positions.push((x + 1, y - 1)); }
    if x < width - 1 && y < height - 1 { surrounded_positions.push((x + 1, y + 1)); }

    for pos in surrounded_positions {
        if Terrain::Wall.is_char(Some(map[pos.1][pos.0])) {
            return false;
        }
    }
    true
}

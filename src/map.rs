use noise::{NoiseFn, Perlin};
use crate::utils::get_char;

pub fn generate_map(width: usize, height: usize, seed: u32) -> Vec<Vec<char>> {
    let perlin = Perlin::new(seed);
    let scale = 0.1;

    let mut map = vec![vec![' '; width]; height];

    for y in 0..height {
        for x in 0..width {
            let value = perlin.get([x as f64 * scale, y as f64 * scale, seed as f64]);
            map[y][x] = get_char(value);
        }
    }

    map
}

pub fn print_map(map: Vec<Vec<char>>) {
    for row in map {
        for col in row {
            print!("{}", col);
        }
        println!();
    }
}
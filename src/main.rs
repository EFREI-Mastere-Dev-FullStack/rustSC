extern crate noise;
extern crate rand;
mod utils;
mod map;

use rand::{Rng, thread_rng};
use map::generate_map;
use map::print_map;


fn main() {
    let width = 100;
    let height = 100;
    let mut rng = thread_rng();
    let seed: u32 = rng.gen();

    println!("Generating map with seed: {}", seed);
    let map = generate_map(width, height, seed);
    print_map(map);
}
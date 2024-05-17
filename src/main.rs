extern crate noise;
extern crate rand;
mod utils;
mod map;
mod robot;

use std::time::Duration;
use std::thread::sleep;
use rand::{Rng, thread_rng};
use map::Map;
use robot::move_robot;

fn main() {
    let width = 100;
    let height = 40;
    let mut rng = thread_rng();
    let seed: u32 = rng.gen();

    println!("Generating map with seed: {}", seed);
    let map: Map = Map::new(width, height, seed);
    let mut robot_pos = (width / 2, height / 2);

    loop {
        print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
        map.print_map(robot_pos);
        move_robot(&mut robot_pos, width, height, &map);
        sleep(Duration::from_millis(200));
    }
}
extern crate noise;
extern crate rand;
mod utils;
mod map;
mod robot;
mod e_map;
mod base;
mod terrain;

use std::io;
use std::time::Duration;
use std::thread::sleep;

use rand::{Rng, thread_rng};
use map::Map;
use robot::Robot;

// debug main
fn main() {
    let width = 80;
    let height = 40;

    /*println!("Entrez la seed :");

    let mut input = String::new();
    let mut seed: u32 = 0;
    io::stdin().read_line(&mut input).expect("Erreur lors de la lecture de l'entrée");

    if !input.trim().is_empty() {
        seed = input.trim().parse().expect("Entrée invalide");
    } else {
        let mut rng = thread_rng();
        seed = rng.gen();
    }*/
    let seed: u32 = 1521335673;
    println!("Generating map with seed: {}", seed);
    let mut map: Map = Map::new(width, height, seed);
    let robot: Robot = Robot::new(width / 2, height / 2, &mut map);
    map.add_robot(robot);
    let robot2: Robot = Robot::new(width / 2 +1, height / 2, &mut map);
    map.add_robot(robot2);
    let robot3: Robot = Robot::new(width / 2, height / 2+1, &mut map);
    map.add_robot(robot3);
    let robot4: Robot = Robot::new(width / 2+1, height / 2+1, &mut map);
    map.add_robot(robot4);
    map.update_known_maps();
    map.robots()[0].print_map(seed);
    loop {
        print!("{esc}[2J{esc}[1;1H", esc = 27 as char);

        map.print_map();
        //map.robots()[0].print_map(seed);
        map.move_robots();
        map.update_known_maps();

        sleep(Duration::from_millis(200));
    }
}

// core main
/*fn main() {
    let width = 80;
    let height = 40;

    println!("Entrez la seed :");

    let mut input = String::new();
    let mut seed: u32 = 0;
    io::stdin().read_line(&mut input).expect("Erreur lors de la lecture de l'entrée");

    if !input.trim().is_empty() {
        seed = input.trim().parse().expect("Entrée invalide");
    } else {
        let mut rng = thread_rng();
        seed = rng.gen();
    }

    println!("Generating map with seed: {}", seed);
    let mut map: Map = Map::new(width, height, seed);
    let robot: Robot = Robot::new(width / 2, height / 2, &mut map);
    map.add_robot(robot);
    let robot2: Robot = Robot::new(width / 2 +1, height / 2, &mut map);
    map.add_robot(robot2);
    let robot3: Robot = Robot::new(width / 2, height / 2+1, &mut map);
    map.add_robot(robot3);
    let robot4: Robot = Robot::new(width / 2+1, height / 2+1, &mut map);
    map.add_robot(robot4);
    map.update_known_maps();
    map.robots()[0].print_map(seed);
    loop {
        print!("{esc}[2J{esc}[1;1H", esc = 27 as char);

        map.print_map();
        map.move_robots();
        map.update_known_maps();

        sleep(Duration::from_millis(200));
    }
}*/
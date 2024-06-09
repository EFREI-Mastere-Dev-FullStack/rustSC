extern crate noise;
extern crate rand;
mod utils;
mod game;
mod robot;
mod map;
mod base;
mod terrain;

use std::io;
use std::time::Duration;
use std::thread::sleep;

use rand::{Rng, thread_rng};
use game::Game;
use robot::Robot;

// debug main
fn main() {
    /*println!("Enter the map seed:");

    let mut input = String::new();
    let mut seed: u32 = 0;
    io::stdin().read_line(&mut input).expect("Error while reading the input");

     if !input.trim().is_empty() {
        seed = input.trim().parse().expect("Invalid input");
    } else {
        let mut rng = thread_rng();
        seed = rng.gen();
    }

    println!("Enter the map height:");
    let mut input = String::new();
    let mut height: usize = 0;
    io::stdin().read_line(&mut input).expect("Error while reading the input");

     if !input.trim().is_empty() {
        height = input.trim().parse().expect("Invalid input");
    } else {
        height = 40;
    }

    println!("Enter the map width:");
    input = String::new();
    let mut width: usize = 0;
    io::stdin().read_line(&mut input).expect("Error while reading the input");

    if !input.trim().is_empty() {
        width = input.trim().parse().expect("Invalid input");
    } else {
        width = 80;
    }*/

    let seed: u32 = 1521335673;
    let width = 80;
    let height = 40;

    println!("Generating map with seed: {}", seed);
    let mut game: Game = Game::new(width, height, seed);
    let robot: Robot = Robot::new(width / 2, height / 2, &mut game);
    game.add_robot(robot);
    let robot2: Robot = Robot::new(width / 2 +1, height / 2, &mut game);
    game.add_robot(robot2);
    let robot3: Robot = Robot::new(width / 2, height / 2+1, &mut game);
    game.add_robot(robot3);
    let robot4: Robot = Robot::new(width / 2+1, height / 2+1, &mut game);
    game.add_robot(robot4);

    game.update_known_maps();

    loop {
        print!("{esc}[2J{esc}[1;1H", esc = 27 as char);

        //game.print_map();
        //game.robots()[0].print_map(seed);
        game.move_robots();
        game.update_known_maps();
        game.base.merge_map(&mut game.robots[0]);
        //game.base.print_merged_map(&mut game.robots);
        game.base.merge_map(&mut game.robots[1]);
        game.base.merge_map(&mut game.robots[2]);
        game.base.merge_map(&mut game.robots[3]);

        game.base.print_merged_map(&mut game.robots);
        //game.robots()[0].print_map(seed);

        sleep(Duration::from_millis(200));
    }
}
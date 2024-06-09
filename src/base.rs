use std::cmp::Ordering;
use crate::game::Game;
use crate::map::Map;
use crate::robot::{Position, Robot};
use crate::robot_type::Robot_type;
use crate::terrain::Terrain;

pub(crate) struct Base {
    pub(crate) ores: usize,
    pub(crate) energy: usize,
    pub(crate) science: usize,
    shared_map: Map,
    pub(crate) coordinates: Position
}

impl Base {
    pub fn new(width: usize, height: usize, center_x: usize, center_y: usize) -> Self {
        Base {ores: 0, energy: 0, science: 0, shared_map: Map::new(width, height, Terrain::Void), coordinates: Position {x: center_x, y: center_y}}
    }

    pub fn print_merged_map(&mut self, robots: &Vec<Robot>) {
        for (y, row) in self.shared_map.data.iter().enumerate() {
            for (x, col) in row.iter().enumerate() {
                let mut is_robot = false;
                for robot in robots {
                    if (x, y) == robot.position().as_tuple() {
                        let displayed_robot = if !robot.is_carrying() { Terrain::Robot.to_char() } else { Terrain::CarryingRobot.to_char() };
                        print!("{}", displayed_robot);
                        is_robot = true;
                        break;
                    }
                }
                if !is_robot {
                    print!("{}", col);
                }
            }

            if y == 0 {
                print!("   | Energy: {}, Ore: {}, Science: {}", self.energy, self.ores, self.science);
            }
            for (i, _) in robots.iter().enumerate() {
                if y < robots.len() + 1 {
                    if y == i + 1 {
                        print!("   | Mission: {}, Position: (x: {}, y: {}), Resource: {}, On: {}", robots[i].mission().to_string(), robots[i].position().x, robots[i].position().y, robots[i].resource().to_char(), &self.shared_map.get_cell(robots[i].position().x, robots[i].position().y).unwrap())
                    }
                }
            }
            println!();
        }
    }

    pub fn shared_map(&mut self) -> &mut Map {
        &mut self.shared_map
    }

    pub fn set_shared_map(&mut self, map: Map) {
        self.shared_map = map;
    }

    pub fn merge_map(&mut self, robot: &mut Robot) {
        let width = self.shared_map.width();
        let height = self.shared_map.height();
        let mut new_map = Map::new(width, height, Terrain::Void);

        for x in 0..height {
            for y in 0..width {
                let position = Position { y, x };
                let base_cell = self.shared_map.get_cell(y, x);
                let robot_cell = robot.known_map().get_cell(y, x);

                let cell = match (base_cell, robot_cell) {
                    (Some(b_cell), _) if !Terrain::Void.is_char(Some(b_cell)) => b_cell,
                    (_, Some(r_cell)) if !Terrain::Void.is_char(Some(r_cell)) => r_cell,
                    _ => Terrain::Void.to_char(),
                };

                new_map.set_cell(position, cell);
            }
        }

        self.set_shared_map(new_map.clone());
        robot.set_known_map(new_map);
    }

    pub fn merge_maps(&mut self, robots: &mut Vec<Robot>) {
        for robot in robots {
            self.merge_map(robot);
        }
    }

    pub fn create_robot(&mut self, game: &mut Game) {
        if self.energy >= 5 {
            let scout_count = game.count_robots(Robot_type::Scout);
            let harvester_count = game.count_robots(Robot_type::Harvester);
            let scientist_count = game.count_robots(Robot_type::Scientist);

            let robot_type = match (scout_count.cmp(&harvester_count), scout_count.cmp(&scientist_count), harvester_count.cmp(&scientist_count)) {
                (Ordering::Less, Ordering::Less, _) => Robot_type::Scout,
                (_, _, Ordering::Less) => Robot_type::Scientist,
                _ => Robot_type::Harvester,
            };

            let new_robot: Robot = Robot::new(self.coordinates.x, self.coordinates.y, robot_type, game);
            game.add_robot(new_robot);
            self.energy -= 5;
        }
    }

    pub fn release_energy_and_merge(&mut self, robot: &mut Robot) {
        if robot.is_on_base(self) {
            if robot.is_carrying() {
                match *robot.resource() {
                    Terrain::Ore => {self.ores += 1}
                    Terrain::Energy => {self.energy += 1}
                    Terrain::Science => {self.science += 1}
                    _ => {}
                }
                robot.set_resource(Terrain::Void);
            }
        }
        self.merge_map(robot);
    }

    pub fn add_ores(&mut self) {
        self.ores += 1;
    }

    pub fn add_energy(&mut self) {
        self.energy += 1;
    }

    pub fn add_science(&mut self) {
        self.science += 1;
    }
}
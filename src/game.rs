use std::cmp::PartialEq;
use noise::{NoiseFn, Perlin};
use crate::utils::{get_char};
use robot::Robot;
use crate::map::Map;
use crate::robot;
use crate::robot::Position;
use crate::terrain::Terrain;
use crate::base::Base;
use crate::robot_type::Robot_type;

pub struct Game {
    pub(crate) data: Map,
    pub(crate) robots: Vec<Robot>,
    pub(crate) base: Base,
    seed: u32
}

impl PartialEq for Robot_type {
    fn eq(&self, other: &Self) -> bool {
        self == other
    }
}

impl Game {
    pub fn new(width: usize, height: usize, seed: u32) -> Self {
        let perlin = Perlin::new(seed);
        let scale = 0.1;
        let mut map = Map::new(width, height, Terrain::Ground);

        for y in 0..height {
            for x in 0..width {
                let value = perlin.get([x as f64 * scale, y as f64 * scale, seed as f64]);
                map.set_cell(Position {x: y, y: x}, get_char(value));
            }
        }

        let center_x = width / 2;
        let center_y = height / 2;
        let place_size = 7;
        let half_place = place_size / 2;

        for y in (center_y - half_place)..=(center_y + half_place) {
            for x in (center_x - half_place)..=(center_x + half_place) {
                if y < height && x < width {
                    map.set_cell(Position {x: y, y: x}, Terrain::Ground.to_char());
                }
            }
        }

        map.set_cell(Position {x: center_y, y: center_x}, '╔');
        map.set_cell(Position {x: center_y, y: center_x + 1}, '╗');
        map.set_cell(Position {x: center_y + 1, y: center_x}, '╚');
        map.set_cell(Position {x: center_y + 1, y: center_x + 1}, '╝');

        Game {data: map, robots: Vec::new(), seed: seed, base: Base::new(width, height, center_y, center_x)}
    }

    pub fn add_robot(&mut self, robot: Robot) {
        self.robots.push(robot);
    }

    pub fn get_cell(&self, x: usize, y: usize) -> Option<char> {
        self.data.get_cell(x, y)
    }

    pub fn width(&self) -> usize {
        self.data.width()
    }

    pub fn height(&self) -> usize {
        self.data.height()
    }

    pub fn robots(&self) -> &Vec<Robot> {
        &self.robots
    }

    pub fn move_robots(&mut self) {
        let width = self.width();
        let height = self.height();
        for robot in &mut self.robots {
            robot.move_robot(width, height);
        }
    }

    pub fn update_known_maps(&mut self) {
        for robot in &mut self.robots {
            robot.update_known_map(&self.data);
        }
    }

    pub fn print_map(&self) {
        for (y, row) in self.data.data.iter().enumerate() {
            for (x, col) in row.iter().enumerate() {
                let mut is_robot = false;
                for robot in &self.robots {
                    if (x, y) == robot.position().as_tuple() {
                        let displayed_robot = if robot.is_carrying() { Terrain::Robot.to_char() } else { Terrain::CarryingRobot.to_char()};
                        print!("{}", displayed_robot);
                        is_robot = true;
                        break;
                    }
                }
                if !is_robot {
                    print!("{}", col);
                }
            }
            println!();
        }
        print!("{}", self.seed);
    }

    pub fn count_robots(&self, robot_type: Robot_type) -> usize {
        let mut count:usize = 0;
        for robot in self.robots {
            if robot.mission() == robot_type {
                count +=1;
            }
        }
        count
    }
}

fn not_near_a_wall_and_valid(width: usize, height: usize, x: usize, y: usize, map: &Map) -> bool {
    if !Terrain::Ground.is_char(map.get_cell(x, y)) {
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
        if Terrain::Wall.is_char(map.get_cell(pos.1, pos.0)) {
            return false;
        }
    }
    true
}
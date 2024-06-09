extern crate rand;

use std::cmp::PartialEq;
use rand::Rng;
use crate::base::Base;
use crate::map::Map;
use crate::game::Game;
use crate::robot_type::Robot_type;
use crate::terrain::Terrain;
use crate::pathfinding;

#[derive(Clone, Copy)]
pub struct Position {
    pub(crate) x: usize,
    pub(crate) y: usize
}

impl Position {
    pub fn new(x: usize, y: usize) -> Position {
        Position {x, y}
    }

    pub fn as_tuple(&self) -> (usize, usize) {
        (self.x, self.y)
    }
}

pub struct Robot {
    position: Position,
    pub(crate) known_map: Map,
    resource: Terrain,
    mission: Robot_type,
}

impl PartialEq for Terrain {
    fn eq(&self, other: &Self) -> bool {
        self.to_char() == other.to_char()
    }
}

impl Robot {
    pub fn new(x: usize, y: usize, mission: Robot_type, game: &mut Game) -> Robot {
        Robot {
            position: Position::new(x, y),
            known_map: Map::new(game.width(), game.height(), Terrain::Void),
            resource: Terrain::Void,
            mission: mission
        }
    }

    pub fn print_map(&self, seed: u32) {
        for (y, row)in self.known_map.data.iter().enumerate() {
            for (x, col) in row.iter().enumerate() {
                let mut is_robot = false;
                if (x, y) == self.position().as_tuple() {
                    print!("{}", Terrain::Robot.to_char());
                    is_robot = true;
                    break;
                }
                if !is_robot {
                    print!("{}", col);
                }
            }
            println!();
        }
        print!("{}", seed.to_string());
    }

    pub fn set_known_map(&mut self, map: Map) {
        self.known_map = map;
    }

    pub fn position(&self) -> &Position {
        &self.position
    }

    pub fn known_map(&mut self) -> &mut Map {
        &mut self.known_map
    }

    pub fn mission(&self) -> &Robot_type {
        &self.mission
    }

    pub fn set_mission(&mut self, mission: Robot_type) {
        self.mission = mission;
    }

    pub fn is_carrying(&self) -> bool {
        !(self.resource == Terrain::Void)
    }

    pub fn resource(&self) -> &Terrain {
        &self.resource
    }

    pub fn set_resource(&mut self, terrain: Terrain) {
        self.resource = terrain;
    }

    pub fn move_robot(&mut self, width: usize, height: usize) {
        let mut pos_is_ok: bool = false;
        while !pos_is_ok {
            let mut rng = rand::thread_rng();
            let direction = rng.gen_range(0..4);
            match direction {
                0 if self.position.x > 0 && self.can_move(self.position.x - 1, self.position.y) => {
                    self.position.x -= 1;
                    pos_is_ok = true;
                }
                1 if self.position.x < width - 1 && self.can_move(self.position.x + 1, self.position.y) => {
                    self.position.x += 1;
                    pos_is_ok = true;
                },
                2 if self.position.y > 0 && self.can_move(self.position.x, self.position.y - 1) => {
                    self.position.y -= 1;
                    pos_is_ok = true;
                },
                3 if self.position.y < height - 1 && self.can_move(self.position.x, self.position.y + 1) => {
                    self.position.y += 1;
                    pos_is_ok = true;
                },
                _ => {}
            }
        }
        self.on_resource();
    }

    fn can_move(&self, x: usize, y: usize) -> bool {
        !Terrain::Wall.is_char(self.known_map.get_cell(x, y))
            && !Terrain::Mountain.is_char(self.known_map.get_cell(x, y))
            && !Terrain::Robot.is_char(self.known_map.get_cell(x, y))
            && !Terrain::Void.is_char(self.known_map.get_cell(x, y))
    }

    fn on_resource(&mut self) {
        if !self.is_carrying()
            && (Terrain::Energy.is_char(self.known_map.get_cell(self.position().x, self.position().y))
            || Terrain::Ore.is_char(self.known_map.get_cell(self.position().x, self.position().y))) {
            self.take_resource();
        }
    }

    fn take_resource(&mut self) {
        if let Some(cell) = self.known_map.get_cell(self.position().y, self.position().x) {
            if !Some(cell).is_none() && !self.is_carrying() {
                self.set_resource(Terrain::from_char(cell));
                self.known_map.set_cell(Position {y: self.position().x, x: self.position().y}, Terrain::Ground.to_char());
            }
        }
    }

    pub(crate) fn is_on_base(&self, base: &mut Base) -> bool {
        if (self.position.x == base.coordinates.x || self.position.x == base.coordinates.x + 1)
            && (self.position.y == base.coordinates.y || self.position.y == base.coordinates.y + 1) {
            return true;
        }
        false
    }

    pub fn set_cell(&mut self, position: Position, val: char) {
        self.known_map.set_cell(position, val);
    }

    pub fn update_known_map(&mut self, map: &Map) {
        let radius = 3;

        let (robot_x, robot_y) = (self.position.x, self.position.y);

        for dy in -radius..=radius {
            for dx in -radius..=radius {
                let x = robot_x as isize + dx;
                let y = robot_y as isize + dy;

                let cell_value = map.get_cell(x as usize, y as usize);
                if !cell_value.is_none() {
                    self.set_cell(Position { x: y as usize, y: x as usize }, cell_value.unwrap());
                }
            }
        }
    }

    pub fn find_path(&self, start: (usize, usize), goal: (usize, usize)) -> Option<Vec<(usize, usize)>> {
        pathfinding::pathfind(self, start, goal)
    }
}

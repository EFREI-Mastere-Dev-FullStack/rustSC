extern crate rand;

use rand::Rng;
use crate::map::{EMap, Map};
use crate::terrain::Terrain;

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
    known_map: EMap,
    resource: bool,
}

impl Robot {
    pub fn new(x: usize, y: usize, map: &mut Map) -> Robot {
        Robot {
            position: Position::new(x, y),
            known_map: EMap::new(map.width(), map.height(), Terrain::Void),
            resource: false,
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

    pub fn position(&self) -> &Position {
        &self.position
    }

    pub fn known_map(&self) -> &EMap {
        &self.known_map
    }

    pub fn is_carrying(&self) -> bool {
        self.resource
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
    }

    fn can_move(&self, x: usize, y: usize) -> bool {
        !Terrain::Wall.is_char(self.known_map.get_cell(x, y))
            && !Terrain::Mountain.is_char(self.known_map.get_cell(x, y))
            && !Terrain::Robot.is_char(self.known_map.get_cell(x, y))
            && !Terrain::Void.is_char(self.known_map.get_cell(x, y))
    }

    pub fn set_cell(&mut self, position: Position, val: char) {
        self.known_map.set_cell(position, val);
    }

    pub fn update_known_map(&mut self, map: &EMap) {
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
}

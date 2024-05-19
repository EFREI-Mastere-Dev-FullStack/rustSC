use crate::map::Map;
use crate::robot::{Position, Robot};
use crate::terrain::Terrain;

pub(crate) struct Base {
    resources: usize,
    shared_map: Map
}

impl Base {
    pub fn new(width: usize, height: usize) -> Self {
        Base {resources: 0, shared_map: Map::new(width, height, Terrain::Void)}
    }

    pub fn print_merged_map(&mut self, robots: &Vec<Robot>) {
        for (y, row) in self.shared_map.data.iter().enumerate() {
            for (x, col) in row.iter().enumerate() {
                let mut is_robot = false;
                for robot in robots {
                    if (x, y) == robot.position().as_tuple() {
                        print!("{}", Terrain::Robot.to_char());
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


    pub fn add_resource(&mut self) {
        self.resources += 1;
    }
}
use std::collections::VecDeque;
use crate::game::SCIENCE_NEEDED;
use crate::map::Map;
use crate::robot::{Position, Robot};
use crate::robot_type::Robot_type;
use crate::terrain::Terrain;

pub struct Base {
    pub(crate) ores: usize,
    pub(crate) energy: usize,
    pub(crate) science: usize,
    pub(crate) shared_map: Map,
    pub(crate) coordinates: Position,
    pub(crate) science_queue: VecDeque<Position>,
    pub(crate) resource_queue: VecDeque<Position>,
    pub(crate) detected_resources: Vec<Position>
}

impl Base {
    pub fn new(width: usize, height: usize, center_x: usize, center_y: usize) -> Self {
        Base {
            ores: 0,
            energy: 0,
            science: 0,
            shared_map: Map::new(width, height, Terrain::Void),
            coordinates: Position {x: center_x, y: center_y},
            science_queue: VecDeque::new(),
            resource_queue: VecDeque::new(),
            detected_resources: Vec::new()
        }
    }

    pub fn print_merged_map(&mut self, robots: &Vec<Robot>, points: &mut usize) {
        print!("len r: {}", self.resource_queue.len());
        for i in 0..self.resource_queue.len() {
            print!("(x: {}, y: {}, r: {}), ", self.resource_queue[i].x, self.resource_queue[i].y, self.shared_map.get_cell(self.resource_queue[i].y, self.resource_queue[i].x).unwrap())
        }
        println!();
        print!("len s: {}", self.science_queue.len());
        for i in 0..self.science_queue.len() {
            print!("(x: {}, y: {}, r: {}), ", self.science_queue[i].x, self.science_queue[i].y, self.shared_map.get_cell(self.science_queue[i].y, self.science_queue[i].x).unwrap())
        }
        println!();
        for (y, row) in self.shared_map.data.iter().enumerate() {
            for (x, col) in row.iter().enumerate() {
                let mut is_robot = false;
                for robot in robots {
                    if (x, y) == robot.position().as_tuple() {
                        //let displayed_robot = if !robot.is_carrying() { Terrain::Robot.to_char() } else { Terrain::CarryingRobot.to_char() };
                        let mission = robot.mission().to_string();
                        let displayed_robot = if mission ==  Robot_type::Scout.to_string() {"S"} else if mission == Robot_type::Harvester.to_string() {"H"} else {"I"};
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
                print!("   | Points: {}, Energy: {}, Ore: {}, Science: {}", points, self.energy, self.ores, self.science);
            }
            for (i, _) in robots.iter().enumerate() {
                if y < robots.len() + 2 {
                    if y == i + 2 {
                        print!("   | Mission: {}, Position: (x: {}, y: {}), Resource: {}, Goal: x: {}, y: {}",
                               robots[i].mission().to_string(),
                               robots[i].position().x,
                               robots[i].position().y,
                               robots[i].resource().to_char(),
                               robots[i].goal().unwrap_or_else(|| Position {x: 0, y:0}).y,
                               robots[i].goal().unwrap_or_else(|| Position {x: 0, y:0}).x
                        )
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

    pub fn pop_science_queue(&mut self) -> Option<Position> {
        self.science_queue.pop_front()
    }

    pub fn pop_resource_queue(&mut self) -> Option<Position> {
        self.resource_queue.pop_front()
    }

    pub fn reset_data(&mut self) {
        self.science -= SCIENCE_NEEDED;
    }

    pub fn resource_queue(&self) -> &VecDeque<Position> {
        &self.resource_queue
    }

    pub fn science_queue(&self) -> &VecDeque<Position> {
        &self.science_queue
    }

    pub fn detected_resources(&self) -> &Vec<Position> {
        &self.detected_resources
    }

    pub fn merge_map(&mut self, robot: &mut Robot) {
        let width = self.shared_map.width();
        let height = self.shared_map.height();
        let mut new_map = Map::new(width, height, Terrain::Void);

        let mut i: isize = 0;
        while i < self.resource_queue.len() as isize {
            if let Some(cell) = self.shared_map.get_cell(self.resource_queue.get(i as usize).unwrap().y, self.resource_queue.get(i as usize).unwrap().x) {
                if cell == Terrain::Ground.to_char() {
                    self.resource_queue.remove(i as usize);
                    i -= 1;
                }
            }
            i += 1;
        }

        while i < self.science_queue.len() as isize {
            if let Some(cell) = self.shared_map.get_cell(self.science_queue.get(i as usize).unwrap().y, self.science_queue.get(i as usize).unwrap().x) {
                if cell == Terrain::Ground.to_char() {
                    self.science_queue.remove(i as usize);
                    i -= 1;
                }
            }
            i += 1;
        }

        for x in 0..height {
            for y in 0..width {
                let position = Position { y, x };
                let base_cell = self.shared_map.get_cell(y, x);
                let robot_cell = robot.known_map().get_cell(y, x);

                let cell = match (base_cell, robot_cell) {
                    (Some(b_cell), Some(r_cell)) if Terrain::Science.is_char(Some(b_cell)) && Terrain::Ground.is_char(Some(r_cell)) => r_cell,
                    (Some(b_cell), Some(r_cell)) if Terrain::Ground.is_char(Some(b_cell)) && Terrain::Science.is_char(Some(r_cell)) => b_cell,
                    (Some(b_cell), Some(r_cell)) if Terrain::Energy.is_char(Some(b_cell)) && Terrain::Ground.is_char(Some(r_cell)) => r_cell,
                    (Some(b_cell), Some(r_cell)) if Terrain::Ground.is_char(Some(b_cell)) && Terrain::Energy.is_char(Some(r_cell)) => b_cell,
                    (Some(b_cell), Some(r_cell)) if Terrain::Ore.is_char(Some(b_cell)) && Terrain::Ground.is_char(Some(r_cell)) => r_cell,
                    (Some(b_cell), Some(r_cell)) if Terrain::Ground.is_char(Some(b_cell)) && Terrain::Ore.is_char(Some(r_cell)) => b_cell,
                    (Some(b_cell), _) if !Terrain::Void.is_char(Some(b_cell)) => b_cell,
                    (_, Some(r_cell)) if !Terrain::Void.is_char(Some(r_cell)) => r_cell,
                    _ => Terrain::Void.to_char(),
                };
                new_map.set_cell(position, cell);
            }
        }

        for x in 0..height {
            for y in 0..width {
                if let Some(cell) = new_map.get_cell(y, x) {
                    let position: Position = Position {x, y};
                    if (Terrain::Energy.is_char(Some(cell)) || Terrain::Ore.is_char(Some(cell))) && !self.resource_queue.contains(&position) && !self.detected_resources.contains(&position) {
                        self.detected_resources.push(Position{x, y});
                        self.resource_queue.push_back(Position {x, y});
                    } else if Terrain::Science.is_char(Some(cell)) && !self.science_queue.contains(&position) && !self.detected_resources.contains(&position) {
                        self.detected_resources.push(Position{x, y});
                        self.science_queue.push_back(Position {x, y});
                    }
                }
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

    pub fn release_energy_and_merge(&mut self, robot: &mut Robot) {
        if robot.is_on_base(self)
        {
            match robot.mission() {
                Robot_type::Scout => {
                    robot.set_void_terrains_discovered(0);
                }
                _ => {
                    if robot.is_carrying() {
                        robot.set_goal(None);
                    }
                }
            }
            if robot.is_carrying() {
                match *robot.resource() {
                    Terrain::Ore => {self.ores += 1}
                    Terrain::Energy => {self.energy += 1}
                    Terrain::Science => {self.science += 1}
                    _ => {}
                }
                robot.set_resource(Terrain::Void);
            }
            self.merge_map(robot);
        }
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
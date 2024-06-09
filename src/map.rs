use crate::robot::Position;
use crate::terrain::Terrain;

#[derive(Clone)]
pub struct Map {
    width: usize,
    height: usize,
    pub(crate) data: Vec<Vec<char>>
}

impl Map {
    pub fn new(width: usize, height: usize, terrain: Terrain) -> Map {
        Map {width, height, data: vec![vec![terrain.to_char(); width]; height]}
    }

    pub fn get_cell(&self, x: usize, y: usize) -> Option<char> {
        if let Some(row) = self.data.get(y) {
            if let Some(&cell) = row.get(x) {
                return Some(cell);
            }
        }
        None
    }

    pub fn print(&self) {
        for (_, row) in self.data.iter().enumerate() {
            for (_, col) in row.iter().enumerate() {
                print!("{}", col);
            }
            println!();
        }
    }

    pub fn set_cell(&mut self, position: Position, val: char) {
        self.data[position.x][position.y] = val;
    }

    pub fn width(&self) -> usize {
        self.data[0].len()
    }

    pub fn height(&self) -> usize {
        self.data.len()
    }
}
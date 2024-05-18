use crate::robot::Position;
use crate::terrain::Terrain;

pub struct EMap {
    width: usize,
    height: usize,
    pub(crate) data: Vec<Vec<char>>
}

impl EMap {
    pub fn new(width: usize, height: usize, terrain: Terrain) -> EMap {
        EMap {width, height, data: vec![vec![terrain.to_char(); width]; height]}
    }

    pub fn get_cell(&self, x: usize, y: usize) -> Option<char> {
        if let Some(row) = self.data.get(y) {
            if let Some(&cell) = row.get(x) {
                return Some(cell);
            }
        }
        None
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
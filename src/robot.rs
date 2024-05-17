extern crate rand;

use rand::Rng;
use crate::utils::Terrain;
use crate::map::Map;

pub fn move_robot(pos: &mut (usize, usize), width: usize, height: usize, map: &Map) {
    let mut pos_is_ok: bool = false;
    while !pos_is_ok {
        let mut rng = rand::thread_rng();
        let direction = rng.gen_range(0..4);
        match direction {
            0 if pos.0 > 0 && !Terrain::Wall.is_char(map.get_cell(pos.0 - 1, pos.1)) => {
                pos.0 -= 1;
                pos_is_ok = true;
            }
            1 if pos.0 < width - 1  && !Terrain::Wall.is_char(map.get_cell(pos.0 + 1, pos.1)) => {
                pos.0 += 1;
                pos_is_ok = true;
            },
            2 if pos.1 > 0  && !Terrain::Wall.is_char(map.get_cell(pos.0, pos.1 - 1)) => {
                pos.1 -= 1;
                pos_is_ok = true;
            },
            3 if pos.1 < height - 1  && !Terrain::Wall.is_char(map.get_cell(pos.0, pos.1 + 1)) => {
                pos.1 += 1;
                pos_is_ok = true;
            },
            _ => {}
        }
    }
}


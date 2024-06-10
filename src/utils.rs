use crate::utils::terrain::Terrain;

pub mod map;
pub mod terrain;
pub mod robot_type;
pub mod pathfinding;

pub fn get_char(val: f64) -> char {
    match val.abs() {
        v if v > 0.0 && v < 0.005 => Terrain::Energy.to_char(),
        v if v > 0.005 && v < 0.01 => Terrain::Ore.to_char(),
        v if v > 0.01 && v < 0.014 => Terrain::Science.to_char(),
        v if v < 0.25 => Terrain::Ground.to_char(),
        v if v < 0.5 => Terrain::Wall.to_char(),
        _ => {Terrain::Mountain.to_char()}
    }
}

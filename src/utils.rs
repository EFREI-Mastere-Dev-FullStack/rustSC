use crate::terrain::Terrain;

pub fn get_char(val: f64) -> char {
    match val.abs() {
        v if v < 0.25 => Terrain::Ground.to_char(),
        v if v < 0.5 => Terrain::Wall.to_char(),
        _ => {Terrain::Mountain.to_char()}
    }
}
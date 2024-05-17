pub enum Terrain {
    Wall,
    Resource,
    Energy,
    Ground,
    Robot
}

impl Terrain {
    pub fn to_char(&self) -> char {
        match self {
            Terrain::Wall => '▓',
            Terrain::Resource => '♦',
            Terrain::Ground => ' ',
            Terrain::Energy => '♥',
            Terrain::Robot => '☻'
        }
    }

    pub fn is_char(&self, val: Option<char>) -> bool {
        if self.to_char() == val.unwrap() {
            return true;
        }
        false
    }
}

pub fn get_char(val: f64) -> char {
    match val.abs() {
        v if v < 0.25 => Terrain::Ground.to_char(),   // Ground 60%
        v if v < 0.50 => Terrain::Wall.to_char(),   // Wall 25%
        _ => {Terrain::Ground.to_char()}
    }
}
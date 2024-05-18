pub enum Terrain {
    Wall,
    Mountain,
    Resource,
    Energy,
    Ground,
    Robot,
    Void
}

impl Terrain {
    pub fn to_char(&self) -> char {
        match self {
            Terrain::Wall => '▒',
            Terrain::Mountain => '▓',
            Terrain::Resource => '♦',
            Terrain::Ground => ' ',
            Terrain::Energy => '♥',
            Terrain::Robot => '☻',
            Terrain::Void => ' ',
        }
    }

    pub fn from_char(val: char) -> Terrain {
        match val {
            '▒' => Terrain::Wall,
            '▓' => Terrain::Mountain,
            '♦' => Terrain::Resource,
            ' ' => Terrain::Ground,
            '♥' => Terrain::Energy,
            '☻' => Terrain::Robot,
            _ => Terrain::Void
        }
    }

    pub fn is_char(&self, val: Option<char>) -> bool {
        if !val.is_none() && self.to_char() == val.unwrap() {
            return true;
        }
        false
    }
}

pub fn get_char(val: f64) -> char {
    match val.abs() {
        v if v < 0.25 => Terrain::Ground.to_char(),
        v if v < 0.5 => Terrain::Wall.to_char(),
        _ => {Terrain::Mountain.to_char()}
    }
}
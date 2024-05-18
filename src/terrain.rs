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
            Terrain::Ground => ' ', // for debug then ' '
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
            '.' => Terrain::Ground, // for debug then ' '
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
pub enum Terrain {
    Wall,
    Mountain,
    Ore, // minerai en fr
    Energy,
    Ground,
    Robot,
    Science,
    CarryingRobot,
    Base,
    Void
}

impl Terrain {
    pub fn to_char(&self) -> char {
        match self {
            Terrain::Wall => '▒',
            Terrain::Mountain => '▓',
            Terrain::Ore => '♦',
            Terrain::Ground => ' ', // for debug then ' '
            Terrain::Energy => '♥',
            Terrain::Robot => '☻',
            Terrain::Science => '▲',
            Terrain::CarryingRobot => '☺',
            Terrain::Base => '╔',
            Terrain::Void => '.' // for debug then '.'
        }
    }

    pub fn from_char(val: char) -> Terrain {
        match val {
            '▒' => Terrain::Wall,
            '▓' => Terrain::Mountain,
            '♦' => Terrain::Ore,
            ' ' => Terrain::Ground, // for debug then ' '
            '♥' => Terrain::Energy,
            '☻' => Terrain::Robot,
            '▲' => Terrain::Science,
            '☺' => Terrain::CarryingRobot,
            '╔' => Terrain::Base,
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

impl PartialEq for Terrain {
    fn eq(&self, other: &Self) -> bool {
        self.to_char() == other.to_char()
    }
}
pub enum Terrain {
    Wall,
    Mountain,
    Ore, // minerai en fr
    Energy,
    Ground,
    Scientist,
    Harvester,
    Scout,
    Science,
    Base,
    Void
}

impl Terrain {
    pub fn to_char(&self) -> char {
        match self {
            Terrain::Wall => '▒',
            Terrain::Mountain => '▓',
            Terrain::Ore => '✧',             //
            Terrain::Ground => ' ',
            Terrain::Energy => '𐌔',
            Terrain::Scientist => '♝',       //
            Terrain::Harvester => '⛏',
            Terrain::Scout => '♞',
            Terrain::Science => '⚛',
            Terrain::Base => '╔',
            Terrain::Void => '⛆'
        }
    }

    pub fn from_char(val: char) -> Terrain {
        match val {
            '▒' => Terrain::Wall,
            '▓' => Terrain::Mountain,
            '✧' => Terrain::Ore,
            ' ' => Terrain::Ground,
            '𐌔' => Terrain::Energy,
            '♝' => Terrain::Scientist,
            '⛏' => Terrain::Harvester,
            '♞' => Terrain::Scout,
            '⚛' => Terrain::Science,
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
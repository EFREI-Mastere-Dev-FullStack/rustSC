pub enum Terrain {
    Wall,
    Resource,
    Ground,
}

impl Terrain {
    pub fn to_char(&self) -> char {
        match self {
            Terrain::Wall => '#',
            Terrain::Resource => '0',
            Terrain::Ground => ' ',
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
    let char_res = match val.abs() {
        v if v < 0.1 => Terrain::Ground,
        v if v < 0.2 => Terrain::Wall, // wall
        v if v < 0.6 => Terrain::Ground,
        v if v < 0.9 => Terrain::Resource,
        v if v <= 1.0 => Terrain::Ground,
        _ => panic!("unexpected value")
    };
    char_res.to_char()
}
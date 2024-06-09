pub enum Robot_type {
    Scout,
    Harvester,
    Scientist
}

impl Robot_type {
    pub fn to_string(&self) -> &str {
        match self {
            Robot_type::Scout => {
                "Scout"
            }
            Robot_type::Harvester => {
                "Harvester"
            }
            Robot_type::Scientist => {
                "Scientist"
            }
        }
    }
}
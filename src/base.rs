use crate::e_map::EMap;
use crate::robot::Robot;
use crate::terrain::Terrain;

pub(crate) struct Base {
    resources: usize,
    shared_map: EMap
}

impl Base {
    pub fn new(width: usize, height: usize) -> Self {
        Base {resources: 0, shared_map: EMap::new(width, height, Terrain::Void)}
    }

    pub fn merge_map(&self, robot: Robot) {

    }

    pub fn add_resource(&mut self) {
        self.resources += 1;
    }
}
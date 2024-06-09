use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap};
use crate::robot::Robot;
use crate::terrain::Terrain;

#[derive(Debug, PartialEq, Eq)]
pub struct Node {
    pub position: (usize, usize),
    pub cost: usize,
    pub estimated_cost: usize,
}

impl Ord for Node {
    fn cmp(&self, other: &Self) -> Ordering {
        other.estimated_cost.cmp(&self.estimated_cost)
    }
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Node {
    pub fn new(position: (usize, usize), cost: usize, estimated_cost: usize) -> Self {
        Node {
            position,
            cost,
            estimated_cost,
        }
    }
}

pub fn pathfind(robot: &Robot, start: (usize, usize), goal: (usize, usize)) -> Option<Vec<(usize, usize)>> {
    let mut open_set = BinaryHeap::new();
    let mut came_from: HashMap<(usize, usize), (usize, usize)> = HashMap::new();
    let mut g_score: HashMap<(usize, usize), usize> = HashMap::new();
    let mut f_score: HashMap<(usize, usize), usize> = HashMap::new();

    let start_node = Node::new(start, 0, heuristic(start, goal));
    open_set.push(start_node);

    g_score.insert(start, 0);
    f_score.insert(start, heuristic(start, goal));

    while let Some(current_node) = open_set.pop() {
        let current = current_node.position;

        if current == goal {
            return Some(reconstruct_path(came_from, current));
        }

        for neighbor in get_neighbors(robot, current) {
            let tentative_g_score = g_score.get(&current).unwrap() + 1;

            if tentative_g_score < *g_score.get(&neighbor).unwrap_or(&usize::MAX) {
                came_from.insert(neighbor, current);
                g_score.insert(neighbor, tentative_g_score);
                f_score.insert(neighbor, tentative_g_score + heuristic(neighbor, goal));

                if !open_set.iter().any(|node| node.position == neighbor) {
                    open_set.push(Node::new(neighbor, tentative_g_score, tentative_g_score + heuristic(neighbor, goal)));
                }
            }
        }
    }

    None
}

fn heuristic(a: (usize, usize), b: (usize, usize)) -> usize {
    (a.0 as isize - b.0 as isize).abs() as usize + (a.1 as isize - b.1 as isize).abs() as usize
}

fn reconstruct_path(came_from: HashMap<(usize, usize), (usize, usize)>, mut current: (usize, usize)) -> Vec<(usize, usize)> {
    let mut total_path = vec![current];
    while let Some(&parent) = came_from.get(&current) {
        current = parent;
        total_path.push(current);
    }
    total_path.reverse();
    total_path
}

fn get_neighbors(robot: &Robot, position: (usize, usize)) -> Vec<(usize, usize)> {
    let mut neighbors = vec![];

    let (x, y) = position;
    let deltas = [(-1, 0), (1, 0), (0, -1), (0, 1)];

    for &(dx, dy) in &deltas {
        let new_x = x as isize + dx;
        let new_y = y as isize + dy;

        if new_x >= 0 && new_x < robot.known_map.width() as isize && new_y >= 0 && new_y < robot.known_map.height() as isize {
            let new_pos = (new_x as usize, new_y as usize);

            let terrain_char = robot.known_map.data[new_pos.1][new_pos.0];
            let terrain = Terrain::from_char(terrain_char);

            if terrain != Terrain::Wall && terrain != Terrain::Mountain && terrain != Terrain::Void {
                neighbors.push(new_pos);
            }
        }
    }

    neighbors
}

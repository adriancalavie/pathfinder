#[derive(Debug, Clone, Copy)]
pub struct Point {
    pub x: f64,
    pub y: f64,
}

impl Point {
    pub fn new(x: f64, y: f64) -> Self {
        Point { x, y }
    }
}
#[derive(Debug, Clone)]
pub struct Node {
    pub point: Point,
    pub neighbors: Vec<Point>,
}

impl Node {
    pub fn new(point: Point, neighbors: Vec<Point>) -> Self {
        Self { point, neighbors }
    }
}

#[derive(Debug, Clone)]
pub struct Map {
    pub nodes: Vec<Node>,
}

impl Map {
    pub fn new(nodes: Vec<Node>) -> Self {
        Self { nodes }
    }

    /// Returns the length of this [`Map`].
    pub fn len(&self) -> usize {
        self.nodes.len()
    }

    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Returns the iter of this [`Map`].
    pub fn iter(&self) -> impl Iterator<Item = &Node> {
        self.nodes.iter()
    }

    /// Returns the iter of this [`Map`].
    pub fn iter_mut(&mut self) -> impl Iterator<Item = &mut Node> {
        self.nodes.iter_mut()
    }
}

pub fn map<T>(value_0_to_1: T, target_min: T, target_max: T) -> T
where
    T: num_traits::Float,
{
    target_min + value_0_to_1 * (target_max - target_min)
}

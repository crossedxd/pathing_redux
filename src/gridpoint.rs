use vector2::Vector2;

#[derive(Debug, Hash, PartialEq, Eq, Copy, Clone)]
pub struct GridPoint {
    pub x: i32,
    pub y: i32,
    pub key: char,
}

#[allow(dead_code)]
impl GridPoint {
    pub fn new(x: i32, y: i32, key: char) -> GridPoint {
        GridPoint { x, y, key }
    }

    pub fn distance_to(&self, g: &GridPoint) -> f64 {
        ((self.x - g.x).abs().pow(2) + (self.y - g.y).abs().pow(2)) as f64
    }

    pub fn manhattan_distance_to(&self, g: &GridPoint) -> f64 {
        (self.x - g.x).abs() as f64 + (self.y - g.y).abs() as f64
    }

    pub fn get_entry_vectors(&self) -> Vec<Vector2> {
        Vector2::vectors_from_key('p')
    }

    pub fn get_exit_vectors(&self) -> Vec<Vector2> {
        Vector2::vectors_from_key(self.key)
    }
}
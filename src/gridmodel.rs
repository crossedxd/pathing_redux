use gridpoint::GridPoint;

#[derive(Debug)]
pub struct GridModel {
    pub width: usize,
    pub height: usize,
    pub grid: Vec<Vec<GridPoint>>,
    // _path: A string of letters representing GridPoints, not currently saved
}

#[allow(dead_code)]
impl GridModel {
    pub fn new(width: usize, height: usize) -> GridModel {
        let mut grid = Vec::new();
        for y in 0..height {
            let mut row = Vec::new();
            for x in 0..width {
                row.push(GridPoint::new(x as i32, y as i32, 'p'));
            }
            grid.push(row);
        }
        GridModel {  width, height, grid }
    }

    pub fn from_string(path: String, width: usize, height: usize) -> GridModel {
        let mut chars = path.chars();
        let mut grid = Vec::new();
        for y in 0..height {
            let mut row = Vec::new();
            for x in 0..width {
                // Note: currently assumes that len(path)==width*height
                match chars.next() {
                    Some(c) => row.push(GridPoint::new(x as i32, y as i32, c)),
                    None => {},
                }
            }
            grid.push(row);
        }
        GridModel {  width, height, grid }
    }

    pub fn get_point(&self, x: usize, y: usize) -> &GridPoint {
        &self.grid[y][x]
    }

    pub fn get_neighbors(&self, current: &GridPoint) -> Vec<&GridPoint> {
        let mut neighbors = Vec::new();
        for exit_vector in current.get_exit_vectors() {
            let mut x = current.x + exit_vector.x;
            let mut y = current.y + exit_vector.y;
            if x >= 0 && x < self.height as i32 && y >= 0 && y < self.width as i32 {
                let neighbor = self.get_point(x as usize, y as usize);
                for entry_vector in neighbor.get_entry_vectors() {
                    x = neighbor.x + entry_vector.x;
                    y = neighbor.y + entry_vector.y;
                    if current.x == x && current.y == y {
                        neighbors.push(neighbor);
                        break;
                    }
                }
            }
        }

        neighbors
    }
}
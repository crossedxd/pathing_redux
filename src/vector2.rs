#[derive(Debug, Hash, PartialEq, Eq, Copy, Clone)]
pub struct Vector2 {
    pub x: i32,
    pub y: i32,
}

#[allow(dead_code)]
impl Vector2 {
    pub fn new(x: i32, y: i32) -> Vector2 {
        Vector2 { x: x, y: y }
    }

    pub fn vectors(direction: &str) -> Vector2 {
        match direction {
            "right" => Vector2 { x: 1, y: 0 },
            "down" => Vector2 { x: 0, y: 1 },
            "left" => Vector2 { x: -1, y: 0 },
            "up" => Vector2 { x: 0, y: -1 },
            _ => Vector2 { x: 0, y: 0 },
        }
    }

    pub fn vectors_from_key(key: char) -> Vec<Vector2> {
        match key {
            'a' => vec![],
            'b' => vec![Vector2::vectors("up")],
            'c' => vec![Vector2::vectors("down")],
            'd' => vec![Vector2::vectors("left")],
            'e' => vec![Vector2::vectors("right")],
            'f' => vec![Vector2::vectors("up"), Vector2::vectors("left")],
            'g' => vec![Vector2::vectors("up"), Vector2::vectors("right")],
            'h' => vec![Vector2::vectors("up"), Vector2::vectors("down")],
            'i' => vec![Vector2::vectors("down"), Vector2::vectors("left")],
            'j' => vec![Vector2::vectors("down"), Vector2::vectors("right")],
            'k' => vec![Vector2::vectors("left"), Vector2::vectors("right")],
            'l' => vec![Vector2::vectors("up"), Vector2::vectors("right"), Vector2::vectors("down")],
            'm' => vec![Vector2::vectors("up"), Vector2::vectors("right"), Vector2::vectors("left")],
            'n' => vec![Vector2::vectors("up"), Vector2::vectors("left"), Vector2::vectors("down")],
            'o' => vec![Vector2::vectors("right"), Vector2::vectors("left"), Vector2::vectors("down")],
            'p' => vec![
                Vector2::vectors("up"), Vector2::vectors("right"),
                Vector2::vectors("down"), Vector2::vectors("left")
                ],
            _ => vec![],
        }
    }
}
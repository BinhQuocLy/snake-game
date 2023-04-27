pub struct Snake {
    pub value: [f64; 4],
    pub color: [f32; 4]
}

pub enum SnakeDirection {
    Up,
    Down,
    Left,
    Right,
}

pub struct Bait {
    pub value: [f64; 4],
    pub color: [f32; 4]
}
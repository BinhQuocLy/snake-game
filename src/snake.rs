pub const GRID_UNIT: f64 = 20.0;

pub struct Snake {
    pub value: [f64; 4], // x, y, w, h
    pub color: [f32; 4],
    pub direction: SnakeDirection
}

pub enum SnakeDirection {
    Up,
    Down,
    Left,
    Right,
}

pub struct Bait {
    pub value: [f64; 4], // x, y, w, h
    pub color: [f32; 4]
}
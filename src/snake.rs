pub const GRID_UNIT: f64 = 20.0;

pub struct Game {
    pub speed: f32,
    pub score: f32,
    ready_to_move: bool
}

impl Game {
    pub fn new(speed: f32, score: f32) -> Game {
        Game {
            speed,
            score,
            ready_to_move: true
        }
    }

    pub fn should_move(&self) -> bool {
        self.ready_to_move
    }
}

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
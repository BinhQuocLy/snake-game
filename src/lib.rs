pub struct Game {
    pub grid_unit: f64,
    pub speed: f64,
    pub score: f32,
    curr_move_weight: f64,
    ready_to_move: bool
}

impl Game {
    pub fn new(grid_unit: f64, speed: f64, score: f32) -> Game {
        Game {
            grid_unit,
            speed,
            score,
            curr_move_weight: 0.0,
            ready_to_move: true
        }
    }

    pub fn should_move(&self) -> bool {
        self.ready_to_move
    }

    pub fn add_tick(&mut self) {
        if self.ready_to_move == true {
            self.ready_to_move = false; 
        }
        self.curr_move_weight += self.speed * 0.01;
        if self.curr_move_weight >= 1.0 {
            self.curr_move_weight = 0.0;
            self.ready_to_move = true;
        }
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
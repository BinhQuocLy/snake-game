use piston_window::*;

// Game ====================
const STD_TIME_UNIT: f64 = 0.01;

pub struct Game {
    curr_frame_weight: f64,
    main_score: f32,

    pub config: GameConfig,
    // pub scene: GameScene,
    pub window: PistonWindow,
}
impl Game {
    pub fn new(config: GameConfig) -> Game {
        Game {
            curr_frame_weight: 0.0,
            main_score: 0.0,
            config,
            window: WindowSettings::new("Snake game", [500, 500])
                .exit_on_esc(true)
                .build()
                .unwrap(),
            // scene: GameScene { objects: vec![] },
        }
    }

    // Scores
    pub fn get_main_score(&self) -> f32 {
        self.main_score
    }

    pub fn add_score(&mut self, value: f32) {
        self.main_score += value;
    }

    // Frames
    pub fn move_now(&mut self) -> bool {
        if self.curr_frame_weight >= 1.0 {
            self.curr_frame_weight = 0.0;
            return true;
        }
        false
    }

    pub fn add_tick(&mut self) {
        self.curr_frame_weight += self.config.game_speed * STD_TIME_UNIT;
    }
}

pub struct GameConfig {
    pub grid_unit: f64,
    pub game_speed: f64,
}

// pub struct GameScene {
//     pub objects: Vec<GameObject>,
// }

// Game objects ====================
// pub enum GameObject {
//     Snake,
//     Bait,
// }

type CellSize = f64;        // w = h
type Position = [f64; 2];   // x y
type Color = [f32; 4];      // r g b a

// Snake ====================
pub struct Snake {
    pub ate: bool,

    pub positions: Vec<Position>,
    pub cell_size: CellSize,
    pub color: Color,
    pub direction: SnakeDirection,
}
impl Snake {
    pub fn new(cell_size: CellSize, position: [f64; 2], color: [f32; 4], direction: SnakeDirection) -> Snake {
        Snake {
            positions: vec![position],
            cell_size,
            color,
            direction,
            ate: false
        }
    }

    pub fn move_by_direction(&mut self, unit_count: f64) {
        match &self.direction {
            SnakeDirection::Up => self.positions.insert(
                0,
                [
                    self.positions[0][0],
                    self.positions[0][1] - unit_count * self.cell_size,
                ],
            ),
            SnakeDirection::Down => self.positions.insert(
                0,
                [
                    self.positions[0][0],
                    self.positions[0][1] + unit_count * self.cell_size,
                ],
            ),
            SnakeDirection::Left => self.positions.insert(
                0,
                [
                    self.positions[0][0] - unit_count * self.cell_size,
                    self.positions[0][1],
                ],
            ),
            SnakeDirection::Right => self.positions.insert(
                0,
                [
                    self.positions[0][0] + unit_count * self.cell_size,
                    self.positions[0][1],
                ],
            ),
        }
        self.positions.pop();
    }

    pub fn grow(&mut self, unit_count: f64) {
        match &self.direction {
            SnakeDirection::Up => self.positions.insert(
                0,
                [
                    self.positions[0][0],
                    self.positions[0][1] - unit_count * self.cell_size,
                ],
            ),
            SnakeDirection::Down => self.positions.insert(
                0,
                [
                    self.positions[0][0],
                    self.positions[0][1] + unit_count * self.cell_size,
                ],
            ),
            SnakeDirection::Left => self.positions.insert(
                0,
                [
                    self.positions[0][0] - unit_count * self.cell_size,
                    self.positions[0][1],
                ],
            ),
            SnakeDirection::Right => self.positions.insert(
                0,
                [
                    self.positions[0][0] + unit_count * self.cell_size,
                    self.positions[0][1],
                ],
            ),
        }
    }

    pub fn reset_stomach(&mut self) {
        self.ate = false;
    }
}

// Bait ====================
pub struct Bait {
    pub position: Position,
    pub size: CellSize,
    pub color: Color,
}
impl Bait {
    pub fn new(size: CellSize, position: [f64; 2], color: [f32; 4]) -> Bait {
        Bait {
            size,
            position,
            color,
        }
    }
}

pub enum SnakeDirection {
    Up,
    Down,
    Left,
    Right,
}

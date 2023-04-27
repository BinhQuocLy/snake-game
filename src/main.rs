use piston_window::*;
// use rand::rngs::StdRng;
// use rand::SeedableRng;
// use std::thread::sleep;
use std::time::{Duration, SystemTime};

extern crate piston_window;

enum SnakeDirection {
    Up,
    Down,
    Left,
    Right,
}

fn get_time_weight() -> f32 {
    let initial_time = SystemTime::UNIX_EPOCH;
    let duration: u64 = SystemTime::now()
        .duration_since(initial_time)
        .unwrap_or(Duration::new(0, 0))
        .as_secs();
    (duration % 100) as f32 / 100.0
}

fn is_new_second(time_weight: f32) -> bool {
    get_time_weight() != time_weight
}

fn main() {
    let window_width: u32 = 500;
    let window_height: u32 = 500;

    let mut window: PistonWindow =
        WindowSettings::new("Hello Piston!", [window_width, window_height])
            .exit_on_esc(true)
            .build()
            .unwrap();

    const GRID_UNIT: f64 = 20.0;

    const SNAKE_WIDTH: f64 = GRID_UNIT;
    const SNAKE_HEIGHT: f64 = GRID_UNIT;
    let mut snake: [f64; 4] = [0.0, 0.0, SNAKE_WIDTH, SNAKE_HEIGHT];
    let snake_color: [f32; 4] = [0.0, 0.0, 0.0, 1.0];

    const BAIT_WIDTH: f64 = GRID_UNIT;
    const BAIT_HEIGHT: f64 = GRID_UNIT;
    let bait = [5.0 * GRID_UNIT, 5.0 * GRID_UNIT, BAIT_WIDTH, BAIT_HEIGHT];
    let bait_color: [f32; 4] = [1.0, 0.0, 0.0, 1.0];

    let mut game_score: i32 = 0;
    // let game_score_color: [f32; 4] = [0.0, 0.0, 0.0, 1.0];

    let mut snake_direction: SnakeDirection = SnakeDirection::Right;

    let mut time_weight: f32 = get_time_weight();
    while let Some(e) = window.next() {
        if let Event::Input(input, _) = &e {
            if let Input::Button(button_args) = input {
                if let Button::Keyboard(key) = button_args.button {
                    match key {
                        Key::Up => snake_direction = SnakeDirection::Up,
                        Key::Down => snake_direction = SnakeDirection::Down,
                        Key::Left => snake_direction = SnakeDirection::Left,
                        Key::Right => snake_direction = SnakeDirection::Right,
                        _ => (),
                    }
                }
            }
        }

        if is_new_second(time_weight) {
            match snake_direction {
                SnakeDirection::Up => snake[1] -= GRID_UNIT,
                SnakeDirection::Down => snake[1] += GRID_UNIT,
                SnakeDirection::Left => snake[0] -= GRID_UNIT,
                SnakeDirection::Right => snake[0] += GRID_UNIT,
            }
            if &snake == &bait {
                game_score += 1;
                println!("Score: {}", game_score);
            }
        }
        time_weight = get_time_weight();

        window.draw_2d(&e, |context, graphics, _device| {
            clear([1.0; 4], graphics);
            rectangle(snake_color, snake, context.transform, graphics);
            rectangle(bait_color, bait, context.transform, graphics);
        });
    }
}

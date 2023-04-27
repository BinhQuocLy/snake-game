extern crate piston_window;

mod utils;
mod snake;

use piston_window::*;
use crate::snake::Bait;
use crate::snake::Snake;
use crate::snake::SnakeDirection;

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
    let mut main_snake: Snake = Snake { 
        value: [0.0, 0.0, SNAKE_WIDTH, SNAKE_HEIGHT], 
        color: [0.0, 0.0, 0.0, 1.0]
    };

    const BAIT_WIDTH: f64 = GRID_UNIT;
    const BAIT_HEIGHT: f64 = GRID_UNIT;
    let mut bait: Bait = Bait { 
        value: [5.0 * GRID_UNIT, 5.0 * GRID_UNIT, BAIT_WIDTH, BAIT_HEIGHT], 
        color: [1.0, 0.0, 0.0, 1.0]
    };

    let mut game_score: i32 = 0;
    // let game_score_color: [f32; 4] = [0.0, 0.0, 0.0, 1.0];

    let mut snake_direction: SnakeDirection = SnakeDirection::Right;

    let mut time_weight: f32 = utils::get_time_weight();
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

        if utils::is_new_second(time_weight) {
            match snake_direction {
                SnakeDirection::Up => main_snake.value[1] -= GRID_UNIT,
                SnakeDirection::Down => main_snake.value[1] += GRID_UNIT,
                SnakeDirection::Left => main_snake.value[0] -= GRID_UNIT,
                SnakeDirection::Right => main_snake.value[0] += GRID_UNIT,
            }
            if &main_snake.value == &bait.value {
                game_score += 1;
                println!("Score: {}", game_score);
            }
        }
        time_weight = utils::get_time_weight();

        window.draw_2d(&e, |context, graphics, _device| {
            clear([1.0; 4], graphics);
            rectangle(main_snake.color, main_snake.value, context.transform, graphics);
            rectangle(bait.color, bait.value, context.transform, graphics);
        });
    }
}

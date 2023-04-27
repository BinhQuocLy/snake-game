extern crate piston_window;

mod utils;
mod snake;

use rand::Rng;
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

    const SNAKE_WIDTH: f64 = snake::GRID_UNIT;
    const SNAKE_HEIGHT: f64 = snake::GRID_UNIT;
    let mut main_snake: Snake = Snake { 
        value: [0.0, 0.0, SNAKE_WIDTH, SNAKE_HEIGHT], 
        color: [0.0, 0.0, 0.0, 1.0],
        direction: SnakeDirection::Right
    };

    const BAIT_WIDTH: f64 = snake::GRID_UNIT;
    const BAIT_HEIGHT: f64 = snake::GRID_UNIT;
    let mut bait: Bait = Bait { 
        value: [5.0 * snake::GRID_UNIT, 5.0 * snake::GRID_UNIT, BAIT_WIDTH, BAIT_HEIGHT], 
        color: [1.0, 0.0, 0.0, 1.0]
    };

    let mut game_score: i32 = 0;

    let mut time_weight: f32 = utils::get_time_weight();

    while let Some(e) = window.next() {
        if let Event::Input(input, _) = &e {
            if let Input::Button(button_args) = input {
                if let Button::Keyboard(key) = button_args.button {
                    match key {
                        Key::Up => main_snake.direction = SnakeDirection::Up,
                        Key::Down => main_snake.direction = SnakeDirection::Down,
                        Key::Left => main_snake.direction = SnakeDirection::Left,
                        Key::Right => main_snake.direction = SnakeDirection::Right,
                        _ => (),
                    }
                }
            }
        }

        if utils::is_new_second(time_weight) {
            match &main_snake.direction {
                SnakeDirection::Up => main_snake.value[1] -= snake::GRID_UNIT,
                SnakeDirection::Down => main_snake.value[1] += snake::GRID_UNIT,
                SnakeDirection::Left => main_snake.value[0] -= snake::GRID_UNIT,
                SnakeDirection::Right => main_snake.value[0] += snake::GRID_UNIT,
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
            if &main_snake.value == &bait.value {
                let random: f64 = rand::thread_rng().gen_range(0..10) as f64;
                bait.value = [random * snake::GRID_UNIT, random * snake::GRID_UNIT, BAIT_WIDTH, BAIT_HEIGHT];
                rectangle(bait.color, bait.value, context.transform, graphics);
            } else {
                rectangle(bait.color, bait.value, context.transform, graphics);
            }
        });
    }
}

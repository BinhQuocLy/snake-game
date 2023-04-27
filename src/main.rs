extern crate piston_window;

mod lib;

use crate::lib::Bait;
use crate::lib::Game;
use crate::lib::Snake;
use crate::lib::SnakeDirection;
use piston_window::*;
use rand::Rng;

fn main() {
    let window_dimensions = [500, 500];
    let mut window: PistonWindow =
        WindowSettings::new("Snake game", window_dimensions)
            .exit_on_esc(true)
            .build()
            .unwrap();

    let mut game: Game = Game::new(20.0, 1.0, 0.0);

    let snake_width: f64 = game.grid_unit;
    let snake_height: f64 = game.grid_unit;
    let mut main_snake: Snake = Snake {
        value: [0.0, 0.0, snake_width, snake_height],
        color: [0.0, 0.0, 0.0, 1.0],
        direction: SnakeDirection::Right,
    };

    let bait_width: f64 = game.grid_unit;
    let bait_height: f64 = game.grid_unit;
    let mut bait: Bait = Bait {
        value: [
            5.0 * game.grid_unit,
            5.0 * game.grid_unit,
            bait_width,
            bait_height,
        ],
        color: [1.0, 0.0, 0.0, 1.0],
    };

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

        if game.should_move() {
            match &main_snake.direction {
                SnakeDirection::Up => main_snake.value[1] -= game.grid_unit,
                SnakeDirection::Down => main_snake.value[1] += game.grid_unit,
                SnakeDirection::Left => main_snake.value[0] -= game.grid_unit,
                SnakeDirection::Right => main_snake.value[0] += game.grid_unit,
            }
            if &main_snake.value == &bait.value {
                game.score += 1.0;
                println!("Score: {}", &game.score);
            }
        }
        game.add_tick();

        window.draw_2d(&e, |context, graphics, _device| {
            clear([1.0; 4], graphics);
            rectangle(
                main_snake.color,
                main_snake.value,
                context.transform,
                graphics,
            );
            if &main_snake.value == &bait.value {
                let random: f64 = rand::thread_rng().gen_range(0..10) as f64;
                bait.value = [
                    random * game.grid_unit,
                    random * game.grid_unit,
                    bait_width,
                    bait_height,
                ];
                rectangle(bait.color, bait.value, context.transform, graphics);
            } else {
                rectangle(bait.color, bait.value, context.transform, graphics);
            }
        });
    }
}

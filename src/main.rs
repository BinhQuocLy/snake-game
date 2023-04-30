mod game_lib;

use crate::game_lib::Bait;
use crate::game_lib::Game;
use crate::game_lib::GameConfig;
use crate::game_lib::Snake;
use crate::game_lib::SnakeDirection;

use piston_window::*;
use rand::Rng;

fn main() {
    let mut game: Game = Game::new(GameConfig {
        grid_unit: 20.0,
        game_speed: 1.0,
    });

    let unit_size = game.config.grid_unit;

    let snake_position = [0.0, 0.0];
    let snake_color = [0.0, 0.0, 0.0, 1.0];
    let mut snake = Snake::new(
        unit_size,
        snake_position,
        snake_color,
        SnakeDirection::Right,
    );

    let bait_position = [5.0 * unit_size, 5.0 * unit_size];
    let bait_color = [1.0, 0.0, 0.0, 1.0];
    let mut bait = Bait::new(unit_size, bait_position, bait_color);

    // Main loop
    while let Some(e) = game.window.next() {
        // Get key input
        if let Event::Input(input, _) = &e {
            if let Input::Button(button_args) = input {
                if let Button::Keyboard(key) = button_args.button {
                    match key {
                        Key::Up => snake.direction = SnakeDirection::Up,
                        Key::Down => snake.direction = SnakeDirection::Down,
                        Key::Left => snake.direction = SnakeDirection::Left,
                        Key::Right => snake.direction = SnakeDirection::Right,
                        _ => (),
                    }
                }
            }
        }

        // Check new tick
        if game.move_now() {
            snake.move_by_direction(1.0);
            if &snake.positions[0] == &bait.position {
                // Eat -> Add score
                snake.ate = true;
                game.add_score(1.0);
                println!("Score: {}", game.get_main_score());
            }
        }
        game.add_tick();

        // Render
        game.window.draw_2d(&e, |context, graphics, _device| {
            clear([1.0; 4], graphics);

            //Render snake
            if snake.ate {
                snake.grow(1.0);
            }
            for i in 0..snake.positions.len() {
                rectangle(
                    snake.color,
                    [
                        snake.positions[i][0],
                        snake.positions[i][1],
                        snake.cell_size,
                        snake.cell_size,
                    ],
                    context.transform,
                    graphics,
                );
            }

            // Render bait
            if snake.ate {
                // Eat -> move bait
                let random: f64 = rand::thread_rng().gen_range(0..10) as f64;
                bait.position = [
                    random * game.config.grid_unit,
                    random * game.config.grid_unit,
                ];
                rectangle(
                    bait.color,
                    [
                        random * game.config.grid_unit,
                        random * game.config.grid_unit,
                        bait.size,
                        bait.size,
                    ],
                    context.transform,
                    graphics,
                );
            } else {
                // Not eat
                rectangle(
                    bait.color,
                    [bait.position[0], bait.position[1], bait.size, bait.size],
                    context.transform,
                    graphics,
                );
            }

            snake.reset_stomach();
        });
    }
}

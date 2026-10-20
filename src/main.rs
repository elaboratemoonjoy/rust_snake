use ::rand::RngExt;

use macroquad::prelude::*;

const GAME_DIMENSION: i32 = 16;

type Point = (i32, i32);

struct Snake {
    segments: Vec<Point>,
    dir: Point,
}

fn window_conf() -> Conf {
    Conf {
        window_title: "Snake Game".to_owned(),
        window_width: 800,
        window_height: 800,
        window_resizable: false,
        ..Default::default()
    }
}

fn grow_new_fruit(grid_size: i32, segments: &[Point]) -> Point {
    let mut rng = ::rand::rng();

    loop {
        let x = rng.random_range(0..grid_size);
        let y = rng.random_range(0..grid_size);
        let fruit = (x, y);

        if !segments.contains(&fruit) {
            return fruit;
        }
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    let screen_size = screen_width();
    let total_squares = (GAME_DIMENSION * GAME_DIMENSION) as usize;

    let sq_size = screen_size as i32 / GAME_DIMENSION;

    let mut snake = Snake {
        segments: vec![(5, 5), (4, 5), (3, 5)],
        dir: (1, 0),
    };

    let up: Point = (0, -1);
    let down: Point = (0, 1);
    let right: Point = (1, 0);
    let left: Point = (-1, 0);

    let mut move_timer = 0.0;
    let move_interval = 0.15;

    let mut pending_dir = snake.dir;

    let mut game_over = false;
    let mut game_win = false;

    let mut fruit = grow_new_fruit(GAME_DIMENSION, &snake.segments);

    loop {
        clear_background(LIGHTGRAY);

        if game_over {
            // Dim background
            draw_rectangle(
                0.0,
                0.0,
                screen_width(),
                screen_height(),
                Color::new(0.0, 0.0, 0.0, 0.6),
            );

            // Draw game over text,
            let text = "GAME OVER";
            let font_size = 50.0;
            let dimensions = measure_text(text, None, font_size as u16, 1.0);
            draw_text(
                text,
                screen_size / 2.0 - dimensions.width / 2.0,
                screen_size / 2.0 + dimensions.height / 2.0,
                font_size,
                WHITE,
            );
        } else if game_win {
            draw_rectangle(
                0.0,
                0.0,
                screen_width(),
                screen_height(),
                Color::new(0.0, 0.0, 0.0, 0.6),
            );

            let text = "YOU WON!";
            let font_size = 50.0;
            let dimensions = measure_text(text, None, font_size as u16, 1.0);
            draw_text(
                text,
                screen_size / 2.0 - dimensions.width / 2.0,
                screen_size / 2.0 + dimensions.height / 2.0,
                font_size,
                WHITE,
            );
        } else {
            if is_key_down(KeyCode::Right) && snake.dir != left {
                pending_dir = right;
            } else if is_key_down(KeyCode::Left) && snake.dir != right {
                pending_dir = left;
            } else if is_key_down(KeyCode::Down) && snake.dir != up {
                pending_dir = down;
            } else if is_key_down(KeyCode::Up) && snake.dir != down {
                pending_dir = up;
            }

            move_timer += get_frame_time();
            if move_timer >= move_interval && !game_over {
                snake.dir = pending_dir;

                let head = snake.segments[0];
                let new_head = (head.0 + snake.dir.0, head.1 + snake.dir.1);

                // Check if out of bounds
                if new_head.0 < 0 || new_head.1 < 0 {
                    game_over = true
                } else if new_head.0 > (GAME_DIMENSION - 1) || new_head.1 > (GAME_DIMENSION - 1) {
                    game_over = true
                }

                // Check if we ate ourselves
                if snake.segments[..&snake.segments.len() - 1].contains(&new_head) {
                    game_over = true
                }

                snake.segments.insert(0, new_head);

                // Check if we ate the fruit
                if new_head == fruit {
                    // Check if we won :)
                    if snake.segments.len() >= total_squares {
                        game_win = true;
                    } else {
                        fruit = grow_new_fruit(GAME_DIMENSION, &snake.segments)
                    }
                } else {
                    snake.segments.pop();
                }

                move_timer = 0.0;
            }
        }

        for segment in &snake.segments {
            draw_rectangle(
                (segment.0 * sq_size) as f32,
                (segment.1 * sq_size) as f32,
                sq_size as f32,
                sq_size as f32,
                WHITE,
            );
        }

        // Draw fruit
        draw_rectangle(
            (fruit.0 * sq_size) as f32,
            (fruit.1 * sq_size) as f32,
            sq_size as f32,
            sq_size as f32,
            GREEN,
        );

        // Grid
        for n in 1..GAME_DIMENSION {
            let offset = (sq_size * n) as f32;
            draw_line(offset, 0.0, offset, screen_size, 1.0, GRAY);
            draw_line(0.0, offset, screen_size, offset, 1.0, GRAY);
        }

        next_frame().await
    }
}

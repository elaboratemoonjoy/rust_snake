use std::{thread::sleep, time::Duration};

use macroquad::prelude::*;

const SQUARES: i32 = 16;

type Point = (f32, f32);

struct Snake {
    segments: Vec<Point>,
    dir: Point,
}

fn window_conf() -> Conf {
    Conf {
        window_title: "MyGame".to_owned(),
        window_width: 800,
        window_height: 800,
        window_resizable: false,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    request_new_screen_size(screen_width(), screen_width());
    let game_size = screen_width();

    let sq_size = game_size / SQUARES as f32;

    let mut snake = Snake {
        segments: vec![(5.0, 5.0), (4.0, 5.0), (3.0, 5.0)],
        dir: (1.0, 0.0),
    };

    let up: Point = (0.0, -1.0);
    let down: Point = (0.0, 1.0);
    let right: Point = (1.0, 0.0);
    let left: Point = (-1.0, 0.0);

    let mut move_timer = 0.0;
    let move_interval = 0.15;

    let mut pending_dir = snake.dir;

    let mut game_over = false;

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
                game_size / 2.0 - dimensions.width / 2.0,
                game_size / 2.0 + dimensions.height / 2.0,
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
            if move_timer >= move_interval {
                snake.dir = pending_dir;

                let head = snake.segments[0];
                let new_head = (head.0 + snake.dir.0, head.1 + snake.dir.1);

                if new_head.0 < 0 as f32 || new_head.1 < 0 as f32 {
                    game_over = true
                } else if new_head.0 > (SQUARES - 1) as f32 || new_head.1 > (SQUARES - 1) as f32 {
                    game_over = true
                }

                snake.segments.insert(0, new_head);
                snake.segments.pop();
                move_timer = 0.0;
            }
        }

        for segment in &snake.segments {
            draw_rectangle(
                segment.0 * sq_size,
                segment.1 * sq_size,
                sq_size,
                sq_size,
                WHITE,
            );
        }

        // Grid
        for n in 1..SQUARES {
            let offset = sq_size * n as f32 as f32;
            draw_line(offset, 0.0, offset, game_size, 1.0, GRAY);
            draw_line(0.0, offset, game_size, offset, 1.0, GRAY);
        }

        next_frame().await
    }
}

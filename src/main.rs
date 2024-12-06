mod ball;
mod bounding_box;
mod icon_data;

use macroquad::miniquad::conf::Icon;
use macroquad::prelude::*;
use ball::Ball;
use bounding_box::BoundingBox;
use icon_data::{ICON_BIG, ICON_MEDIUM, ICON_SMALL};

const SCREEN_WIDTH: f32 = 800.0;
const SCREEN_HEIGHT: f32 = 600.0;

const BALL_RADIUS: f32 = 8.0;
const PEG_RADIUS: f32 = 10.0;
const GRAVITY: f32 = 20000.0;
const FRICTION: f32 = 0.98;
const BOUNCE_DAMPING: f32 = 0.8;
const ROWS: usize = 10;

fn window_conf() -> Conf {
    Conf {
        window_title: "Plinko".to_string(),
        window_width: 800,
        window_height: 450,
        window_resizable: false,
        icon: Some(Icon { small: ICON_SMALL, medium: ICON_MEDIUM, big: ICON_BIG}),
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    let pegs = create_triangle_pegs();
    let mut balls: Vec<Ball> = Vec::new();
    let mut auto_spawn = false;
    let mut spawn_timer = 0.0;
    let button_bb = BoundingBox {
        x: SCREEN_WIDTH / 2.0 + 150.0,
        y: 10.0,
        width: 200.0,
        height: 40.0,
    };

    loop {
        clear_background(DARKBLUE);

        for &(x, y) in &pegs {
            draw_circle(x, y, PEG_RADIUS, WHITE);
        }

        let button_color = if auto_spawn { GREEN } else { RED };
        draw_rectangle(button_bb.x, button_bb.y, button_bb.width, button_bb.height, button_color);
        draw_text(
            if auto_spawn { "Auto Spawn: ON" } else { "Auto Spawn: OFF" },
            button_bb.x + 1.0,
            button_bb.y + 25.0,
            30.0,
            BLACK,
        );

        if is_mouse_button_pressed(MouseButton::Left) {
            let (mouse_x, mouse_y) = mouse_position();
            if button_bb.contains(mouse_x, mouse_y) {
                auto_spawn = !auto_spawn;
            } else {
                balls.push(Ball::new(mouse_x, mouse_y));
            }
        }

        let dt = get_frame_time();
        if auto_spawn {
            spawn_timer += dt;
            if spawn_timer >= 0.5 {
                spawn_timer = 0.0;
                balls.push(Ball::new(SCREEN_WIDTH / 2.0, 0.0));
            }
        }

        let dt = get_frame_time();
        draw_text(&*("FPS: ".to_owned() + &*(1f32 / get_frame_time()).to_string()), 10f32, 40f32, 35f32, BLACK);

        for ball in &mut balls {
            ball.update(dt, &pegs);
            ball.draw();
        }

        balls.retain(|ball| {
            ball.y + BALL_RADIUS < SCREEN_HEIGHT
        });

        draw_text(&*("Balls amount: ".to_owned() + &*balls.len().to_string()), 10f32, 70f32, 35f32, BLACK);

        next_frame().await;
    }
}

fn create_triangle_pegs() -> Vec<(f32, f32)> {
    let mut pegs = Vec::new();
    let spacing_x = 50.0;
    let spacing_y = 40.0;
    let start_x = SCREEN_WIDTH / 2f32;

    for row in 0..ROWS {
        let row_width = row as f32 * spacing_x;
        let row_start_x = start_x - row_width / 2.0;
        for col in 0..=row {
            let x = row_start_x + col as f32 * spacing_x;
            let y = 40f32 + spacing_y * row as f32;
            pegs.push((x, y));
        }
    }
    pegs
}

use raylib::prelude::*;
use rand::Rng;

enum GameState {
    Menu,
    Playing,
    GameOver,
}

fn main() {
    let (mut rl, thread) = raylib::init()
        .size(800, 600)
        .title("Geometry Rays")
        .build();

    rl.set_target_fps(60);

    let mut game_state = GameState::Menu;
    let mut player = Rectangle::new(50.0, 500.0, 50.0, 50.0);
    let mut obstacles = vec![generate_spike(800.0), generate_spike(1100.0)];
    let mut velocity_y = 0.0;
    let gravity = 0.8;
    let jump_force = -15.0;
    let mut is_on_ground = true;
    let mut score = 0;
    let mut high_score = 0;
    let mut background_offset = 0.0;

    while !rl.window_should_close() {
        // Handle inputs and game state logic
        let enter_pressed = rl.is_key_pressed(KeyboardKey::KEY_ENTER);
        let space_pressed = rl.is_key_pressed(KeyboardKey::KEY_SPACE);

        match game_state {
            GameState::Menu => {
                if enter_pressed {
                    game_state = GameState::Playing;
                    score = 0;
                    player.x = 50.0;
                    player.y = 500.0;
                    obstacles = vec![generate_spike(800.0), generate_spike(1100.0)];
                }
            }
            GameState::Playing => {
                if is_on_ground && space_pressed {
                    velocity_y = jump_force;
                    is_on_ground = false;
                }

                velocity_y += gravity;
                player.y += velocity_y;

                if player.y >= 500.0 {
                    player.y = 500.0;
                    velocity_y = 0.0;
                    is_on_ground = true;
                }

                for obstacle in obstacles.iter_mut() {
                    obstacle.x -= 5.0;
                    if obstacle.x + 50.0 < 0.0 {
                        *obstacle = generate_spike(800.0 + rand::thread_rng().gen_range(100.0..400.0));
                        score += 1;
                    }
                }

                for obstacle in &obstacles {
                    if check_collision_triangle_rectangle(
                        obstacle.x,
                        obstacle.y,
                        obstacle.x + 50.0,
                        obstacle.y + 50.0,
                        obstacle.x + 50.0,
                        obstacle.y,
                        player,
                    ) {
                        game_state = GameState::GameOver;
                        high_score = high_score.max(score);
                    }
                }
            }
            GameState::GameOver => {
                if enter_pressed {
                    game_state = GameState::Menu;
                }
            }
        }

        // Rendering
        let mut d = rl.begin_drawing(&thread);
        match game_state {
            GameState::Menu => {
                d.clear_background(Color::BLACK);
                d.draw_text("Geometry Rays", 220, 150, 50, Color::WHITE);
                d.draw_text("Press ENTER to Start", 230, 300, 20, Color::GRAY);
            }
            GameState::Playing => {
                d.clear_background(Color::RAYWHITE);

                background_offset -= 2.0;
                if background_offset < -800.0 {
                    background_offset = 0.0;
                }
                d.draw_rectangle(background_offset as i32, 0, 800, 600, Color::DARKGRAY);
                d.draw_rectangle((background_offset + 800.0) as i32, 0, 800, 600, Color::DARKGRAY);

                d.draw_rectangle_rec(player, Color::BLUE);

                for obstacle in &obstacles {
                    d.draw_triangle(
                        Vector2::new(obstacle.x, obstacle.y),
                        Vector2::new(obstacle.x + 50.0, obstacle.y + 50.0),
                        Vector2::new(obstacle.x + 50.0, obstacle.y),
                        Color::RED,
                    );
                }

                d.draw_text(&format!("Score: {}", score), 10, 10, 20, Color::BLACK);
                d.draw_text(&format!("High Score: {}", high_score), 10, 40, 20, Color::BLACK);
            }
            GameState::GameOver => {
                d.clear_background(Color::DARKRED);
                d.draw_text("Game Over!", 250, 150, 50, Color::WHITE);
                d.draw_text(&format!("Your Score: {}", score), 300, 250, 20, Color::GRAY);
                d.draw_text(&format!("High Score: {}", high_score), 300, 280, 20, Color::GRAY);
                d.draw_text("Press ENTER to Restart", 220, 400, 20, Color::WHITE);
            }
        }
    }
}

fn generate_spike(x: f32) -> Rectangle {
    Rectangle::new(x, 500.0, 50.0, 50.0)
}

fn check_collision_triangle_rectangle(
    x1: f32,
    y1: f32,
    x2: f32,
    y2: f32,
    x3: f32,
    y3: f32,
    rect: Rectangle,
) -> bool {
    let rect_points = [
        (rect.x, rect.y),
        (rect.x + rect.width, rect.y),
        (rect.x, rect.y + rect.height),
        (rect.x + rect.width, rect.y + rect.height),
    ];

    for (rx, ry) in rect_points.iter() {
        if point_in_triangle(*rx, *ry, x1, y1, x2, y2, x3, y3) {
            return true;
        }
    }

    false
}

fn point_in_triangle(px: f32, py: f32, x1: f32, y1: f32, x2: f32, y2: f32, x3: f32, y3: f32) -> bool {
    let area_orig = ((x2 - x1) * (y3 - y1) - (x3 - x1) * (y2 - y1)).abs();
    let area1 = ((x1 - px) * (y2 - py) - (x2 - px) * (y1 - py)).abs();
    let area2 = ((x2 - px) * (y3 - py) - (x3 - px) * (y2 - py)).abs();
    let area3 = ((x3 - px) * (y1 - py) - (x1 - px) * (y3 - py)).abs();
    (area1 + area2 + area3 - area_orig).abs() < 0.01
}
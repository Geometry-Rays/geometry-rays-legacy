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

    // A bunch of variables for the game to function
    // What each of these are used for should be self explanatory
    let mut game_state = GameState::Menu;
    let mut player = Rectangle::new(200.0, 500.0, 40.0, 40.0);
    let mut obstacles = vec![generate_spike(800.0), generate_spike(1100.0)];
    let mut velocity_y = 0.0;
    let gravity = 0.8;
    let jump_force = -15.0;
    let mut is_on_ground = true;
    let mut world_offset = 0.0;
    let movement_speed = 5.0;
    let mut rotation = 0.0;
    let mut attempt = 1;

    // Textures
    let game_bg = rl
        .load_texture(&thread, "Resources/default-bg.png")
        .expect("Failed to load background texture");

    let menu_bg = rl
        .load_texture(&thread, "Resources/default-bg-no-gradient.png")
        .expect("Failed to load menu background texture");

    while !rl.window_should_close() {
        let enter_pressed = rl.is_key_pressed(KeyboardKey::KEY_ENTER);
        let _space_pressed = rl.is_key_pressed(KeyboardKey::KEY_SPACE);
        let space_down = rl.is_key_down(KeyboardKey::KEY_SPACE);

        match game_state {
            GameState::Menu => {
                if enter_pressed {
                    game_state = GameState::Playing;
                    player.y = 500.0;
                    world_offset = 0.0;
                    obstacles = vec![generate_spike(800.0), generate_spike(1100.0)];
                    rotation = 0.0;
                }
            }
            GameState::Playing => {
                // Geometry Rays style controls - hold space to continuously jump when on ground
                if is_on_ground && space_down {
                    velocity_y = jump_force;
                    is_on_ground = false;
                }

                // Move world instead of player
                world_offset -= movement_speed;

                // Update player physics
                velocity_y += gravity;
                player.y += velocity_y;
                
                // Ground collision
                if player.y >= 500.0 {
                    player.y = 500.0;
                    velocity_y = 0.0;
                    is_on_ground = true;
                    rotation = 0.0;
                } else {
                    // Rotate player while in air
                    rotation += 5.0;
                }
                
                // Check for collisions with adjusted obstacle positions
                for obstacle in &obstacles {
                    let actual_x = obstacle.x + world_offset;
                    if check_collision_triangle_rectangle(
                        actual_x,
                        obstacle.y,
                        actual_x + 50.0,
                        obstacle.y + 50.0,
                        actual_x + 50.0,
                        obstacle.y,
                        player,
                    ) {
                        game_state = GameState::GameOver;
                    }
                }
                
                // Update obstacles
                for obstacle in obstacles.iter_mut() {
                    if obstacle.x + world_offset < -50.0 {
                        obstacle.x = 800.0 + rand::thread_rng().gen_range(100.0..400.0);
                    }
                }
            }
            GameState::GameOver => {
                if enter_pressed {
                    game_state = GameState::Menu;
                    attempt += 1;
                }
            }
        }

        // Rendering
        let mut d = rl.begin_drawing(&thread);
        match game_state {
            GameState::Menu => {
                d.clear_background(Color::WHITE);
                d.draw_texture_ex(&menu_bg, Vector2::new(0.0, -100.0), 0.0, 0.8, Color { r:50, g:50, b:50, a:255 });

                d.draw_text("Geometry Rays", 220, 150, 50, Color::WHITE);
                d.draw_text("Press ENTER to Start", 230, 300, 20, Color::GRAY);
                d.draw_text("Hold SPACE to Jump", 250, 330, 20, Color::GRAY);
            }
            GameState::Playing => {
                // Draw background
                d.clear_background(Color::WHITE);
                d.draw_texture_ex(&game_bg, Vector2::new(0.0, 0.0), 0.0, 0.5, Color::WHITE);

                // Draw ground
                d.draw_rectangle(0, 520, 800, 100, Color::DARKGRAY);

                
                // Draw player with rotation
                let _player_center = Vector2::new(
                    player.x + player.width / 2.0,
                    player.y + player.height / 2.0,
                );
                d.draw_rectangle_pro(
                    player,
                    Vector2::new(player.width / 2.0, player.height / 2.0),
                    rotation,
                    Color::BLUE,
                );

                // Draw obstacles with world offset
                for obstacle in &obstacles {
                    let actual_x = obstacle.x + world_offset;
                    d.draw_triangle(
                        Vector2::new(actual_x, obstacle.y),
                        Vector2::new(actual_x + 50.0, obstacle.y + 50.0),
                        Vector2::new(actual_x + 50.0, obstacle.y),
                        Color::RED,
                    );
                }

                // Draw attempt text
                d.draw_text(&format!("Attempt: {}", attempt), 10, 10, 20, Color::RED);
            }
            GameState::GameOver => {
                d.clear_background(Color::WHITE);
                d.draw_texture_ex(&menu_bg, Vector2::new(0.0, -100.0), 0.0, 0.8, Color::DARKRED);

                d.draw_text("Game Over!", 250, 150, 50, Color::WHITE);
                d.draw_text(&format!("Attempts: {}", attempt), 330, 250, 20, Color::WHITE);
                d.draw_text("Press ENTER to Restart", 250, 330, 20, Color::WHITE);
            }
        }
    }
}

fn generate_spike(x: f32) -> Rectangle {
    Rectangle::new(x, 470.0, 50.0, 50.0)
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
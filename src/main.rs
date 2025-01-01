use raylib::prelude::*;
use rand::Rng;

use rodio::{Decoder, OutputStream, Sink, Source};
use std::fs::File;
use std::io::BufReader;

enum GameState {
    Menu,
    Playing,
    GameOver,
    Editor,
}

fn main() {
    let (mut rl, thread) = raylib::init()
        .size(800, 600)
        .title("Geometry Rays")
        .build();

    let (_stream, stream_handle) = OutputStream::try_default().unwrap();
    let sink = Sink::try_new(&stream_handle).unwrap();

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
    let version = "ALPHA";

    // Color Channels
    // CC stands for Color Channel
    // 1001 is the bg
    // 1002 is the ground
    // 1003 is the player
    // 1004 is used by spikes and eventually blocks by default so basically obj color in gd
    // Everything before 1001 is just like in gd where you can use them for whatever you want
    // But custom color channels dont exist yet
    let cc_1001 = Color { r:0, g:0, b:50, a:255 };
    let cc_1002 = Color { r:0, g:0, b:100, a:255 };
    let cc_1003 = Color::BLUE;
    let cc_1004 = Color::WHITE;

    // Textures
    let game_bg = rl
        .load_texture(&thread, "Resources/default-bg.png")
        .expect("Failed to load background texture");

    let menu_bg = rl
        .load_texture(&thread, "Resources/default-bg-no-gradient.png")
        .expect("Failed to load menu background texture");

    let spike_texture = rl
        .load_texture(&thread, "Resources/spike.png")
        .expect("Failed to load spike texture");

    let logo = rl
        .load_texture(&thread, "Resources/logo.png")
        .expect("Failed to load logo texture");

    let ground_texture = rl
        .load_texture(&thread, "Resources/ground.png")
        .expect("Failed to load ground texture");

    // Audio
    let menu_loop_file = BufReader::new(File::open("Resources/menu-loop.mp3").expect("Failed to open MP3 file"));
    let menu_loop = Decoder::new(menu_loop_file).expect("Failed to decode MP3 file").repeat_infinite();
    sink.append(menu_loop);

    while !rl.window_should_close() {
        let play_button_pressed = rl.is_key_pressed(KeyboardKey::KEY_ENTER);
        let editor_button_pressed = rl.is_key_pressed(KeyboardKey::KEY_E);
        let menu_button_pressed = rl.is_key_pressed(KeyboardKey::KEY_M);
        let _space_pressed = rl.is_key_pressed(KeyboardKey::KEY_SPACE);
        let space_down = rl.is_key_down(KeyboardKey::KEY_SPACE);
        let mouse_down = rl.is_mouse_button_down(MouseButton::MOUSE_BUTTON_LEFT);

        match game_state {
            GameState::Menu => {
                if play_button_pressed {
                    game_state = GameState::Playing;
                    player.y = 500.0;
                    world_offset = 0.0;
                    obstacles = vec![generate_spike(800.0), generate_spike(1100.0)];
                    rotation = 0.0;
                }

                if editor_button_pressed {
                    game_state = GameState::Editor;
                }

                sink.play();
            }
            GameState::Playing => {
                // Geometry Rays style controls - hold space to continuously jump when on ground
                if is_on_ground && space_down || is_on_ground && mouse_down {
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
                if play_button_pressed {
                    game_state = GameState::Menu;
                    attempt += 1;
                }
            }
            GameState::Editor => {
                if menu_button_pressed {
                    game_state = GameState::Menu;
                }
            }
        }

        // Rendering
        let mut d = rl.begin_drawing(&thread);
        match game_state {
            GameState::Menu => {
                d.clear_background(Color::WHITE);
                d.draw_texture_ex(&menu_bg, Vector2::new(-200.0, -250.0), 0.0, 0.8, Color { r:50, g:50, b:50, a:255 });

                d.draw_text("Geometry Rays", 220, 150, 50, Color::WHITE);
                
                d.draw_text("Press ENTER to Start", 250, 300, 20, Color::WHITE);
                d.draw_text("Hold SPACE to Jump", 250, 330, 20, Color::WHITE);
                d.draw_text("Press E to go to the Level Editor", 250, 360, 20, Color::GRAY);

                d.draw_text(&format!("Version: {}", version), 10, 10, 15, Color::WHITE);

                d.draw_rectangle_pro(
                    Rectangle::new(360.0, 60.0, 100.0, 100.0),
                    Vector2::new(40.0 / 2.0, 40.0 / 2.0),
                    0.0,
                    Color::BLACK,
                );

                d.draw_texture_ex(&logo, Vector2::new(350.0, 50.0), 0.0, 0.1, Color::WHITE);
            }
            GameState::Playing => {
                // Draw background
                d.clear_background(Color::WHITE);
                d.draw_texture_ex(&game_bg, Vector2::new(0.0, 0.0), 0.0, 0.5, cc_1001);
                
                // Old way of drawing the ground cuz why not keep it here
                // New way is drawn after the player
                // d.draw_rectangle(0, 520, 800, 100, Color::DARKGRAY);

                
                // Draw player with rotation
                let _player_center = Vector2::new(
                    player.x + player.width / 2.0,
                    player.y + player.height / 2.0,
                );
                d.draw_rectangle_pro(
                    player,
                    Vector2::new(player.width / 2.0, player.height / 2.0),
                    rotation,
                    cc_1003,
                );
                
                // Draw ground
                d.draw_texture_ex(&ground_texture, Vector2::new(0.0, 520.0), 0.0, 0.2, cc_1002);
                d.draw_texture_ex(&ground_texture, Vector2::new(150.0, 520.0), 0.0, 0.2, cc_1002);
                d.draw_texture_ex(&ground_texture, Vector2::new(300.0, 520.0), 0.0, 0.2, cc_1002);
                d.draw_texture_ex(&ground_texture, Vector2::new(450.0, 520.0), 0.0, 0.2, cc_1002);
                d.draw_texture_ex(&ground_texture, Vector2::new(600.0, 520.0), 0.0, 0.2, cc_1002);
                d.draw_texture_ex(&ground_texture, Vector2::new(750.0, 520.0), 0.0, 0.2, cc_1002);

                // Draw obstacles with world offset
                for obstacle in &obstacles {
                    let actual_x = obstacle.x + world_offset;
                    d.draw_texture_ex(&spike_texture, Vector2::new(actual_x, 440.0), 0.0, 0.1, cc_1004);

                    // Old way of drawing spikes
                    // Keeping cuz why not
                    // d.draw_triangle(
                    //     Vector2::new(actual_x, obstacle.y),
                    //     Vector2::new(actual_x + 50.0, obstacle.y + 50.0),
                    //     Vector2::new(actual_x + 50.0, obstacle.y),
                    //     Color::RED,
                    // );
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
            GameState::Editor => {
                d.clear_background(Color::WHITE);
                d.draw_texture_ex(&menu_bg, Vector2::new(-200.0, -250.0), 0.0, 0.8, Color { r:50, g:50, b:50, a:255 });
                
                d.draw_text("Editor will be added eventually!", 50, 250, 45, Color::WHITE);
                d.draw_text("Press M to go to back to the Main Menu!", 175, 310, 20, Color::WHITE);
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
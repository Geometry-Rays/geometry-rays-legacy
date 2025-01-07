use raylib::prelude::*;
use rand::Rng;
use rodio::{Decoder, OutputStream, Sink, Source};
use std::fs::File;
use std::io::BufReader;
use webbrowser;

enum GameState {
    Menu,
    Playing,
    GameOver,
    CreatorMenu,
    Editor,
}

struct Button {
    rect: Rectangle,
    text: String,
    font_size: i32,
    base_color: Color,
    hover_color: Color,
    hover_scale: f32,
    press_offset: f32,
    is_pressed: bool,
    animation_timer: f32,
}

impl Button {
    fn new(x: f32, y: f32, width: f32, height: f32, text: &str, font_size: i32) -> Self {
        Button {
            rect: Rectangle::new(x, y, width, height),
            text: text.to_string(),
            font_size,
            base_color: Color::WHITE,
            hover_color: Color::new(240, 240, 240, 255),
            hover_scale: 1.0,
            press_offset: 0.0,
            is_pressed: false,
            animation_timer: 0.0,
        }
    }

    fn update(&mut self, rl: &RaylibHandle, delta_time: f32) {
        let mouse_pos = rl.get_mouse_position();
        let is_hovered = self.is_hovered(mouse_pos);
        let is_pressed = is_hovered && rl.is_mouse_button_down(MouseButton::MOUSE_BUTTON_LEFT);
        
        // Update hover animation
        let target_scale = if is_hovered { 1.1 } else { 1.0 };
        self.hover_scale += (target_scale - self.hover_scale) * (delta_time * 12.0);
        
        // Update press animation
        let target_offset = if is_pressed { 4.0 } else { 0.0 };
        self.press_offset += (target_offset - self.press_offset) * (delta_time * 15.0);
        
        // Update color animation
        if is_hovered {
            self.animation_timer = (self.animation_timer + delta_time * 8.0).min(1.0);
        } else {
            self.animation_timer = (self.animation_timer - delta_time * 8.0).max(0.0);
        }
        
        self.is_pressed = is_pressed;
    }

    fn draw(&self, d: &mut RaylibDrawHandle) {
        let scale_offset_x = self.rect.width * (self.hover_scale - 1.0) * 0.5;
        let scale_offset_y = self.rect.height * (self.hover_scale - 1.0) * 0.5;
        
        let scaled_rect = Rectangle::new(
            self.rect.x - scale_offset_x,
            self.rect.y - scale_offset_y + self.press_offset,
            self.rect.width * self.hover_scale,
            self.rect.height * self.hover_scale,
        );

        // Draw shadow
        if !self.is_pressed {
            d.draw_rectangle(
                (scaled_rect.x + 4.0) as i32,
                (scaled_rect.y + 4.0) as i32,
                scaled_rect.width as i32,
                scaled_rect.height as i32,
                Color::new(0, 0, 0, 40),
            );
        }

        // Draw button background with interpolated color
        let current_color = Color::new(
            self.base_color.r + ((self.hover_color.r as i32 - self.base_color.r as i32) as f32 * self.animation_timer) as u8,
            self.base_color.g + ((self.hover_color.g as i32 - self.base_color.g as i32) as f32 * self.animation_timer) as u8,
            self.base_color.b + ((self.hover_color.b as i32 - self.base_color.b as i32) as f32 * self.animation_timer) as u8,
            255,
        );
        
        // Draw main button body
        d.draw_rectangle(
            scaled_rect.x as i32,
            scaled_rect.y as i32,
            scaled_rect.width as i32,
            scaled_rect.height as i32,
            current_color,
        );

        // Draw button border
        d.draw_rectangle_lines(
            scaled_rect.x as i32,
            scaled_rect.y as i32,
            scaled_rect.width as i32,
            scaled_rect.height as i32,
            Color::new(0, 0, 0, 255),
        );

        // Draw text with perfect centering
        let text_width = d.measure_text(&self.text, self.font_size);
        let text_x = scaled_rect.x as i32 + ((scaled_rect.width as i32 - text_width) / 2);
        let text_y = scaled_rect.y as i32 + ((scaled_rect.height as i32 - self.font_size) / 2);
        
        // Draw text shadow
        d.draw_text(
            &self.text,
            text_x + 1,
            text_y + 1,
            self.font_size,
            Color::new(0, 0, 0, 30),
        );
        
        // Draw main text
        d.draw_text(
            &self.text,
            text_x,
            text_y,
            self.font_size,
            Color::BLACK,
        );
    }

    fn is_hovered(&self, mouse_pos: Vector2) -> bool {
        self.rect.check_collision_point_rec(mouse_pos)
    }

    fn is_clicked(&self, rl: &RaylibHandle) -> bool {
        let mouse_pos = rl.get_mouse_position();
        self.is_hovered(mouse_pos) && rl.is_mouse_button_pressed(MouseButton::MOUSE_BUTTON_LEFT)
    }
}

// Enums, Structs, Variables, And functions that are used by the editor
enum EditorTab {
    Build,
    _Edit,
    _Delete
}

// Variables put in a function so that they can be defined here
fn define_editor_variables() -> () {
    let mut _active_tab = EditorTab::Build;
}

fn main() {
    let (mut rl, thread) = raylib::init()
        .size(800, 600)
        .title("Geometry Rays")
        .build();

    let (_stream, stream_handle) = OutputStream::try_default().unwrap();
    let sink = Sink::try_new(&stream_handle).unwrap();

    rl.set_target_fps(60);
    let logo_image = Image::load_image("Resources/logo.png").expect("Failed to load image");
    rl.set_window_icon(&logo_image);

    // Create main menu buttons
    let mut play_button = Button::new(300.0, 250.0, 200.0, 50.0, "Play", 24);
    let mut editor_button = Button::new(300.0, 320.0, 200.0, 50.0, "Level Editor", 24);
    let mut restart_button = Button::new(300.0, 320.0, 200.0, 50.0, "Restart", 24);
    
    // Create online level buttons
    let mut menu_button = Button::new(20.0, 20.0, 200.0, 50.0, "Back to Menu", 24);
    let mut create_button = Button::new(120.0, 230.0, 150.0, 150.0, "Create", 30);
    let mut featured_button = Button::new(320.0, 230.0, 150.0, 150.0, "Featured", 30);
    let mut search_button = Button::new(520.0, 230.0, 150.0, 150.0, "Search", 30);

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
    let mut not_done_yet_text = false;
    define_editor_variables();

    // Color channels
    let cc_1001 = Color { r:0, g:0, b:50, a:255 };
    let cc_1002 = Color { r:0, g:0, b:100, a:255 };
    let cc_1003 = Color::BLUE;
    let cc_1004 = Color::WHITE;

    // Load textures
    let game_bg = rl.load_texture(&thread, "Resources/default-bg.png")
        .expect("Failed to load background texture");
    let menu_bg = rl.load_texture(&thread, "Resources/default-bg-no-gradient.png")
        .expect("Failed to load menu background texture");
    let spike_texture = rl.load_texture(&thread, "Resources/spike.png")
        .expect("Failed to load spike texture");
    let logo = rl.load_texture(&thread, "Resources/logo.png")
        .expect("Failed to load logo texture");
    let ground_texture = rl.load_texture(&thread, "Resources/ground.png")
        .expect("Failed to load ground texture");
    let discord_icon = rl.load_texture(&thread, "Resources/discord-icon.png")
        .expect("Failed to load discord icon texture");

    // Audio setup
    let menu_loop_file = BufReader::new(File::open("Resources/menu-loop.mp3").expect("Failed to open MP3 file"));
    let menu_loop = Decoder::new(menu_loop_file).expect("Failed to decode MP3 file").repeat_infinite();
    sink.append(menu_loop);

    // Discord button setup
    let padding = 20.0;
    let icon_size = 32.0;
    let discord_rect = Rectangle::new(
        800.0 - icon_size - padding,
        600.0 - icon_size - padding,
        icon_size,
        icon_size
    );

    while !rl.window_should_close() {
        let space_down = rl.is_key_down(KeyboardKey::KEY_SPACE);
        let mouse_down = rl.is_mouse_button_down(MouseButton::MOUSE_BUTTON_LEFT);
        let delta_time = rl.get_frame_time();
        let mouse_pos = rl.get_mouse_position();

        // Update buttons based on game state
        match game_state {
            GameState::Menu => {
                play_button.update(&rl, delta_time);
                editor_button.update(&rl, delta_time);

                not_done_yet_text = false;

                // Check for Discord icon click
                if discord_rect.check_collision_point_rec(mouse_pos) && 
                   rl.is_mouse_button_pressed(MouseButton::MOUSE_BUTTON_LEFT) {
                    let _ = webbrowser::open("https://discord.gg/XV9Qsvmbfj");
                }

                if play_button.is_clicked(&rl) {
                    game_state = GameState::Playing;
                    player.y = 500.0;
                    world_offset = 0.0;
                    obstacles = vec![generate_spike(800.0), generate_spike(1100.0)];
                    rotation = 0.0;
                }

                if editor_button.is_clicked(&rl) {
                    game_state = GameState::CreatorMenu;
                }

                sink.play();
            }
            GameState::Playing => {
                if is_on_ground && (space_down || mouse_down) {
                    velocity_y = jump_force;
                    is_on_ground = false;
                }

                world_offset -= movement_speed;
                velocity_y += gravity;
                player.y += velocity_y;
                
                if player.y >= 500.0 {
                    player.y = 500.0;
                    velocity_y = 0.0;
                    is_on_ground = true;
                    rotation = 0.0;
                } else {
                    rotation += 5.0;
                }
                
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
                
                for obstacle in obstacles.iter_mut() {
                    if obstacle.x + world_offset < -50.0 {
                        obstacle.x = 800.0 + rand::thread_rng().gen_range(100.0..400.0);
                    }
                }
            }
            GameState::GameOver => {
                restart_button.update(&rl, delta_time);
                
                if restart_button.is_clicked(&rl) {
                    game_state = GameState::Menu;
                    attempt += 1;
                }
            }
            GameState::CreatorMenu => {
                menu_button.update(&rl, delta_time);
                create_button.update(&rl, delta_time);
                featured_button.update(&rl, delta_time);
                search_button.update(&rl, delta_time);
                
                if menu_button.is_clicked(&rl) {
                    game_state = GameState::Menu;
                }

                if create_button.is_clicked(&rl) {
                    game_state = GameState::Editor;
                }

                if featured_button.is_clicked(&rl) {
                    not_done_yet_text = true;
                }

                if search_button.is_clicked(&rl) {
                    not_done_yet_text = true;
                }
            }
            GameState::Editor => {
                println!("Nothing here yet");
            }
        }

        // Rendering
        let mut d = rl.begin_drawing(&thread);
        match game_state {
            GameState::Menu => {
                d.clear_background(Color::WHITE);
                d.draw_texture_ex(&menu_bg, Vector2::new(-200.0, -250.0), 0.0, 0.8, Color { r:50, g:50, b:50, a:255 });

                d.draw_text("Geometry Rays", 220, 150, 50, Color::WHITE);
                
                play_button.draw(&mut d);
                editor_button.draw(&mut d);

                d.draw_text(&format!("Version: {}", version), 10, 10, 15, Color::WHITE);

                d.draw_rectangle_pro(
                    Rectangle::new(360.0, 60.0, 100.0, 100.0),
                    Vector2::new(40.0 / 2.0, 40.0 / 2.0),
                    0.0,
                    Color::BLACK,
                );

                d.draw_texture_ex(&logo, Vector2::new(350.0, 50.0), 0.0, 0.1, Color::WHITE);

                // Draw Discord icon with hover effect
                let discord_color = if discord_rect.check_collision_point_rec(mouse_pos) {
                    Color::new(200, 200, 200, 255)
                } else {
                    Color::WHITE
                };

                d.draw_texture_ex(
                    &discord_icon,
                    Vector2::new(discord_rect.x, discord_rect.y),
                    0.0,
                    icon_size / discord_icon.height() as f32,
                    discord_color,
                );
            }
            GameState::Playing => {
                d.clear_background(Color::WHITE);
                d.draw_texture_ex(&game_bg, Vector2::new(0.0, -150.0), 0.0, 0.7, cc_1001);
                
                d.draw_rectangle_pro(
                    player,
                    Vector2::new(player.width / 2.0, player.height / 2.0),
                    rotation,
                    cc_1003,
                );
                
                // Draw ground
                for i in 0..6 {
                    d.draw_texture_ex(
                        &ground_texture,
                        Vector2::new(i as f32 * 150.0, 520.0),
                        0.0,
                        0.2,
                        cc_1002,
                    );
                }

                // Draw obstacles
                for obstacle in &obstacles {
                    let actual_x = obstacle.x + world_offset;
                    d.draw_texture_ex(&spike_texture, Vector2::new(actual_x, 440.0), 0.0, 0.1, cc_1004);
                }

                d.draw_text(&format!("Attempt: {}", attempt), 10, 10, 20, Color::RED);
            }
            GameState::GameOver => {
                d.clear_background(Color::WHITE);
                d.draw_texture_ex(&menu_bg, Vector2::new(0.0, -100.0), 0.0, 0.8, Color::DARKRED);

                d.draw_text("Game Over!", 250, 150, 50, Color::WHITE);
                d.draw_text(&format!("Attempts: {}", attempt), 330, 250, 20, Color::WHITE);
                
                restart_button.draw(&mut d);
            }
            GameState::CreatorMenu => {
                d.clear_background(Color::WHITE);
                d.draw_texture_ex(&menu_bg, Vector2::new(-200.0, -250.0), 0.0, 0.8, Color { r:50, g:50, b:50, a:255 });
                
                // d.draw_text("Editor will be added eventually!", 50, 250, 45, Color::WHITE);
                menu_button.draw(&mut d);
                create_button.draw(&mut d);
                featured_button.draw(&mut d);
                search_button.draw(&mut d);

                if not_done_yet_text {
                    d.draw_text("This will be added eventually!", 250, 30, 30, Color::WHITE);
                }
            }
            GameState::Editor => {
                d.clear_background(Color::WHITE);
                d.draw_texture_ex(&game_bg, Vector2::new(0.0, -150.0), 0.0, 0.7, cc_1001);
            }
        }
    }
}

fn generate_spike(x: f32) -> Rectangle {
    Rectangle::new(x, 470.0, 50.0, 50.0)
}

fn check_collision_triangle_rectangle(
    x1: f32, y1: f32, x2: f32, y2: f32, x3: f32, y3: f32, rect: Rectangle,
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

use raylib::prelude::*;
use rodio::{Decoder, OutputStream, Sink, Source};
use std::fs::File;
use std::fs;
use std::io::BufReader;
use std::process::exit;
use webbrowser;
use std::collections::HashMap;

use reqwest::Client;
use reqwest::Proxy;
use tokio::net::TcpStream;

enum GameState {
    Menu,
    Playing,
    GameOver,
    CreatorMenu,
    Editor,
    LevelOptions,
    LevelSelect,
}

struct Button {
    rect: Rectangle,
    text: String,
    font_size: i32,
    base_color: Color,
    hover_scale: f32,
    press_offset: f32,
    is_pressed: bool,
    animation_timer: f32,
    is_disabled: bool,
}

impl Button {
    fn new(x: f32, y: f32, width: f32, height: f32, text: &str, font_size: i32, is_disabled: bool) -> Self {
        Button {
            rect: Rectangle::new(x, y, width, height),
            text: text.to_string(),
            font_size,
            base_color: Color::WHITE,
            hover_scale: 1.0,
            press_offset: 0.0,
            is_pressed: false,
            animation_timer: 0.0,
            is_disabled: is_disabled,
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
        
        // Draw main button body
        d.draw_rectangle(
            scaled_rect.x as i32,
            scaled_rect.y as i32,
            scaled_rect.width as i32,
            scaled_rect.height as i32,
            if self.is_disabled { Color::BLACK } else { self.base_color },
        );

        // Old way of drawing button borders
        // d.draw_rectangle_lines(
        //     scaled_rect.x as i32,
        //     scaled_rect.y as i32,
        //     scaled_rect.width as i32,
        //     scaled_rect.height as i32,
        //     Color::new(0, 0, 0, 255),
        // );

        // Draw button borders
        d.draw_rectangle_rounded_lines(scaled_rect, 0.0, 4, 5.0, if self.is_disabled { Color::WHITE } else { Color::BLACK });

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
            if self.is_disabled { Color::WHITE } else { Color::BLACK },
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

// Check to see if the tor protocol is running
async fn is_tor_running() -> bool {
    match TcpStream::connect("127.0.0.1:9050").await {
        Ok(_) => true,
        Err(_) => false,
    }
}

async fn make_request(url: String) -> String {
    let proxy = Proxy::all("socks5h://127.0.0.1:9050")
        .expect("Failed to set up proxy");

    let client = Client::builder()
        .proxy(proxy)
        .build()
        .expect("Failed to build client");

    let res = client
        .get(url)
        .send()
        .await
        .expect("Failed to send request");

    let text = res.text().await.unwrap();
    return format!("{}", text);
}

struct MainLevel {
    name: String,
    difficulty: u8,
    song: String,
    data: String
}

#[derive(PartialEq)]
enum GameMode {
    Cube,
    Ship
}

// Enums, Structs, And functions that are used by the editor
#[derive(PartialEq)]
enum EditorTab {
    Build,
    Edit,
    Delete
}

#[derive(Debug)]
#[allow(dead_code)]
struct ObjectStruct {
    y: i32,
    x: i32,
    id: u32
}

#[tokio::main]
async fn main() {
    if !is_tor_running().await {
        println!("Tor is not running. Please start tor");
        exit(1)
    } else {
        println!("Tor is already running.");
    }

    let (mut rl, thread) = raylib::init()
        .size(800, 600)
        .title("Geometry Rays")
        .build();

    let (_stream, stream_handle) = OutputStream::try_default().unwrap();
    let sink = Sink::try_new(&stream_handle).unwrap();

    rl.set_target_fps(60);
    let logo_image = Image::load_image("Resources/logo.png").expect("Failed to load image");
    rl.set_window_icon(&logo_image);

    // Loading the textures for objects here so that they can be referenced in texture_ids
    let spike_texture = rl.load_texture(&thread, "Resources/spike.png")
        .expect("Failed to load spike texture");
    let block_texture = rl.load_texture(&thread, "Resources/block.png")
        .expect("Failed to load null texture");
    let pad_texture = rl.load_texture(&thread, "Resources/pad.png")
        .expect("Failed to load orb texture");
    let orb_texture = rl.load_texture(&thread, "Resources/orb.png")
        .expect("Failed to load orb texture");
    let upside_down_portal_texture = rl.load_texture(&thread, "Resources/upside-down-portal.png")
        .expect("Failed to load upside down portal texture");
    let right_side_up_portal_texture = rl.load_texture(&thread, "Resources/right-side-up-portal.png")
        .expect("Failed to load right side up portal texture");
    let short_spike_texture = rl.load_texture(&thread, "Resources/short-spike.png")
        .expect("Failed to load short spike texture");
    let cube_portal_texture = rl.load_texture(&thread, "Resources/cube-portal.png")
        .expect("Failed to load cube portal texture");
    let ship_portal_texture = rl.load_texture(&thread, "Resources/ship-portal.png")
        .expect("Failed to load ship portal texture");

    // Create main menu buttons
    let mut play_button = Button::new(300.0, 250.0, 200.0, 50.0, "Play", 24, false);
    let mut editor_button = Button::new(300.0, 320.0, 200.0, 50.0, "Custom Levels", 24, false);
    let mut restart_button = Button::new(300.0, 320.0, 200.0, 50.0, "Restart", 24, false);
    
    // Create online level buttons
    let mut menu_button = Button::new(20.0, 20.0, 200.0, 50.0, "Back to Menu", 24, false);
    let mut create_button = Button::new(120.0, 230.0, 150.0, 150.0, "Create", 30, false);
    let mut featured_button = Button::new(320.0, 230.0, 150.0, 150.0, "Featured", 30, true);
    let mut search_button = Button::new(520.0, 230.0, 150.0, 150.0, "Search", 30, true);

    // Create editor buttons
    let mut build_tab_button = Button::new(12.0, 415.0, 150.0, 50.0, "Build", 20, false);
    let mut edit_tab_button = Button::new(12.0, 475.0, 150.0, 50.0, "Edit", 20, false);
    let mut delete_tab_button = Button::new(12.0, 535.0, 150.0, 50.0, "Delete", 20, false);
    let grid_button = Button::new(0.0, 0.0, 800.0, 400.0, "", 20, false);
    let mut editor_back = Button::new(675.0, 20.0, 100.0, 50.0, "Back to Menu", 13, false);
    let mut level_options_button = Button::new(675.0, 90.0, 100.0, 50.0, "Level Options", 13, false);
    let mut playtest_button = Button::new(20.0, 100.0, 75.0, 75.0, "Playtest", 20, false);

    let mut level_options_back = Button::new(20.0, 20.0, 200.0, 50.0, "Back to Editor", 24, false);
    let red_bg_slider = Button::new(470.0, 100.0, 10.0, 150.0, "", 20, false);
    let green_bg_slider = Button::new(595.0, 100.0, 10.0, 150.0, "", 20, false);
    let blue_bg_slider = Button::new(720.0, 100.0, 10.0, 150.0, "", 20, false);

    let red_ground_slider = Button::new(470.0, 380.0, 10.0, 150.0, "", 20, false);
    let green_ground_slider = Button::new(595.0, 380.0, 10.0, 150.0, "", 20, false);
    let blue_ground_slider = Button::new(720.0, 380.0, 10.0, 150.0, "", 20, false);

    // Variables for the urls since tor urls are long af
    let tor_url = "http://georays.yuoqw7ywmixj55zxljkhqvcwunovze32df7pqemwacfaq2itqefbixad.onion/php-code/".to_string();
    let latest_version_url: String = format!("{}get-latest-version.php", tor_url).to_string();

    // Variables required for the game to work
    let mut game_state = GameState::Menu;
    let mut player = Rectangle::new(200.0, 500.0, 40.0, 40.0);
    let mut small_player = player;
    let mut velocity_y = 0.0;
    let mut gravity = 0.8;
    let mut jump_force = -13.0;
    let mut is_on_ground = true;
    let mut world_offset = 0.0;
    let movement_speed = 6.0;
    let mut rotation = 0.0;
    let mut attempt = 1;
    let version = "BETA";
    let latest_version = std::sync::Arc::new(std::sync::Mutex::new(String::from("Loading...")));
    let mut not_done_yet_text = false;
    let mut show_debug_text = false;
    let mut texture_ids: HashMap<u32, &Texture2D> = HashMap::new();
    let mut kill_player: bool = false;
    let mut on_orb: bool = false;
    let main_levels: Vec<MainLevel> = vec![
        MainLevel {
            name: "Plummet".to_string(),
            difficulty: 1,
            song: "./Resources/main-level-songs/0.mp3".to_string(),
            data: fs::read_to_string("./save-data/main-levels/0.txt")
                .expect("Failed to load main level")
        },

        MainLevel {
            name: "Color Blockade".to_string(),
            difficulty: 3,
            song: "./Resources/main-level-songs/1.mp3".to_string(),
            data: fs::read_to_string("./save-data/main-levels/1.txt")
                .expect("Failed to load main level")
        }
    ];
    let mut current_level = 0;
    let mut reset_menu_music = false;
    let mut current_gamemode = GameMode::Cube;
    let mut player_cam_y: i32 = 0;
    let mut touching_block_ceiling: bool = false;

    texture_ids.insert(1, &spike_texture);
    texture_ids.insert(2, &block_texture);
    texture_ids.insert(3, &pad_texture);
    texture_ids.insert(4, &orb_texture);
    texture_ids.insert(5, &upside_down_portal_texture);
    texture_ids.insert(6, &right_side_up_portal_texture);
    texture_ids.insert(7, &short_spike_texture);
    texture_ids.insert(8, &cube_portal_texture);
    texture_ids.insert(9, &ship_portal_texture);

    // Variables for editor stuff
    let mut active_tab = EditorTab::Build;
    let mut edit_not_done_yet = false;
    let mut objects: HashMap<u32, &str> = HashMap::new();
    let mut current_object = 1;
    let mut _advanced_page_number = 0;
    let mut cam_pos_x = 0;
    let mut cam_pos_y = 0;
    let mut object_grid: Vec<ObjectStruct> = vec![];
    let grid_size = 40;
    let mut red_bg_slider_pos: u8 = 75;
    let mut green_bg_slider_pos: u8 = 75;
    let mut blue_bg_slider_pos: u8 = 125;
    let mut level_string = fs::read_to_string("./save-data/levels/level.txt")
        .expect("Failed to load level file");
    let mut parts: Vec<&str> = level_string.split(";;;").collect();
    let mut _level_metadata = parts[0];
    let mut _object_string = parts[1];
    let mut been_to_editor: bool = false;

    let mut red_ground_slider_pos: i32 = 355;
    let mut green_ground_slider_pos: i32  = 355;
    let mut blue_ground_slider_pos: i32 = 455;

    objects.insert(1, "spike");
    objects.insert(2, "block");
    objects.insert(3, "pad");
    objects.insert(4, "orb");
    objects.insert(5, "upside down");
    objects.insert(6, "right side up");
    objects.insert(7, "short spike");
    objects.insert(8, "cube portal");
    objects.insert(9, "ship portal");

    let obj_button_off = 65.0;
    let mut obj1_button = Button::new(187.0, 415.0, 50.0, 50.0, objects.get(&1).unwrap(), 10, false);
    let mut obj2_button = Button::new(187.0 + (obj_button_off), 415.0, 50.0, 50.0, objects.get(&2).unwrap(), 10, false);
    let mut obj3_button = Button::new(187.0 + (obj_button_off * 2.0), 415.0, 50.0, 50.0, objects.get(&3).unwrap(), 10, false);
    let mut obj4_button = Button::new(187.0 + (obj_button_off * 3.0), 415.0, 50.0, 50.0, objects.get(&4).unwrap(), 10, false);
    let mut obj5_button = Button::new(187.0 + (obj_button_off * 4.0), 415.0, 50.0, 50.0, objects.get(&5).unwrap(), 10, false);
    let mut obj6_button = Button::new(187.0 + (obj_button_off * 5.0), 415.0, 50.0, 50.0, objects.get(&6).unwrap(), 10, false);
    let mut obj7_button = Button::new(187.0 + (obj_button_off * 6.0), 415.0, 50.0, 50.0, objects.get(&7).unwrap(), 10, false);
    let mut obj8_button = Button::new(187.0 + (obj_button_off * 7.0), 415.0, 50.0, 50.0, objects.get(&8).unwrap(), 10, false);
    let mut obj9_button = Button::new(187.0 + (obj_button_off * 8.0), 415.0, 50.0, 50.0, objects.get(&9).unwrap(), 10, false);

    let mut bg_red = red_bg_slider_pos - 75;
    let mut bg_green = green_bg_slider_pos - 75;
    let mut bg_blue = blue_bg_slider_pos - 75;

    let mut ground_red = red_ground_slider_pos - 355;
    let mut ground_green = green_ground_slider_pos - 355;
    let mut ground_blue = blue_ground_slider_pos - 355;

    // Color Channels
    // CC stands for Color Channel
    // 1001 is the bg
    // 1002 is the ground
    // 1003 is the player
    // 1004 is used by spikes and eventually blocks by default so basically obj color in gd
    // Everything before 1001 is just like in gd where you can use them for whatever you want
    // But custom color channels dont exist yet
    let mut cc_1001 = Color { r:bg_red, g:bg_green, b:bg_blue, a:255 };
    let mut cc_1002 = Color { r:ground_red as u8, g:ground_green as u8, b:ground_blue as u8, a:255 };
    let cc_1003 = Color::BLUE;
    let cc_1004 = Color::WHITE;

    // Load textures
    let game_bg = rl.load_texture(&thread, "Resources/default-bg.png")
        .expect("Failed to load background texture");
    let menu_bg = rl.load_texture(&thread, "Resources/default-bg-no-gradient.png")
        .expect("Failed to load menu background texture");
    let logo = rl.load_texture(&thread, "Resources/logo.png")
        .expect("Failed to load logo texture");
    let ground_texture = rl.load_texture(&thread, "Resources/ground.png")
        .expect("Failed to load ground texture");
    let discord_icon = rl.load_texture(&thread, "Resources/discord-icon.png")
        .expect("Failed to load discord icon texture");

    // Audio setup
    let menu_loop_file = BufReader::new(File::open("Resources/menu-loop.mp3").expect("Failed to open MP3 file"));
    let menu_loop = Decoder::new(menu_loop_file).expect("Failed to decode MP3 file").repeat_infinite();
    sink.append(menu_loop.clone());

    let mut level_music_file = BufReader::new(File::open("Resources/main-level-songs/0.mp3").expect("Failed to open MP3 file"));
    let mut _level_music = Decoder::new(level_music_file).expect("Failed to decode MP3 file");

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
        let slash_pressed = rl.is_key_pressed(KeyboardKey::KEY_SLASH);

        let one_pressed = rl.is_key_pressed(KeyboardKey::KEY_ONE);
        let two_pressed = rl.is_key_pressed(KeyboardKey::KEY_TWO);
        let three_pressed = rl.is_key_pressed(KeyboardKey::KEY_THREE);

        let up_arrow_down = rl.is_key_down(KeyboardKey::KEY_UP);
        let down_arrow_down = rl.is_key_down(KeyboardKey::KEY_DOWN);
        let left_arrow_down = rl.is_key_down(KeyboardKey::KEY_LEFT);
        let right_arrow_down = rl.is_key_down(KeyboardKey::KEY_RIGHT);

        let mouse_x = rl.get_mouse_x();
        let mouse_y = rl.get_mouse_y();
        let snapped_cam_x = cam_pos_x as i32;
        let snapped_cam_y = cam_pos_y as i32;
        let snapped_x = ((mouse_x + (snapped_cam_x * 5)) / grid_size) * grid_size;
        let snapped_y = ((mouse_y - (snapped_cam_y * 5)) / grid_size) * grid_size;

        cc_1001 = Color { r:bg_red, g:bg_green, b:bg_blue, a:255 };
        cc_1002 = Color { r:ground_red as u8, g:ground_green as u8, b:ground_blue as u8, a:255 };

        // Update buttons based on game state
        match game_state {
            GameState::Menu => {
                play_button.update(&rl, delta_time);
                editor_button.update(&rl, delta_time);

                if *latest_version.lock().unwrap() == "Loading..." {
                    let latest_version_clone = std::sync::Arc::clone(&latest_version);
                    let latest_version_url = latest_version_url.to_owned();
                    
                    let _ = tokio::task::spawn(async move {
                        let version = make_request(latest_version_url).await;
                        let mut latest_version = latest_version_clone.lock().unwrap();
                        *latest_version = version;
                    });
                }

                not_done_yet_text = false;

                // Check for Discord icon click
                if discord_rect.check_collision_point_rec(mouse_pos) && 
                   rl.is_mouse_button_pressed(MouseButton::MOUSE_BUTTON_LEFT) {
                    let _ = webbrowser::open("https://discord.gg/XV9Qsvmbfj");
                }

                if play_button.is_clicked(&rl) {
                    game_state = GameState::LevelSelect;
                    player.y = 500.0;
                    world_offset = 0.0;
                    rotation = 0.0;
                    gravity = 0.8;
                    jump_force = -13.0;
                    current_gamemode = GameMode::Cube;
                }

                if editor_button.is_clicked(&rl) {
                    game_state = GameState::CreatorMenu;
                }

                if slash_pressed {
                    show_debug_text = true;
                }


                if reset_menu_music {
                    sink.stop();
                    sink.append(menu_loop.clone());
                    sink.play();
                    reset_menu_music = false;
                }
            }
            GameState::Playing => {
                if kill_player == true {
                    kill_player = false;
                }

                if current_gamemode == GameMode::Cube {
                    if is_on_ground && (space_down || mouse_down) {
                        velocity_y = jump_force;
                        is_on_ground = false;
                    }
                } else if current_gamemode == GameMode::Ship {
                    if !touching_block_ceiling {
                        if mouse_down || space_down {
                            if gravity == 0.8 {
                                for _ in 0..10 {
                                    if velocity_y > -10.0 {
                                        velocity_y -= 0.1
                                    }
                                }
                            } else {
                                for _ in 0..10 {
                                    if velocity_y < 10.0 {
                                        velocity_y += 0.1
                                    }
                                }
                            }
                        } else {
                            if gravity == 0.8 {
                                for _ in 0..10 {
                                    if velocity_y < 10.0 {
                                        velocity_y += 0.1
                                    }
                                }
                            } else {
                                for _ in 0..10 {
                                    if velocity_y > -10.0 {
                                        velocity_y -= 0.1
                                    }
                                }
                            }
                        }
                    } else {
                        velocity_y = 0.0
                    }
                }

                world_offset -= movement_speed;
                if current_gamemode == GameMode::Cube {
                    velocity_y += gravity;
                }
                player.y += velocity_y;

                if player.y >= 500.0 - player_cam_y as f32 {
                    player.y = 500.0;
                    velocity_y = 0.0;
                    is_on_ground = true;
                    rotation = 0.0;
                } else {
                    rotation += 5.0;
                }

                if player.y >= 501.0 {
                    player_cam_y += velocity_y as i32;
                    player.y = 502.0
                }

                if player.y <= 50.0 {
                    player_cam_y += velocity_y as i32;
                    player.y = 49.0
                }

                // for obstacle in &obstacles {
                //     let actual_x = obstacle.x + world_offset;
                //     if check_collision_triangle_rectangle(
                //         actual_x,
                //         obstacle.y,
                //         actual_x + 50.0,
                //         obstacle.y + 50.0,
                //         actual_x + 50.0,
                //         obstacle.y,
                //         player,
                //     ) {
                //         game_state = GameState::GameOver;
                //     }
                // }
                
                // for obstacle in obstacles.iter_mut() {
                //     if obstacle.x + world_offset < -50.0 {
                //         obstacle.x = 800.0 + rand::thread_rng().gen_range(100.0..400.0);
                //     }
                // }

                small_player = player;
                small_player.x = player.x - 10.0;
                small_player.y = player.y - 10.0;
                small_player.width = 20.0;
                small_player.height = 20.0;

                for object in &object_grid {
                    if object.id == 1 {
                        kill_player |= player.check_collision_recs(&Rectangle {
                            x: object.x as f32 + world_offset + 20.0,
                            y: object.y as f32 + 20.0 - player_cam_y as f32,
                            width: 10.0,
                            height: 20.0
                        });
                    }

                    if object.id == 2 {
                        kill_player |= small_player.check_collision_recs(&Rectangle {
                            x: object.x as f32 + world_offset,
                            y: object.y as f32 + 10.0 - player_cam_y as f32,
                            width: 3.0,
                            height: 20.0
                        });

                        if player.check_collision_recs(&Rectangle {
                            x: object.x as f32 + world_offset + 3.0,
                            y: object.y as f32 - player_cam_y as f32,
                            width: 37.0,
                            height: 3.0
                        }) {
                            is_on_ground = true;
                            rotation = 0.0;
                            if !mouse_down {
                                player.y = object.y as f32 - 21.0;
                                velocity_y = 0.0;
                            } else {
                                if gravity < 0.0 {
                                    touching_block_ceiling = true;
                                    player.y = object.y as f32 - 21.0;
                                }
                            }
                        } else {
                            touching_block_ceiling = false;
                        }

                        if player.check_collision_recs(&Rectangle {
                            x: object.x as f32 + world_offset + 3.0,
                            y: object.y as f32 + 58.0 - player_cam_y as f32,
                            width: 37.0,
                            height: 3.0
                        }) {
                            is_on_ground = true;
                            rotation = 0.0;
                            if !mouse_down {
                                player.y = object.y as f32 + 61.0;
                                velocity_y = 0.0;
                            } else {
                                if gravity > 0.0 {
                                    touching_block_ceiling = true;
                                    player.y = object.y as f32 + 61.0;
                                }
                            }
                        } else {
                            touching_block_ceiling = false;
                        }
                    }

                    if object.id == 3 {
                        if player.check_collision_recs(&Rectangle {
                            x: object.x as f32 + world_offset,
                            y: object.y as f32 + 35.0 - player_cam_y as f32,
                            width: 40.0,
                            height: 5.0
                        }) {
                            if gravity > 0.0 {
                                velocity_y = -15.0;
                            } else {
                                velocity_y = 15.0
                            }
                        }
                    }

                    if object.id == 4 {
                        if player.check_collision_recs(&Rectangle {
                            x: object.x as f32 + world_offset,
                            y: object.y as f32 - player_cam_y as f32,
                            width: 40.0,
                            height: 40.0
                        }) {
                            on_orb = true;
                            if on_orb && mouse_down {
                                if gravity > 0.0 {
                                    velocity_y = -13.0;
                                } else {
                                    velocity_y = 13.0
                                }
                                on_orb = false
                            }
                        }
                    }

                    if object.id == 5 || object.id == 6 {
                        if player.check_collision_recs(&Rectangle {
                            x: object.x as f32 + world_offset + 10.0,
                            y: object.y as f32 + 11.0 - player_cam_y as f32,
                            width: 20.0,
                            height: 80.0
                        }) {
                            if object.id == 5 {
                                jump_force = 13.0;
                                gravity = -0.8;
                            } else {
                                jump_force = -13.0;
                                gravity = 0.8;
                            }
                        }
                    }

                    if object.id == 7 {
                        kill_player |= player.check_collision_recs(&Rectangle {
                            x: object.x as f32 + world_offset + 20.0,
                            y: object.y as f32 + 25.0 - player_cam_y as f32,
                            width: 10.0,
                            height: 10.0
                        });
                    }

                    if object.id == 8 || object.id == 9 {
                        if player.check_collision_recs(&Rectangle {
                            x: object.x as f32 + world_offset + 10.0,
                            y: object.y as f32 + 11.0 - player_cam_y as f32,
                            width: 20.0,
                            height: 80.0
                        }) {
                            if object.id == 8 {
                                current_gamemode = GameMode::Cube;
                            } else {
                                current_gamemode = GameMode::Ship;
                            }
                        }
                    }
                }

                if kill_player {
                    game_state = GameState::GameOver;
                }

                if rl.is_key_pressed(KeyboardKey::KEY_B) {
                    game_state = GameState::LevelSelect;
                }

                reset_menu_music = true;
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
                    parts = level_string.split(";;;").collect();
                    _level_metadata = parts[0];
                    _object_string = parts[1];
                    let metadata_pairs: Vec<&str> = _level_metadata.split(';').collect();
                    for pair in metadata_pairs {
                        let key_value: Vec<&str> = pair.split(':').collect();
                        let key = key_value[0];
                        let value = key_value[1];
                
                        if key == "version" {
                            if value != "ALPHA" {
                                println!("Level version not recognized");
                                break;
                            }
                        } else if key == "c1001" {
                            let colors: Vec<&str> = value.split(',').collect();
                
                            bg_red = colors[0].parse::<u8>().unwrap();
                            bg_green = colors[1].parse::<u8>().unwrap();
                            bg_blue = colors[2].parse::<u8>().unwrap();
                        } else if key == "c1002" {
                            let colors: Vec<&str> = value.split(',').collect();
                
                            ground_red = colors[0].parse::<i32>().unwrap();
                            ground_green = colors[1].parse::<i32>().unwrap();
                            ground_blue = colors[2].parse::<i32>().unwrap();
                        }
                    }
                
                    let object_list: Vec<&str> = _object_string.split(';').collect();
                    for object in object_list {
                        let xyid: Vec<&str> = object.split(':').collect();
                
                        object_grid.push(ObjectStruct { y:xyid[0].parse::<i32>().unwrap(), x:xyid[1].parse::<i32>().unwrap(), id:xyid[2].parse::<u32>().unwrap() });
                    }

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
                build_tab_button.update(&rl, delta_time);
                edit_tab_button.update(&rl, delta_time);
                delete_tab_button.update(&rl, delta_time);
                level_options_button.update(&rl, delta_time);
                editor_back.update(&rl, delta_time);
                playtest_button.update(&rl, delta_time);
                obj1_button.update(&rl, delta_time);
                obj2_button.update(&rl, delta_time);
                obj3_button.update(&rl, delta_time);
                obj4_button.update(&rl, delta_time);
                obj5_button.update(&rl, delta_time);
                obj6_button.update(&rl, delta_time);
                obj7_button.update(&rl, delta_time);
                obj8_button.update(&rl, delta_time);
                obj9_button.update(&rl, delta_time);

                if build_tab_button.is_clicked(&rl) {
                    active_tab = EditorTab::Build;
                }

                if edit_tab_button.is_clicked(&rl) {
                    active_tab = EditorTab::Edit;
                }

                if delete_tab_button.is_clicked(&rl) {
                    active_tab = EditorTab::Delete;
                }

                if one_pressed {
                    active_tab = EditorTab::Build;
                }

                if two_pressed {
                    active_tab = EditorTab::Edit;
                }

                if three_pressed {
                    active_tab = EditorTab::Delete;
                }
                
                if obj1_button.is_clicked(&rl) && active_tab == EditorTab::Build {
                    current_object = 1 + _advanced_page_number;
                }

                else if obj2_button.is_clicked(&rl) && active_tab == EditorTab::Build {
                    current_object = 2 + _advanced_page_number;
                }

                else if obj3_button.is_clicked(&rl) && active_tab == EditorTab::Build {
                    current_object = 3 + _advanced_page_number;
                }

                else if obj4_button.is_clicked(&rl) && active_tab == EditorTab::Build {
                    current_object = 4 + _advanced_page_number;
                }

                else if obj5_button.is_clicked(&rl) && active_tab == EditorTab::Build {
                    current_object = 5 + _advanced_page_number;
                }

                else if obj6_button.is_clicked(&rl) && active_tab == EditorTab::Build {
                    current_object = 6 + _advanced_page_number;
                }

                else if obj7_button.is_clicked(&rl) && active_tab == EditorTab::Build {
                    current_object = 7 + _advanced_page_number;
                }

                else if obj8_button.is_clicked(&rl) && active_tab == EditorTab::Build {
                    current_object = 8 + _advanced_page_number;
                }

                else if obj9_button.is_clicked(&rl) && active_tab == EditorTab::Build {
                    current_object = 9 + _advanced_page_number;
                }

                else if grid_button.is_clicked(&rl) {
                    // let obj_x = snapped_x;
                    // let obj_y = snapped_y;
                    if !level_options_button.is_clicked(&rl) && !editor_back.is_clicked(&rl) && !playtest_button.is_clicked(&rl) {
                        if active_tab == EditorTab::Build {
                            object_grid.push(ObjectStruct { y:snapped_y, x:snapped_x, id:current_object });
                        } else if active_tab == EditorTab::Delete {
                            let mut obj_index = 0;
                            while obj_index < object_grid.len() {
                                if object_grid[obj_index].x == snapped_x && object_grid[obj_index].y == snapped_y {
                                    object_grid.remove(obj_index);
                                } else {
                                    obj_index += 1;
                                }
                            }
                        }
                    }
                }

                if level_options_button.is_clicked(&rl) {
                    game_state = GameState::LevelOptions;
                }

                if active_tab == EditorTab::Edit {
                    edit_not_done_yet = true;
                } else {
                    edit_not_done_yet = false;
                }

                if up_arrow_down {
                    cam_pos_y += 1;
                }

                if down_arrow_down {
                    cam_pos_y -= 1;
                }

                if left_arrow_down {
                    cam_pos_x -= 1;
                }

                if right_arrow_down {
                    cam_pos_x += 1;
                }

                if editor_back.is_clicked(&rl) {
                    game_state = GameState::CreatorMenu;
                }

                if playtest_button.is_clicked(&rl) {
                    player.y = 500.0;
                    world_offset = 0.0;
                    rotation = 0.0;
                    gravity = 0.8;
                    jump_force = -13.0;
                    current_gamemode = GameMode::Cube;

                    game_state = GameState::Playing;
                }

                been_to_editor = true;
            }
            GameState::LevelOptions => {
                level_options_back.update(&rl, delta_time);

                if level_options_back.is_clicked(&rl) {
                    game_state = GameState::Editor;
                }

                if red_bg_slider.is_clicked(&rl) {
                    red_bg_slider_pos = mouse_y as u8 - 25;
                    bg_red = red_bg_slider_pos - 75;
                }

                if green_bg_slider.is_clicked(&rl) {
                    green_bg_slider_pos = mouse_y as u8 - 25;
                    bg_green = green_bg_slider_pos - 75;
                }

                if blue_bg_slider.is_clicked(&rl) {
                    blue_bg_slider_pos = mouse_y as u8 - 25;
                    bg_blue = blue_bg_slider_pos - 75;
                }


                if red_ground_slider.is_clicked(&rl) {
                    red_ground_slider_pos = mouse_y - 25;
                    ground_red = red_ground_slider_pos - 355;
                }

                if green_ground_slider.is_clicked(&rl) {
                    green_ground_slider_pos = mouse_y - 25;
                    ground_green = green_ground_slider_pos - 355;
                }

                if blue_ground_slider.is_clicked(&rl) {
                    blue_ground_slider_pos = mouse_y - 25;
                    ground_blue = blue_ground_slider_pos - 355;
                }
            }
            GameState::LevelSelect => {
                if rl.is_key_pressed(KeyboardKey::KEY_LEFT) {
                    if current_level > 0 {
                        current_level -= 1;
                    }
                }

                if rl.is_key_pressed(KeyboardKey::KEY_RIGHT) {
                    if current_level < main_levels.len() - 1 {
                        current_level += 1;
                    }
                }

                if rl.is_key_pressed(KeyboardKey::KEY_SPACE) {
                    parts = main_levels[current_level].data.split(";;;").collect();
                    _level_metadata = parts[0];
                    _object_string = parts[1];
                    object_grid.clear();
                    let metadata_pairs: Vec<&str> = _level_metadata.split(';').collect();
                    for pair in metadata_pairs {
                        let key_value: Vec<&str> = pair.split(':').collect();
                        let key = key_value[0];
                        let value = key_value[1];

                        if key == "version" {
                            if value != "ALPHA" {
                                println!("Level version not recognized");
                                break;
                            }
                        } else if key == "c1001" {
                            let colors: Vec<&str> = value.split(',').collect();

                            bg_red = colors[0].parse::<u8>().unwrap();
                            bg_green = colors[1].parse::<u8>().unwrap();
                            bg_blue = colors[2].parse::<u8>().unwrap();
                        } else if key == "c1002" {
                            let colors: Vec<&str> = value.split(',').collect();

                            ground_red = colors[0].parse::<i32>().unwrap();
                            ground_green = colors[1].parse::<i32>().unwrap();
                            ground_blue = colors[2].parse::<i32>().unwrap();
                        }
                    }

                    let object_list: Vec<&str> = _object_string.split(';').collect();
                    for object in object_list {
                        let xyid: Vec<&str> = object.split(':').collect();

                        object_grid.push(ObjectStruct { y:xyid[0].parse::<i32>().unwrap(), x:xyid[1].parse::<i32>().unwrap(), id:xyid[2].parse::<u32>().unwrap() });
                    }

                    level_music_file = BufReader::new(File::open(format!("{}", main_levels[current_level].song)).expect("Failed to open MP3 file"));
                    _level_music = Decoder::new(level_music_file).expect("Failed to decode MP3 file");
                    sink.stop();
                    sink.append(_level_music);
                    sink.play();

                    player.y = 500.0;
                    world_offset = 0.0;
                    rotation = 0.0;
                    gravity = 0.8;
                    jump_force = -13.0;
                    current_gamemode = GameMode::Cube;

                    game_state = GameState::Playing;
                }

                if reset_menu_music {
                    sink.stop();
                    sink.append(menu_loop.clone());
                    sink.play();
                    reset_menu_music = false;
                }

                if rl.is_key_pressed(KeyboardKey::KEY_B) {
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

                play_button.draw(&mut d);
                editor_button.draw(&mut d);

                d.draw_text(&format!("Version: {}", version), 10, 10, 15, Color::WHITE);
                d.draw_text(&format!("Latest Version: {}", *latest_version.lock().unwrap()), 10, 30, 15, Color::WHITE);

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
                        Vector2::new(i as f32 * 150.0, 520.0 - player_cam_y as f32),
                        0.0,
                        0.2,
                        cc_1002,
                    );
                }

                for i in &object_grid {
                    let object_x = i.x as f32 + world_offset as f32;
                    let object_y = i.y as f32 + 0 as f32;
                    d.draw_texture_ex(&texture_ids.get(&i.id).unwrap(), Vector2::new(object_x, object_y - player_cam_y as f32), 0.0, 0.05, cc_1004);
                }

                // Draw obstacles (old)
                // for obstacle in &obstacles {
                //     let actual_x = obstacle.x + world_offset;
                //     d.draw_texture_ex(&texture_ids.get(&1).unwrap(), Vector2::new(actual_x, 480.0), 0.0, 0.05, cc_1004);
                // }

                if show_debug_text {
                    for object in &object_grid {
                        if object.id == 1 {
                            d.draw_rectangle_lines(
                                object.x + world_offset as i32 + 15,
                                object.y + 10 - player_cam_y,
                                10,
                                20,
                                Color::RED
                            );
                        }

                        if object.id == 2 {
                            d.draw_rectangle_lines(
                                object.x + world_offset as i32,
                                object.y + 10 - player_cam_y,
                                3,
                                20,
                                Color::RED
                            );

                            d.draw_rectangle_lines(
                                object.x + world_offset as i32 + 3,
                                object.y - player_cam_y,
                                37,
                                3,
                                Color::BLUEVIOLET
                            );

                            d.draw_rectangle_lines(
                                object.x + world_offset as i32 + 3,
                                object.y + 58 - player_cam_y,
                                37,
                                3,
                                Color::BLUEVIOLET
                            );
                        }

                        if object.id == 3 {
                            d.draw_rectangle_lines(
                                object.x + world_offset as i32,
                                object.y + 35 - player_cam_y,
                                40,
                                5,
                                Color::TEAL
                            );
                        }

                        if object.id == 4 {
                            d.draw_rectangle_lines(
                                object.x + world_offset as i32,
                                object.y - player_cam_y,
                                40,
                                40,
                                Color::TEAL
                            );
                        }

                        if object.id == 5 || object.id == 6 {
                            d.draw_rectangle_lines(
                                object.x + world_offset as i32 + 10,
                                object.y + 11 - player_cam_y,
                                20,
                                80,
                                Color::TEAL
                            );
                        }

                        if object.id == 7 {
                            d.draw_rectangle_lines(
                                object.x + world_offset as i32 + 15,
                                object.y + 25 - player_cam_y,
                                10,
                                10,
                                Color::RED
                            );
                        }

                        if object.id == 8 || object.id == 9 {
                            d.draw_rectangle_lines(
                                object.x + world_offset as i32 + 10,
                                object.y + 11 - player_cam_y,
                                20,
                                80,
                                Color::TEAL
                            );
                        }
                    }

                    d.draw_rectangle_lines(
                        small_player.x as i32,
                        small_player.y as i32,
                        small_player.width as i32,
                        small_player.height as i32,
                        Color::YELLOW
                    );
                }

                d.draw_text(&format!("Attempt: {}", attempt), 10, 10, 20, Color::WHITE);

                if show_debug_text {
                    d.draw_text(&format!("Velocity Y: {}", velocity_y), 10, 40, 20, Color::GREEN);
                    d.draw_text(&format!("On Ground: {}", is_on_ground), 10, 70, 20, Color::GREEN);
                    d.draw_text(&format!("Touching block ceiling: {}", touching_block_ceiling), 10, 100, 20, Color::GREEN);
                }
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

                for i in &object_grid {
                    let object_x = i.x as f32 - cam_pos_x as f32 * 5.0;
                    let object_y = i.y as f32 + cam_pos_y as f32 * 5.0;
                    d.draw_texture_ex(&texture_ids.get(&i.id).unwrap(), Vector2::new(object_x, object_y), 0.0, 0.05, cc_1004);
                }

                // Draw ground
                for i in 0..6 {
                    d.draw_texture_ex(
                        &ground_texture,
                        Vector2::new(i as f32 * 150.0, cam_pos_y as f32 * 5.0 + 520.0),
                        0.0,
                        0.2,
                        cc_1002,
                    );
                }

                d.draw_rectangle_gradient_v(0, cam_pos_y * 5 + 590, 800, 100, Color { r:0, g:0, b:0, a:0 } , Color::BLACK);
                d.draw_rectangle(0, cam_pos_y * 5 + 690, 800, 500, Color::BLACK);

                d.draw_rectangle(0, 400, 800, 200, Color { r:30, g:30, b:30, a:100 });

                d.draw_line(175, 400, 175, 600, Color::WHITE);

                build_tab_button.draw(&mut d);
                edit_tab_button.draw(&mut d);
                delete_tab_button.draw(&mut d);
                level_options_button.draw(&mut d);
                editor_back.draw(&mut d);
                playtest_button.draw(&mut d);

                if edit_not_done_yet {
                    d.draw_text("Edit tab coming soon!", 270, 490, 40, Color::WHITE);
                }


                // Draw all the object buttons
                if active_tab == EditorTab::Build {
                    obj1_button.draw(&mut d);
                    obj2_button.draw(&mut d);
                    obj3_button.draw(&mut d);
                    obj4_button.draw(&mut d);
                    obj5_button.draw(&mut d);
                    obj6_button.draw(&mut d);
                    obj7_button.draw(&mut d);
                    obj8_button.draw(&mut d);
                    obj9_button.draw(&mut d);
                }

                d.draw_text(&format!("Selected Object: {}", objects.get(&current_object).unwrap()), 10, 10, 20, Color::WHITE);
                if show_debug_text {
                    d.draw_text(&format!("Camera pos X: {}", cam_pos_x), 10, 40, 20, Color::GREEN);
                    d.draw_text(&format!("Camera pos Y: {}", cam_pos_y), 10, 70, 20, Color::GREEN);
                    d.draw_text(&format!("Advanced Page Number: {}", _advanced_page_number), 10, 100, 20, Color::GREEN);
                    d.draw_text(&format!("Mouse X On Grid: {}", snapped_x), 10, 130, 20, Color::GREEN);
                    d.draw_text(&format!("Mouse Y On Grid: {}", snapped_y), 10, 160, 20, Color::GREEN);
                    d.draw_text(&format!("Mouse X: {}", mouse_x), 10, 190, 20, Color::GREEN);
                    d.draw_text(&format!("Mouse Y: {}", mouse_y), 10, 220, 20, Color::GREEN);

                    d.draw_text(&format!("Object Grid: {:?}", object_grid), 10, 250, 20, Color::GREEN);
                }
            }
            GameState::LevelOptions => {
                d.clear_background(Color {r:0, g:0, b:75, a:255});

                level_options_back.draw(&mut d);

                d.draw_rectangle(425, 20, 100, 50, Color {r:255, g:0, b:0, a:255});
                d.draw_rectangle(550, 20, 100, 50, Color {r:0, g:255, b:0, a:255});
                d.draw_rectangle(675, 20, 100, 50, Color {r:0, g:0, b:255, a:255});

                d.draw_rectangle_rounded_lines(Rectangle { x:425.0, y:20.0, width:100.0, height:50.0 }, 0.0, 4, 5.0, Color::BLACK);
                d.draw_rectangle_rounded_lines(Rectangle { x:550.0, y:20.0, width:100.0, height:50.0 }, 0.0, 4, 5.0, Color::BLACK);
                d.draw_rectangle_rounded_lines(Rectangle { x:675.0, y:20.0, width:100.0, height:50.0 }, 0.0, 4, 5.0, Color::BLACK);

                d.draw_rectangle(470, 100, 10, 150, Color {r:255, g:0, b:0, a:255});
                d.draw_rectangle(595, 100, 10, 150, Color {r:0, g:255, b:0, a:255});
                d.draw_rectangle(720, 100, 10, 150, Color {r:0, g:0, b:255, a:255});

                d.draw_rectangle_rounded_lines(Rectangle {x: 470.0, y: 100.0, width:10.0, height:150.0}, 0.0, 4, 5.0, Color::BLACK);
                d.draw_rectangle_rounded_lines(Rectangle {x: 595.0, y: 100.0, width:10.0, height:150.0}, 0.0, 4, 5.0, Color::BLACK);
                d.draw_rectangle_rounded_lines(Rectangle {x: 720.0, y: 100.0, width:10.0, height:150.0}, 0.0, 4, 5.0, Color::BLACK);

                d.draw_rectangle(450, red_bg_slider_pos as i32, 50, 50, Color::WHITE);
                d.draw_rectangle(575, green_bg_slider_pos as i32, 50, 50, Color::WHITE);
                d.draw_rectangle(700, blue_bg_slider_pos as i32, 50, 50, Color::WHITE);

                d.draw_rectangle_rounded_lines(Rectangle {x: 450.0, y: red_bg_slider_pos as f32, width:50.0, height:50.0}, 0.0, 4, 5.0, Color::BLACK);
                d.draw_rectangle_rounded_lines(Rectangle {x: 575.0, y: green_bg_slider_pos as f32, width:50.0, height:50.0}, 0.0, 4, 5.0, Color::BLACK);
                d.draw_rectangle_rounded_lines(Rectangle {x: 700.0, y: blue_bg_slider_pos as f32, width:50.0, height:50.0}, 0.0, 4, 5.0, Color::BLACK);

                d.draw_text(&format!("{}", bg_red), 435, 25, 50, Color::BLACK);
                d.draw_text(&format!("{}", bg_green), 560, 25, 50, Color::BLACK);
                d.draw_text(&format!("{}", bg_blue), 685, 25, 50, Color::BLACK);

                d.draw_rectangle(425, 300, 100, 50, Color {r:255, g:0, b:0, a:255});
                d.draw_rectangle(550, 300, 100, 50, Color {r:0, g:255, b:0, a:255});
                d.draw_rectangle(675, 300, 100, 50, Color {r:0, g:0, b:255, a:255});

                d.draw_rectangle_rounded_lines(Rectangle { x:425.0, y:300.0, width:100.0, height:50.0 }, 0.0, 4, 5.0, Color::BLACK);
                d.draw_rectangle_rounded_lines(Rectangle { x:550.0, y:300.0, width:100.0, height:50.0 }, 0.0, 4, 5.0, Color::BLACK);
                d.draw_rectangle_rounded_lines(Rectangle { x:675.0, y:300.0, width:100.0, height:50.0 }, 0.0, 4, 5.0, Color::BLACK);

                d.draw_rectangle(470, 380, 10, 150, Color {r:255, g:0, b:0, a:255});
                d.draw_rectangle(595, 380, 10, 150, Color {r:0, g:255, b:0, a:255});
                d.draw_rectangle(720, 380, 10, 150, Color {r:0, g:0, b:255, a:255});

                d.draw_rectangle_rounded_lines(Rectangle {x: 470.0, y: 380.0, width:10.0, height:150.0}, 0.0, 4, 5.0, Color::BLACK);
                d.draw_rectangle_rounded_lines(Rectangle {x: 595.0, y: 380.0, width:10.0, height:150.0}, 0.0, 4, 5.0, Color::BLACK);
                d.draw_rectangle_rounded_lines(Rectangle {x: 720.0, y: 380.0, width:10.0, height:150.0}, 0.0, 4, 5.0, Color::BLACK);

                d.draw_rectangle(450, red_ground_slider_pos as i32, 50, 50, Color::WHITE);
                d.draw_rectangle(575, green_ground_slider_pos as i32, 50, 50, Color::WHITE);
                d.draw_rectangle(700, blue_ground_slider_pos as i32, 50, 50, Color::WHITE);

                d.draw_rectangle_rounded_lines(Rectangle {x: 450.0, y: red_ground_slider_pos as f32, width:50.0, height:50.0}, 0.0, 4, 5.0, Color::BLACK);
                d.draw_rectangle_rounded_lines(Rectangle {x: 575.0, y: green_ground_slider_pos as f32, width:50.0, height:50.0}, 0.0, 4, 5.0, Color::BLACK);
                d.draw_rectangle_rounded_lines(Rectangle {x: 700.0, y: blue_ground_slider_pos as f32, width:50.0, height:50.0}, 0.0, 4, 5.0, Color::BLACK);

                d.draw_text(&format!("{}", ground_red), 435, 305, 50, Color::BLACK);
                d.draw_text(&format!("{}", ground_green), 560, 305, 50, Color::BLACK);
                d.draw_text(&format!("{}", ground_blue), 685, 305, 50, Color::BLACK);
            }
            GameState::LevelSelect => {
                d.clear_background(Color::BLACK);
                d.draw_text(&format!("{}", main_levels[current_level].name), 275, 275, 50, Color::WHITE);
                d.draw_text(&format!("{}", main_levels[current_level].difficulty), 350, 200, 50, Color::WHITE);
            }
        }
    }

    if been_to_editor {
        level_string = format!(
            "version:ALPHA;name:hi;desc:testing level loading;c1001:{},{},{};c1002:{},{},{};c1004:255,255,255;bg:1;grnd:1;;;",

            bg_red,
            bg_green,
            bg_blue,

            ground_red,
            ground_green,
            ground_blue
        ).to_string();

        for object in object_grid {
            level_string.push_str( &format!("{}:{}:{};", object.y, object.x, object.id));
        }

        level_string.pop();

        let write_result = fs::write("./save-data/levels/level.txt", level_string);

        println!("{:?}", write_result);
    }

    // Print statements to make unused variable warnings go away because rust is stupid
    println!("{}", on_orb);
    println!("{:?}", cc_1001);
    println!("{:?}", cc_1002);
}
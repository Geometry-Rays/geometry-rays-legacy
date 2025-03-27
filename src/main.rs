use raylib::prelude::*;
use rodio::{Decoder, OutputStream, Sink, Source};
use std::fs::File;
use std::fs;
use std::io::BufReader;
use webbrowser;
use std::collections::HashMap;

use ez_reqwest::*;

mod funcs;
mod types;
use funcs::*;
use types::*;

#[tokio::main]
async fn main() {
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
    let _null_texture = rl.load_texture(&thread, "Resources/null.png")
        .expect("Failed to load null texture");
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
    let outline_block1 = rl.load_texture(&thread, "Resources/blocks/outline-block.png")
        .expect("Failed to load outline block");
    let outline_block2 = rl.load_texture(&thread, "Resources/blocks/outline-block-variant1.png")
        .expect("Failed to load outline block variant 1");
    let outline_block3 = rl.load_texture(&thread, "Resources/blocks/outline-block-variant2.png")
        .expect("Failed to load outline block variant 2");
    let outline_block4 = rl.load_texture(&thread, "Resources/blocks/outline-block-variant3.png")
        .expect("Failed to load outline block variant 3");
    let outline_block5 = rl.load_texture(&thread, "Resources/blocks/outline-block-variant4.png")
        .expect("Failed to load outline block variant 4");
    let end_trigger_texture = rl.load_texture(&thread, "Resources/end-trigger.png")
        .expect("Failed to load end trigger texture");
    let black_block_texture = rl.load_texture(&thread, "Resources/blocks/black-block.png")
        .expect("Failed to load black block texture");
    let half_speed_texture = rl.load_texture(&thread, "Resources/speed/05x.png")
        .expect("Failed to load 0.5x speed portal texture");
    let normal_speed_texture = rl.load_texture(&thread, "Resources/speed/1x.png")
        .expect("Failed to load 1x speed portal texture");
    let double_speed_texture = rl.load_texture(&thread, "Resources/speed/2x.png")
        .expect("Failed to load 2x speed portal texture");
    let triple_speed_texture = rl.load_texture(&thread, "Resources/speed/3x.png")
        .expect("Failed to load 3x speed portal texture");
    let gravity_pad_texture = rl.load_texture(&thread, "Resources/gravity-pad.png")
        .expect("Failed to load gravity pad texture");
    let gravity_orb_texture = rl.load_texture(&thread, "Resources/gravity-orb.png")
        .expect("Failed to load gravity orb texture");
    let color_trigger_texture = rl.load_texture(&thread, "Resources/color.png")
        .expect("Failed to load color trigger texture");

    // Create main menu buttons
    let mut play_button = Button::new(300.0, 250.0, 200.0, 50.0, "Play", 24, false);
    let mut editor_button = Button::new(300.0, 320.0, 200.0, 50.0, "Custom Levels", 24, false);
    let mut restart_button = Button::new(300.0, 320.0, 200.0, 50.0, "Restart", 24, false);
    let mut account_page_button = Button::new(rl.get_screen_width() as f32 - 220.0, 20.0, 200.0, 50.0, "Account Page", 24, false);

    // Create online level buttons
    let mut menu_button = Button::new(20.0, 20.0, 200.0, 50.0, "Back to Menu", 24, false);
    let mut create_button = Button::new(120.0, 230.0, 150.0, 150.0, "Create", 30, false);
    let mut featured_button = Button::new(320.0, 230.0, 150.0, 150.0, "Featured", 30, true);
    let mut search_button = Button::new(520.0, 230.0, 150.0, 150.0, "Search", 30, false);
    let mut keybinds_button = Button::new(rl.get_screen_width() as f32 - 220.0, 20.0, 200.0, 50.0, "Editor Keybinds", 24, false);
    let mut download_level_button = Button::new(rl.get_screen_width() as f32 - 220.0, 80.0, 200.0, 50.0, "Download Level", 24, false);

    let mut level_id_textbox = TextBox {
        rect: Rectangle {
            x: rl.get_screen_width() as f32 - 10.0 * 30.0 / 1.9 - 20.0,
            y: 20.0,
            width: 10.0 * 30.0 / 1.9,
            height: 50.0
        },
        text: "Level ID".to_string(),
        text_size: 30,
        max_length: 5,
        spaces_allowed: false,
        active: false
    };

    // Create editor buttons
    let mut build_tab_button = Button::new(12.0, 415.0, 150.0, 50.0, "Build", 20, false);
    let mut edit_tab_button = Button::new(12.0, 475.0, 150.0, 50.0, "Edit", 20, false);
    let mut delete_tab_button = Button::new(12.0, 535.0, 150.0, 50.0, "Delete", 20, false);
    let grid_button = Button::new(0.0, 0.0, 800.0, 400.0, "", 20, false);
    let mut editor_back = Button::new(675.0, 20.0, 100.0, 50.0, "Back to Menu", 13, false);
    let mut level_options_button = Button::new(675.0, 90.0, 100.0, 50.0, "Level Options", 13, false);
    let mut level_save_button = Button::new(675.0, 160.0, 100.0, 50.0, "Save", 20, false);
    let mut playtest_button = Button::new(20.0, 150.0, 75.0, 75.0, "Playtest", 20, false);
    let mut level_upload_button = Button::new(675.0, 230.0, 100.0, 50.0, "Upload", 20, false);

    let mut level_options_back = Button::new(20.0, 20.0, 200.0, 50.0, "Back to Editor", 24, false);
    let red_bg_slider = Button::new(470.0, 100.0, 10.0, 150.0, "", 20, false);
    let green_bg_slider = Button::new(595.0, 100.0, 10.0, 150.0, "", 20, false);
    let blue_bg_slider = Button::new(720.0, 100.0, 10.0, 150.0, "", 20, false);

    let red_ground_slider = Button::new(470.0, 380.0, 10.0, 150.0, "", 20, false);
    let green_ground_slider = Button::new(595.0, 380.0, 10.0, 150.0, "", 20, false);
    let blue_ground_slider = Button::new(720.0, 380.0, 10.0, 150.0, "", 20, false);

    let mut level_complete_back_button = Button::new(300.0, 320.0, 200.0, 50.0, "Back To Menu", 24, false);

    // Account page textboxes and buttons
    let mut username_textbox = TextBox {
        rect: Rectangle {
            x: rl.get_screen_width() as f32 / 2.0 - 20.0 * 30.0 / 1.9 / 2.0,
            y: rl.get_screen_height() as f32 / 2.0 - 50.0,
            width: 20.0 * 30.0 / 1.9,
            height: 50.0
        },
        text: "Username".to_string(),
        text_size: 30,
        max_length: 20,
        spaces_allowed: false,
        active: false
    };

    let mut password_textbox = TextBox {
        rect: Rectangle {
            x: rl.get_screen_width() as f32 / 2.0 - 20.0 * 30.0 / 1.9 / 2.0,
            y: rl.get_screen_height() as f32 / 2.0 + 50.0,
            width: 20.0 * 30.0 / 1.9,
            height: 50.0
        },
        text: "Password".to_string(),
        text_size: 30,
        max_length: 20,
        spaces_allowed: false,
        active: false
    };

    let mut login_button = Button::new(
        rl.get_screen_width() as f32 / 2.0 - 100.0,
        rl.get_screen_height() as f32 - 140.0,
        200.0,
        50.0,
        "Login",
        20,
        false
    );

    let mut register_button = Button::new(
        rl.get_screen_width() as f32 / 2.0 - 100.0,
        rl.get_screen_height() as f32 - 70.0,
        200.0,
        50.0,
        "Register",
        20,
        false
    );



    let mut level_name_textbox = TextBox {
        rect: Rectangle {
            x: rl.get_screen_width() as f32 / 2.0 - 20.0 * 30.0 / 1.9 / 2.0,
            y: rl.get_screen_height() as f32 / 2.0 - 50.0,
            width: 20.0 * 30.0 / 1.9,
            height: 50.0
        },
        text: "Level Name".to_string(),
        text_size: 30,
        max_length: 20,
        spaces_allowed: true,
        active: false
    };

    let mut level_desc_textbox = TextBox {
        rect: Rectangle {
            x: rl.get_screen_width() as f32 / 2.0 - 50.0 * 30.0 / 1.9 / 2.0,
            y: rl.get_screen_height() as f32 / 2.0 + 50.0,
            width: 50.0 * 30.0 / 1.9,
            height: 50.0
        },
        text: "Level Description".to_string(),
        text_size: 30,
        max_length: 50,
        spaces_allowed: true,
        active: false
    };

    let mut upload_button = Button::new(
        rl.get_screen_width() as f32 / 2.0 - 100.0,
        rl.get_screen_height() as f32 - 140.0,
        200.0,
        50.0,
        "Upload",
        20,
        false
    );



    let mut level_play_button = Button::new(
        rl.get_screen_width() as f32 / 2.0 - 100.0,
        rl.get_screen_height() as f32 / 2.0 - 50.0,
        200.0,
        100.0,
        "Play",
        20,
        false
    );

    let mut level_rate_button = Button::new(
        20.0,
        rl.get_screen_height() as f32 - 120.0,
        100.0,
        100.0,
        "Rate",
        20,
        false
    );

    let mut submit_rating_button = Button::new(
        rl.get_screen_width() as f32 / 2.0 - 100.0,
        rl.get_screen_height() as f32 / 2.0 - 50.0,
        200.0,
        100.0,
        "Submit",
        20,
        false
    );

    let mut clear_level_button = Button::new(
        rl.get_screen_width() as f32 - 120.0,
        rl.get_screen_height() as f32 - 120.0,
        100.0,
        100.0,
        "Clear Level",
        18,
        false
    );

    let main_url = "http://georays.puppet57.xyz/php-code/".to_string();
    let latest_version_url: String = format!("{}get-latest-version.php", main_url).to_string();
    let register_url: String = format!("{}register.php", main_url).to_string();
    let login_url: String = format!("{}login.php", main_url).to_string();
    let upload_url: String = format!("{}upload-level.php", main_url).to_string();
    let download_url: String = format!("{}download-level.php", main_url);
    let rate_url: String = format!("{}rate-level.php", main_url);

    // Variables required for the game to work
    let mut game_state = GameState::Menu;
    let mut player = Rectangle::new(200.0, 500.0, 40.0, 40.0);
    let mut centered_player = Rectangle::new(player.x + player.width / 2.0, player.y + player.height / 2.0, player.width, player.height);
    let mut small_player = player;
    let mut is_on_ground = true;
    let mut world_offset = 0.0;
    let mut rotation = 0.0;
    let mut attempt = 1;
    let mut on_orb: bool = false;
    let mut kill_player: bool = false;
    let mut texture_ids: HashMap<u32, &Texture2D> = HashMap::new();
    let mut current_gamemode = GameMode::Cube;
    let mut player_cam_y: i32 = 0;
    let mut touching_block_ceiling: bool = false;

    // Physics Values
    let mut velocity_y = 0.0;
    let mut gravity = 0.8;
    let default_gravity = gravity;
    let mut jump_force = -13.0;
    let default_jump_force = jump_force;
    let mut movement_speed = 6.0;
    let default_movement_speed = movement_speed;
    let ship_power: f32 = 0.7;
    let ship_falling_speed: f32 = 0.5;

    let version = "1.3";
    let latest_version = std::sync::Arc::new(std::sync::Mutex::new(String::from("Loading...")));
    let mut not_done_yet_text = false;
    let mut show_debug_text = false;
    let main_levels: Vec<MainLevel> = vec![
        MainLevel {
            name: "Plummet".to_string(),
            difficulty: 1,
            song: "./Resources/main-level-songs/0.mp3".to_string(),
            artist: "1f1n1ty".to_string(),
            data: fs::read_to_string("./save-data/main-levels/0.txt")
                .expect("Failed to load main level")
        },

        MainLevel {
            name: "Color Blockade".to_string(),
            difficulty: 3,
            song: "./Resources/main-level-songs/1.mp3".to_string(),
            artist: "Waterflame".to_string(),
            data: fs::read_to_string("./save-data/main-levels/1.txt")
                .expect("Failed to load main level")
        },

        MainLevel {
            name: "Ultimate Destruction".to_string(),
            difficulty: 2,
            song: "./Resources/main-level-songs/2.mp3".to_string(),
            artist: "TMM43".to_string(),
            data: fs::read_to_string("./save-data/main-levels/2.txt")
                .expect("Failed to load main level")
        },

        MainLevel {
            name: "Detorium".to_string(),
            difficulty: 4,
            song: "./Resources/main-level-songs/3.mp3".to_string(),
            artist: "Fluix".to_string(),
            data: fs::read_to_string("./save-data/main-levels/3.txt")
                .expect("Failed to load main level")
        }
    ];
    let mut current_level = 0;
    let mut reset_menu_music = false;
    let mut stars: u32 = 0;
    let save_data = fs::read_to_string("./save-data/save.txt")
        .expect("Failed to read save file");
    let mut in_custom_level: bool = false;
    let mut levels_completed_vec: Vec<bool> = vec![
        false,
        false,
        false,
        false
    ];
    let mut logged_in: bool = false;
    let mut online_levels_beaten: Vec<u16> = vec![];
    let mut is_mod: bool = false;
    let default_level: &str = "version:1.3;song:0;c1001:0,0,50;c1002:0,0,100;c1004:255,255,255;bg:1;grnd:1;;;480:480:0:0:0:1";
    let mut start_pos: u16 = 0;

    let mut get_latest_version = true;
    let mut register_result = "".to_string();
    let mut login_result = "".to_string();
    let mut level_upload_result = "".to_string();
    let mut level_download_result = "".to_string();
    let mut online_level_name = "".to_string();
    let mut online_level_desc = "".to_string();
    let mut online_level_data = "".to_string();
    let mut online_level_diff: u8 = 0;
    let mut online_level_rated: bool = false;
    let mut online_level_creator = "".to_string();
    let mut show_level_not_found: bool = false;
    let mut online_level_upload_diff: u8 = 0;
    let mut online_level_rate_diff: u8 = 0;
    let mut level_rate_result = "".to_string();

    texture_ids.insert(1, &spike_texture);
    texture_ids.insert(2, &block_texture);
    texture_ids.insert(3, &pad_texture);
    texture_ids.insert(4, &orb_texture);
    texture_ids.insert(5, &upside_down_portal_texture);
    texture_ids.insert(6, &right_side_up_portal_texture);
    texture_ids.insert(7, &short_spike_texture);
    texture_ids.insert(8, &cube_portal_texture);
    texture_ids.insert(9, &ship_portal_texture);
    texture_ids.insert(10, &outline_block1);
    texture_ids.insert(11, &outline_block2);
    texture_ids.insert(12, &outline_block3);
    texture_ids.insert(13, &outline_block4);
    texture_ids.insert(14, &outline_block5);
    texture_ids.insert(15, &end_trigger_texture);
    texture_ids.insert(16, &black_block_texture);
    texture_ids.insert(17, &normal_speed_texture);
    texture_ids.insert(18, &double_speed_texture);
    texture_ids.insert(19, &triple_speed_texture);
    texture_ids.insert(20, &half_speed_texture);
    texture_ids.insert(21, &gravity_pad_texture);
    texture_ids.insert(22, &gravity_orb_texture);
    texture_ids.insert(23, &color_trigger_texture);

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
    let mut current_song: u8 = 0;
    let mut song_selected: bool = false;
    let mut from_editor: bool = false;
    let mut player_path: Vec<Vector2> = vec![];
    let mut editor_guide_scroll: u16 = 0;

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
    objects.insert(10, "outline block");
    objects.insert(11, "outline block 2");
    objects.insert(12, "outline block 3");
    objects.insert(13, "outline block 4");
    objects.insert(14, "outline block 5");
    objects.insert(15, "end trigger");
    objects.insert(16, "black block");
    objects.insert(17, "1x speed");
    objects.insert(18, "2x speed");
    objects.insert(19, "3x speed");
    objects.insert(20, "0.5x speed");
    objects.insert(21, "gravity pad");
    objects.insert(22, "gravity orb");
    objects.insert(23, "color trigger");

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
    let mut obj10_button = Button::new(187.0, 415.0 + obj_button_off, 50.0, 50.0, objects.get(&10).unwrap(), 10, false);
    let mut obj11_button = Button::new(187.0 + obj_button_off, 415.0 + obj_button_off, 50.0, 50.0, objects.get(&11).unwrap(), 10, false);
    let mut obj12_button = Button::new(187.0 + (obj_button_off * 2.0), 415.0 + obj_button_off, 50.0, 50.0, objects.get(&12).unwrap(), 10, false);
    let mut obj13_button = Button::new(187.0 + (obj_button_off * 3.0), 415.0 + obj_button_off, 50.0, 50.0, objects.get(&13).unwrap(), 10, false);
    let mut obj14_button = Button::new(187.0 + (obj_button_off * 4.0), 415.0 + obj_button_off, 50.0, 50.0, objects.get(&14).unwrap(), 10, false);
    let mut obj15_button = Button::new(187.0 + (obj_button_off * 5.0), 415.0 + obj_button_off, 50.0, 50.0, objects.get(&15).unwrap(), 10, false);
    let mut obj16_button = Button::new(187.0 + (obj_button_off * 6.0), 415.0 + obj_button_off, 50.0, 50.0, objects.get(&16).unwrap(), 10, false);
    let mut obj17_button = Button::new(187.0 + (obj_button_off * 7.0), 415.0 + obj_button_off, 50.0, 50.0, objects.get(&17).unwrap(), 10, false);
    let mut obj18_button = Button::new(187.0 + (obj_button_off * 8.0), 415.0 + obj_button_off, 50.0, 50.0, objects.get(&18).unwrap(), 10, false);
    let mut obj19_button = Button::new(187.0, 415.0 + (obj_button_off * 2.0), 50.0, 50.0, objects.get(&19).unwrap(), 10, false);
    let mut obj20_button = Button::new(187.0 + obj_button_off, 415.0 + (obj_button_off * 2.0), 50.0, 50.0, objects.get(&20).unwrap(), 10, false);
    let mut obj21_button = Button::new(187.0 + (obj_button_off * 2.0), 415.0 + (obj_button_off * 2.0), 50.0, 50.0, objects.get(&21).unwrap(), 10, false);
    let mut obj22_button = Button::new(187.0 + (obj_button_off * 3.0), 415.0 + (obj_button_off * 2.0), 50.0, 50.0, objects.get(&22).unwrap(), 10, false);
    let mut obj23_button = Button::new(187.0 + (obj_button_off * 4.0), 415.0 + (obj_button_off * 2.0), 50.0, 50.0, objects.get(&23).unwrap(), 10, false);

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
    let mut cc_1003 = Color::LIME;
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
    let star_texture = rl.load_texture(&thread, "Resources/star.png")
        .expect("Failed to load star texture");
    
    let difficulties: Vec<Texture2D> = vec![
        rl.load_texture(&thread, "./Resources/difficulties/0.png").expect("Failed to load difficulty face"),
        rl.load_texture(&thread, "./Resources/difficulties/1.png").expect("Failed to load difficulty face"),
        rl.load_texture(&thread, "./Resources/difficulties/2.png").expect("Failed to load difficulty face"),
        rl.load_texture(&thread, "./Resources/difficulties/3.png").expect("Failed to load difficulty face"),
        rl.load_texture(&thread, "./Resources/difficulties/4.png").expect("Failed to load difficulty face"),
        rl.load_texture(&thread, "./Resources/difficulties/5.png").expect("Failed to load difficulty face"),
        rl.load_texture(&thread, "./Resources/difficulties/6.png").expect("Failed to load difficulty face"),
        rl.load_texture(&thread, "./Resources/difficulties/7.png").expect("Failed to load difficulty face"),
        rl.load_texture(&thread, "./Resources/difficulties/8.png").expect("Failed to load difficulty face"),
        rl.load_texture(&thread, "./Resources/difficulties/9.png").expect("Failed to load difficulty face"),
        rl.load_texture(&thread, "./Resources/difficulties/10.png").expect("Failed to load difficulty face"),
    ];

    // Audio setup
    let menu_loop_file = BufReader::new(File::open("Resources/menu-loop.mp3").expect("Failed to open MP3 file"));
    let menu_loop = Decoder::new(menu_loop_file).expect("Failed to decode MP3 file").repeat_infinite();
    sink.append(menu_loop.clone());

    let mut level_music_file = BufReader::new(File::open("Resources/main-level-songs/0.mp3").expect("Failed to open MP3 file"));
    let mut _level_music = Decoder::new(level_music_file).expect("Failed to decode MP3 file");

    // Discord button setup
    let icon_size = 32.0;
    let discord_rect = Rectangle::new(
        20.0,
        rl.get_screen_height() as f32 - 32.0 * 2.0 - 5.0,
        icon_size,
        icon_size
    );

    let values_levels: Vec<&str> = save_data.split(";;;").collect();
    let save_pairs: Vec<&str> = values_levels[0].split(";").collect();
    let levels_completed: Vec<&str> = values_levels[1].split(";").collect();
    let online_levels_completed: Vec<&str> = values_levels[2].split(";").collect();
    let mut user = "0".to_string();
    let mut pass = "0".to_string();
    for pair in save_pairs {
        let key_value: Vec<&str> = pair.split(":").collect();

        if key_value[0] == "stars" {
            stars = key_value[1].parse::<u32>().unwrap();
        }

        if key_value[0] == "user" {
            if key_value[1] != "0" {
                user = key_value[1].to_string();
            }
        }

        if key_value[0] == "pass" {
            if key_value[1] != "0" {
                pass = key_value[1].to_string();
            }
        }
    }

    let level_index: u8 = 0;
    for level in levels_completed {
        let key_value: Vec<&str> = level.split(":").collect();
        if key_value[1] == "1" {
            levels_completed_vec[level_index as usize] = true
        }
    }

    for level in online_levels_completed {
        online_levels_beaten.push(level.parse().unwrap());
    }

    if user != "0" && pass != "0" {
        login_result = post_request(
            login_url.clone(),
            Some(hashmap! {
                "user".to_string() => user.clone(),
                "pass".to_string() => pass.clone()
            })
        ).await;

        if login_result == "Logged in!" {
            logged_in = true;
            if user == "Puppet" {
                is_mod = true
            }
        }
    }

    // Variables for text boxes
    let mut username: String = "".to_string();
    let mut password: String = "".to_string();

    let mut level_name: String = "".to_string();
    let mut level_desc: String = "".to_string();

    let mut level_id: String = "".to_string();

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

                if *latest_version.lock().unwrap() == "Loading..." && get_latest_version {
                    let latest_version_clone = std::sync::Arc::clone(&latest_version);
                    let latest_version_url = latest_version_url.to_owned();
                    
                    let _ = tokio::task::spawn(async move {
                        let version = get_request(latest_version_url, None).await;
                        let mut latest_version = latest_version_clone.lock().unwrap();
                        *latest_version = version;
                    });

                    get_latest_version = false
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
                    jump_force = default_jump_force;
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

                if account_page_button.is_clicked(&rl) {
                    game_state = GameState::AccountPage
                }

                account_page_button.update(&rl, delta_time);
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
                            if gravity > 0.0 {
                                if velocity_y > -10.0 {
                                    velocity_y -= ship_power
                                }
                            } else {
                                if velocity_y < 10.0 {
                                    velocity_y += ship_power
                                }
                            }
                        } else {
                            if gravity > 0.0 {
                                if velocity_y < 10.0 {
                                    velocity_y += ship_falling_speed
                                }
                            } else {
                                if velocity_y > -10.0 {
                                    velocity_y -= ship_falling_speed
                                }
                            }
                        }
                    } else {
                        velocity_y = 0.0
                    }
                }

                world_offset -= movement_speed;
                if current_gamemode == GameMode::Cube && velocity_y < 20.0 && velocity_y > -20.0 {
                    velocity_y += gravity;
                }
                player.y += velocity_y;

                if player.y >= 500.0 - player_cam_y as f32 {
                    player.y = 500.0;
                    velocity_y = 0.0;
                    is_on_ground = true;
                    rotation = 0.0;
                } else {
                    if gravity > 0.0 {
                        rotation += 5.0;
                    } else {
                        rotation -= 5.0;
                    }
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

                centered_player = Rectangle::new(player.x - player.width / 2.0, player.y - player.height / 2.0, player.width, player.height);

                small_player = centered_player;
                small_player.x = centered_player.x + 15.0;
                small_player.y = centered_player.y + 10.0;
                small_player.width = 20.0;
                small_player.height = 20.0;

                for object in &object_grid {
                    if object.x as f32 + world_offset < rl.get_screen_width() as f32 &&
                    object.x as f32 + world_offset > -40.0 && object.no_touch == 0 {
                        if object.id == 1 {
                            kill_player |= centered_player.check_collision_recs(&Rectangle {
                                x: object.x as f32 + world_offset + 20.0,
                                y: object.y as f32 + 20.0 - player_cam_y as f32,
                                width: 10.0,
                                height: 20.0
                            });
                        }

                        if object.id == 2 ||
                        object.id == 10 ||
                        object.id == 11 ||
                        object.id == 12 ||
                        object.id == 13 ||
                        object.id == 14 {
                            kill_player |= small_player.check_collision_recs(&Rectangle {
                                x: object.x as f32 + world_offset,
                                y: object.y as f32 + 10.0 - player_cam_y as f32,
                                width: 3.0,
                                height: 20.0
                            });

                            if centered_player.check_collision_recs(&Rectangle {
                                x: object.x as f32 + world_offset + 3.0,
                                y: object.y as f32 + 1.0 - player_cam_y as f32,
                                width: 37.0,
                                height: 3.0
                            }) {
                                is_on_ground = true;
                                rotation = 0.0;
                                if !mouse_down {
                                    player.y = object.y as f32 - 19.0 - player_cam_y as f32;
                                    velocity_y = 0.0;
                                } else {
                                    if gravity < 0.0 {
                                        touching_block_ceiling = true;
                                        player.y = object.y as f32 - 21.0 - player_cam_y as f32;
                                    }
                                }
                            } else {
                                touching_block_ceiling = false;
                            }

                            if centered_player.check_collision_recs(&Rectangle {
                                x: object.x as f32 + world_offset + 3.0,
                                y: object.y as f32 + 38.0 - player_cam_y as f32,
                                width: 37.0,
                                height: 3.0
                            }) {
                                is_on_ground = true;
                                rotation = 0.0;
                                if !mouse_down {
                                    player.y = object.y as f32 + 61.0 - player_cam_y as f32;
                                    velocity_y = 0.0;
                                } else {
                                    if gravity > 0.0 {
                                        touching_block_ceiling = true;
                                        player.y = object.y as f32 + 61.0 - player_cam_y as f32;
                                    }
                                }
                            } else {
                                touching_block_ceiling = false;
                            }

                            if centered_player.check_collision_recs(&Rectangle {
                                x: object.x as f32 + world_offset + 80.0,
                                y: object.y as f32 - player_cam_y as f32 + 10.0,
                                width: 3.0,
                                height: 20.0,
                            }) {
                                is_on_ground = false;
                            }
                        }

                        if object.id == 3
                        || object.id == 21 {
                            if centered_player.check_collision_recs(&Rectangle {
                                x: object.x as f32 + world_offset,
                                y: object.y as f32 + 35.0 - player_cam_y as f32,
                                width: 40.0,
                                height: 5.0
                            }) {
                                if object.id == 3 {
                                    if gravity > 0.0 {
                                        velocity_y = -15.0;
                                    } else {
                                        velocity_y = 15.0
                                    }
                                } else if object.id == 21 {
                                    if gravity > 0.0 {
                                        velocity_y = -7.0;
                                        gravity = -default_gravity
                                    } else {
                                        velocity_y = 7.0;
                                        gravity = default_gravity
                                    }
                                }
                                is_on_ground = false;
                            }
                        }

                        if object.id == 4
                        || object.id == 22 {
                            if centered_player.check_collision_recs(&Rectangle {
                                x: object.x as f32 - 10.0 + world_offset,
                                y: object.y as f32 - 10.0 - player_cam_y as f32,
                                width: 60.0,
                                height: 60.0
                            }) {
                                if on_orb && mouse_down {
                                    if object.id == 4 {
                                        if gravity > 0.0 {
                                            velocity_y = -13.0;
                                        } else {
                                            velocity_y = 13.0
                                        }
                                    } else if object.id == 22 {
                                        if gravity > 0.0 {
                                            velocity_y = -7.0;
                                            gravity = -default_gravity
                                        } else {
                                            velocity_y = 7.0;
                                            gravity = default_gravity
                                        }
                                    }
                                    on_orb = false
                                }

                                is_on_ground = false
                            }
                        }

                        if object.id == 5 || object.id == 6 {
                            if centered_player.check_collision_recs(&Rectangle {
                                x: object.x as f32 + world_offset + if object.rotation == 0 || object.rotation == 180 || object.rotation == -180 { 10.0 } else { -20.0 },
                                y: object.y as f32 - if object.rotation == 0 || object.rotation == 180 || object.rotation == -180 { 11.0 } else { -11.0 } - player_cam_y as f32,
                                width: if object.rotation == 0 || object.rotation == 180 || object.rotation == -180 { 20.0 } else { 80.0 },
                                height: if object.rotation == 0 || object.rotation == 180 || object.rotation == -180 { 80.0 } else { 20.0 }
                            }) {
                                if object.id == 5 {
                                    jump_force = -default_jump_force;
                                    gravity = -default_gravity;
                                } else {
                                    jump_force = default_jump_force;
                                    gravity = default_gravity;
                                }

                                is_on_ground = false
                            }
                        }

                        if object.id == 7 {
                            kill_player |= centered_player.check_collision_recs(&Rectangle {
                                x: object.x as f32 + world_offset + 20.0,
                                y: object.y as f32 + if object.rotation > 145 || object.rotation < -145 { 5.0 } else { 25.0 } - player_cam_y as f32,
                                width: 10.0,
                                height: 10.0
                            });
                        }

                        if object.id == 8 || object.id == 9 {
                            if centered_player.check_collision_recs(&Rectangle {
                                x: object.x as f32 + world_offset + if object.rotation == 0 || object.rotation == 180 || object.rotation == -180 { 10.0 } else { -20.0 },
                                y: object.y as f32 - if object.rotation == 0 || object.rotation == 180 || object.rotation == -180 { 11.0 } else { -11.0 } - player_cam_y as f32,
                                width: if object.rotation == 0 || object.rotation == 180 || object.rotation == -180 { 20.0 } else { 80.0 },
                                height: if object.rotation == 0 || object.rotation == 180 || object.rotation == -180 { 80.0 } else { 20.0 }
                            }) {
                                if object.id == 8 {
                                    current_gamemode = GameMode::Cube;
                                    cc_1003 = Color::LIME;
                                    is_on_ground = false
                                } else {
                                    current_gamemode = GameMode::Ship;
                                    cc_1003 = Color::MAGENTA;
                                    is_on_ground = false
                                }
                            }
                        }

                        if object.id == 15 {
                            if centered_player.check_collision_recs(&Rectangle {
                                x: object.x as f32 + world_offset,
                                y: object.y as f32 - player_cam_y as f32,
                                width: 40.0,
                                height: 40.0
                            }) {
                                if !in_custom_level && !levels_completed_vec[current_level] {
                                    stars += main_levels[current_level].difficulty as u32;
                                    levels_completed_vec[current_level] = true
                                } else if online_level_rated && in_custom_level {
                                    if !online_levels_beaten.contains(&level_id.parse().unwrap()) {
                                        stars += online_level_diff as u32;
                                        online_levels_beaten.push(level_id.parse().unwrap());
                                    }
                                }
                                game_state = GameState::LevelComplete;
                            }
                        }

                        if object.id == 17 ||
                        object.id == 18 ||
                        object.id == 19 ||
                        object.id == 20 {
                            if centered_player.check_collision_recs(&Rectangle {
                                x: object.x as f32 + world_offset + if object.rotation == 0 || object.rotation == 180 || object.rotation == -180 { 10.0 } else { -20.0 },
                                y: object.y as f32 - if object.rotation == 0 || object.rotation == 180 || object.rotation == -180 { 11.0 } else { -11.0 } - player_cam_y as f32,
                                width: if object.rotation == 0 || object.rotation == 180 || object.rotation == -180 { 20.0 } else { 80.0 },
                                height: if object.rotation == 0 || object.rotation == 180 || object.rotation == -180 { 80.0 } else { 20.0 }
                            }) {
                                movement_speed = if object.id == 17 {
                                    default_movement_speed
                                } else if object.id == 18 {
                                    default_movement_speed * 1.4
                                } else if object.id == 19 {
                                    default_movement_speed * 1.8
                                } else {
                                    default_movement_speed * 0.8
                                }
                            }
                        }

                        if object.id == 23 {
                            if centered_player.check_collision_recs(&Rectangle {
                                x: object.x as f32 + world_offset,
                                y: object.y as f32 - player_cam_y as f32,
                                width: 40.0,
                                height: 40.0
                            }) {
                                let color_trigger_red: u8 = object.properties.clone().unwrap()[0].clone().parse().unwrap();
                                let color_trigger_green: u8 = object.properties.clone().unwrap()[1].clone().parse().unwrap();
                                let color_trigger_blue: u8 = object.properties.clone().unwrap()[2].clone().parse().unwrap();
                                let color_trigger_fade: f32 = object.properties.clone().unwrap()[3].clone().parse().unwrap();

                                println!("{:?}", object.properties.clone().unwrap());

                                bg_red = color_trigger_red;
                                bg_green = color_trigger_green;
                                bg_blue = color_trigger_blue;

                                // let og_red = bg_red;
                                // let og_green = bg_green;
                                // let og_blue = bg_blue;

                                // let mut index = color_trigger_fade as i16;
                                // for _i in 0..color_trigger_fade as i32 {
                                //     bg_red = ((og_red as i16 - index + color_trigger_red as i16) / 2) as u8;
                                //     bg_green = ((og_green as i16 - index + color_trigger_green as i16) / 2) as u8;
                                //     bg_blue = ((og_blue as i16 - index + color_trigger_blue as i16) / 2) as u8;

                                //     index -= 1;
                                // }
                            }
                        }
                    }
                }

                if from_editor {
                    player_path.push(Vector2 { x: 200.0 - world_offset, y: player.y + player_cam_y as f32 });
                }

                if kill_player {
                    if from_editor {
                        sink.stop();
                        sink.append(menu_loop.clone());
                        sink.play();
                        game_state = GameState::Editor
                    } else {
                        game_state = GameState::GameOver
                    }
                }

                if rl.is_key_pressed(KeyboardKey::KEY_B) {
                    game_state = GameState::LevelSelect;
                }

                if rl.is_mouse_button_released(MouseButton::MOUSE_BUTTON_LEFT) ||
                rl.is_key_released(KeyboardKey::KEY_SPACE) {
                    on_orb = true
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
                keybinds_button.update(&rl, delta_time);
                download_level_button.update(&rl, delta_time);
                clear_level_button.update(&rl, delta_time);

                if menu_button.is_clicked(&rl) {
                    game_state = GameState::Menu;
                }

                if clear_level_button.is_clicked(&rl) {
                    level_string = default_level.to_string();
                }

                if create_button.is_clicked(&rl) {
                    parts = level_string.split(";;;").collect();
                    _level_metadata = parts[0];
                    _object_string = parts[1];
                    load_level(
                        &mut _level_metadata.to_string(),
                        &mut _object_string.to_string(),
                        &mut object_grid,
                        &mut bg_red,
                        &mut bg_green,
                        &mut bg_blue,
                        &mut ground_red,
                        &mut ground_green,
                        &mut ground_blue,
                        song_selected,
                        &mut current_song,
                        true,
                        true
                    );

                    from_editor = true;

                    game_state = GameState::Editor;
                }

                if featured_button.is_clicked(&rl) {
                    not_done_yet_text = true;
                }

                if search_button.is_clicked(&rl) {
                    game_state = GameState::SearchPage
                }

                if keybinds_button.is_clicked(&rl) {
                    game_state = GameState::EditorKeybinds
                }
            }
            GameState::Editor => {
                build_tab_button.update(&rl, delta_time);
                edit_tab_button.update(&rl, delta_time);
                delete_tab_button.update(&rl, delta_time);
                level_options_button.update(&rl, delta_time);
                editor_back.update(&rl, delta_time);
                level_save_button.update(&rl, delta_time);
                playtest_button.update(&rl, delta_time);
                level_upload_button.update(&rl, delta_time);
                obj1_button.update(&rl, delta_time);
                obj2_button.update(&rl, delta_time);
                obj3_button.update(&rl, delta_time);
                obj4_button.update(&rl, delta_time);
                obj5_button.update(&rl, delta_time);
                obj6_button.update(&rl, delta_time);
                obj7_button.update(&rl, delta_time);
                obj8_button.update(&rl, delta_time);
                obj9_button.update(&rl, delta_time);
                obj10_button.update(&rl, delta_time);
                obj11_button.update(&rl, delta_time);
                obj12_button.update(&rl, delta_time);
                obj13_button.update(&rl, delta_time);
                obj14_button.update(&rl, delta_time);
                obj15_button.update(&rl, delta_time);
                obj16_button.update(&rl, delta_time);
                obj17_button.update(&rl, delta_time);
                obj18_button.update(&rl, delta_time);
                obj19_button.update(&rl, delta_time);
                obj20_button.update(&rl, delta_time);
                obj21_button.update(&rl, delta_time);
                obj22_button.update(&rl, delta_time);
                obj23_button.update(&rl, delta_time);

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

                else if obj10_button.is_clicked(&rl) && active_tab == EditorTab::Build {
                    current_object = 10 + _advanced_page_number;
                }

                else if obj11_button.is_clicked(&rl) && active_tab == EditorTab::Build {
                    current_object = 11 + _advanced_page_number;
                }

                else if obj12_button.is_clicked(&rl) && active_tab == EditorTab::Build {
                    current_object = 12 + _advanced_page_number;
                }

                else if obj13_button.is_clicked(&rl) && active_tab == EditorTab::Build {
                    current_object = 13 + _advanced_page_number;
                }

                else if obj14_button.is_clicked(&rl) && active_tab == EditorTab::Build {
                    current_object = 14 + _advanced_page_number;
                }

                else if obj15_button.is_clicked(&rl) && active_tab == EditorTab::Build {
                    current_object = 15 + _advanced_page_number;
                }

                else if obj16_button.is_clicked(&rl) && active_tab == EditorTab::Build {
                    current_object = 16 + _advanced_page_number;
                }

                else if obj17_button.is_clicked(&rl) && active_tab == EditorTab::Build {
                    current_object = 17 + _advanced_page_number;
                }

                else if obj18_button.is_clicked(&rl) && active_tab == EditorTab::Build {
                    current_object = 18 + _advanced_page_number;
                }

                else if obj19_button.is_clicked(&rl) && active_tab == EditorTab::Build {
                    current_object = 19 + _advanced_page_number;
                }

                else if obj20_button.is_clicked(&rl) && active_tab == EditorTab::Build {
                    current_object = 20 + _advanced_page_number;
                }

                else if obj21_button.is_clicked(&rl) && active_tab == EditorTab::Build {
                    current_object = 21 + _advanced_page_number;
                }

                else if obj22_button.is_clicked(&rl) && active_tab == EditorTab::Build {
                    current_object = 22 + _advanced_page_number;
                }

                else if obj23_button.is_clicked(&rl) && active_tab == EditorTab::Build {
                    current_object = 23 + _advanced_page_number;
                }

                else if grid_button.is_clicked(&rl) {
                    // let obj_x = snapped_x;
                    // let obj_y = snapped_y;
                    if !level_options_button.is_clicked(&rl) && !editor_back.is_clicked(&rl) && !playtest_button.is_clicked(&rl) && !level_save_button.is_clicked(&rl) && !level_upload_button.is_clicked(&rl) {
                        if active_tab == EditorTab::Build {
                            object_grid.push(ObjectStruct {
                                y: if snapped_y < 0 { snapped_y - 40 } else { snapped_y },
                                x: if snapped_x < 0 { snapped_x - 40 } else { snapped_x },
                                no_touch: 0,
                                hide: 0,
                                id: current_object,
                                rotation: 0,
                                selected: false,
                                properties: if current_object == 23 { Some(
                                    vec![
                                        "50".to_string(),
                                        "50".to_string(),
                                        "50".to_string(),
                                        "0.50".to_string()
                                    ]
                                )} else {
                                    None
                                }
                            });
                        } else if active_tab == EditorTab::Delete {
                            let mut obj_index = 0;
                            while obj_index < object_grid.len() {
                                if object_grid[obj_index].x == if snapped_x < 0 { snapped_x - 40 } else { snapped_x } && object_grid[obj_index].y == if snapped_y < 0 { snapped_y - 40 } else { snapped_y } {
                                    object_grid.remove(obj_index);
                                    break;
                                } else {
                                    obj_index += 1;
                                }
                            }
                        } else if active_tab == EditorTab::Edit {
                            let mut obj_index = 0;
                            while obj_index < object_grid.len() {
                                if object_grid[obj_index].x == if snapped_x < 0 { snapped_x - 40 } else { snapped_x } && object_grid[obj_index].y == if snapped_y < 0 { snapped_y - 40 } else { snapped_y } && !object_grid[obj_index].selected {
                                    if rl.is_key_up(KeyboardKey::KEY_LEFT_SHIFT) {
                                        let mut objj_index = 0;
                                        while objj_index < object_grid.len() {
                                            object_grid[objj_index].selected = false;
                                            objj_index += 1
                                        }
                                    }

                                    object_grid[obj_index].selected = true;
                                    break;
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
                    if rl.is_key_down(KeyboardKey::KEY_LEFT_CONTROL) {
                        cam_pos_y += 5
                    } else {
                        cam_pos_y += 1
                    }
                }

                if down_arrow_down {
                    if rl.is_key_down(KeyboardKey::KEY_LEFT_CONTROL) {
                        cam_pos_y -= 5
                    } else {
                        cam_pos_y -= 1
                    }
                }

                if left_arrow_down {
                    if rl.is_key_down(KeyboardKey::KEY_LEFT_CONTROL) {
                        cam_pos_x -= 5
                    } else {
                        cam_pos_x -= 1
                    }
                }

                if right_arrow_down {
                    if rl.is_key_down(KeyboardKey::KEY_LEFT_CONTROL) {
                        cam_pos_x += 5
                    } else {
                        cam_pos_x += 1
                    }
                }

                if editor_back.is_clicked(&rl) {
                    let mut obj_index = 0;
                    while obj_index < object_grid.len() {
                        if object_grid[obj_index].selected {
                            object_grid[obj_index].selected = false
                        } else {
                            obj_index += 1;
                        }
                    }

                    game_state = GameState::CreatorMenu;
                }

                if level_save_button.is_clicked(&rl) {
                    level_string = get_level_text(
                        current_song,
                        bg_red,
                        bg_green,
                        bg_blue,
                        ground_red as u8,
                        ground_green as u8,
                        ground_blue as u8,
                        &object_grid
                    );

                    let write_result = fs::write("./save-data/levels/level.txt", &level_string);

                    println!("{:?}", write_result);
                }

                if playtest_button.is_clicked(&rl) {
                    player.y = 500.0;
                    world_offset = -(start_pos as f32 - 200.0);
                    rotation = 0.0;
                    gravity = default_gravity;
                    jump_force = default_jump_force;
                    current_gamemode = GameMode::Cube;
                    cc_1003 = Color::LIME;
                    in_custom_level = true;
                    velocity_y = 0.0;
                    player_cam_y = 0;
                    movement_speed = default_movement_speed;

                    let mut obj_index = 0;
                    while obj_index < object_grid.len() {
                        if object_grid[obj_index].selected {
                            object_grid[obj_index].selected = false
                        } else {
                            obj_index += 1;
                        }
                    }

                    level_music_file = BufReader::new(File::open(format!("{}", main_levels[current_song as usize].song)).expect("Failed to open MP3 file"));
                    _level_music = Decoder::new(level_music_file).expect("Failed to decode MP3 file");
                    sink.stop();
                    sink.append(_level_music);
                    sink.play();

                    player_path.clear();

                    game_state = GameState::Playing;
                }

                if rl.is_key_pressed(KeyboardKey::KEY_DELETE) {
                    let mut obj_index = 0;
                    while obj_index < object_grid.len() {
                        if object_grid[obj_index].selected {
                            object_grid.remove(obj_index);
                        } else {
                            obj_index += 1;
                        }
                    }
                }

                if rl.is_key_pressed(KeyboardKey::KEY_A) {
                    let mut obj_index = 0;
                    while obj_index < object_grid.len() {
                        if object_grid[obj_index].selected {
                            object_grid[obj_index].x -= 40;
                            obj_index += 1;
                        } else {
                            obj_index += 1;
                        }
                    }
                }

                if rl.is_key_pressed(KeyboardKey::KEY_D) {
                    let mut obj_index = 0;
                    while obj_index < object_grid.len() {
                        if object_grid[obj_index].selected {
                            object_grid[obj_index].x += 40;
                            obj_index += 1;
                        } else {
                            obj_index += 1;
                        }
                    }
                }

                if rl.is_key_pressed(KeyboardKey::KEY_W) {
                    let mut obj_index = 0;
                    while obj_index < object_grid.len() {
                        if object_grid[obj_index].selected {
                            object_grid[obj_index].y -= 40;
                            obj_index += 1;
                        } else {
                            obj_index += 1;
                        }
                    }
                }

                if rl.is_key_pressed(KeyboardKey::KEY_S) {
                    let mut obj_index = 0;
                    while obj_index < object_grid.len() {
                        if object_grid[obj_index].selected {
                            object_grid[obj_index].y += 40;
                            obj_index += 1;
                        } else {
                            obj_index += 1;
                        }
                    }
                }

                if rl.is_key_pressed(KeyboardKey::KEY_J) {
                    let mut obj_index = 0;
                    while obj_index < object_grid.len() {
                        if object_grid[obj_index].selected {
                            object_grid[obj_index].x -= 1;
                            obj_index += 1;
                        } else {
                            obj_index += 1;
                        }
                    }
                }

                if rl.is_key_pressed(KeyboardKey::KEY_L) {
                    let mut obj_index = 0;
                    while obj_index < object_grid.len() {
                        if object_grid[obj_index].selected {
                            object_grid[obj_index].x += 1;
                            obj_index += 1;
                        } else {
                            obj_index += 1;
                        }
                    }
                }

                if rl.is_key_pressed(KeyboardKey::KEY_I) {
                    let mut obj_index = 0;
                    while obj_index < object_grid.len() {
                        if object_grid[obj_index].selected {
                            object_grid[obj_index].y -= 1;
                            obj_index += 1;
                        } else {
                            obj_index += 1;
                        }
                    }
                }

                if rl.is_key_pressed(KeyboardKey::KEY_K) {
                    let mut obj_index = 0;
                    while obj_index < object_grid.len() {
                        if object_grid[obj_index].selected {
                            object_grid[obj_index].y += 1;
                            obj_index += 1;
                        } else {
                            obj_index += 1;
                        }
                    }
                }

                if rl.is_key_pressed(KeyboardKey::KEY_Q) {
                    let mut obj_index = 0;
                    while obj_index < object_grid.len() {
                        if object_grid[obj_index].selected {
                            if object_grid[obj_index].rotation != -270 {
                                object_grid[obj_index].rotation -= 90;
                            } else {
                                object_grid[obj_index].rotation = 0;
                            }

                            obj_index += 1;
                        } else {
                            obj_index += 1;
                        }
                    }
                }

                if rl.is_key_pressed(KeyboardKey::KEY_E) {
                    let mut obj_index = 0;
                    while obj_index < object_grid.len() {
                        if object_grid[obj_index].selected {
                            if object_grid[obj_index].rotation != 270 {
                                object_grid[obj_index].rotation += 90;
                            } else {
                                object_grid[obj_index].rotation = 0;
                            }

                            obj_index += 1;
                        } else {
                            obj_index += 1;
                        }
                    }
                }

                if level_upload_button.is_clicked(&rl) {
                    game_state = GameState::LevelUpload
                }

                if rl.is_key_down(KeyboardKey::KEY_PERIOD) {
                    if rl.is_key_down(KeyboardKey::KEY_LEFT_CONTROL) {
                        start_pos += 25
                    } else {
                        start_pos += 5;
                    }
                }

                if rl.is_key_down(KeyboardKey::KEY_COMMA) && start_pos > 0 {
                    if rl.is_key_down(KeyboardKey::KEY_LEFT_CONTROL) {
                        start_pos -= 25
                    } else {
                        start_pos -= 5;
                    }
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
                    load_level(
                        &mut _level_metadata.to_string(),
                        &mut _object_string.to_string(),
                        &mut object_grid,
                        &mut bg_red,
                        &mut bg_green,
                        &mut bg_blue,
                        &mut ground_red,
                        &mut ground_green,
                        &mut ground_blue,
                        song_selected,
                        &mut current_song,
                        false,
                        false
                    );

                    level_music_file = BufReader::new(File::open(format!("{}", main_levels[current_level].song)).expect("Failed to open MP3 file"));
                    _level_music = Decoder::new(level_music_file).expect("Failed to decode MP3 file");
                    sink.stop();
                    sink.append(_level_music);
                    sink.play();

                    player.y = 500.0;
                    world_offset = 0.0;
                    rotation = 0.0;
                    gravity = default_gravity;
                    jump_force = default_jump_force;
                    current_gamemode = GameMode::Cube;
                    cc_1003 = Color::LIME;
                    in_custom_level = false;
                    velocity_y = 0.0;
                    player_cam_y = 0;
                    movement_speed = default_movement_speed;

                    from_editor = false;
                    player_path.clear();

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

                if rl.is_key_pressed(KeyboardKey::KEY_S) {
                    current_song = current_level as u8;
                    song_selected = true;
                }
            }
            GameState::LevelComplete => {
                level_complete_back_button.update(&rl, delta_time);

                if level_complete_back_button.is_pressed {
                    game_state = GameState::Menu;
                }
            }
            GameState::EditorKeybinds => {
                menu_button.update(&rl, delta_time);

                if menu_button.is_clicked(&rl) {
                    game_state = GameState::CreatorMenu
                }

                if rl.get_mouse_wheel_move() < 0.0 {
                    editor_guide_scroll += 50
                } else if rl.get_mouse_wheel_move() > 0.0 &&
                editor_guide_scroll > 0 {
                    editor_guide_scroll -= 50
                }
            }
            GameState::AccountPage => {
                menu_button.update(&rl, delta_time);
                login_button.update(&rl, delta_time);
                register_button.update(&rl, delta_time);

                if menu_button.is_clicked(&rl) {
                    game_state = GameState::Menu
                }

                if login_button.is_clicked(&rl) {
                    let login_url = login_url.to_owned();

                    let login_result_string = post_request(
                        login_url,
                        Some(hashmap! {
                            "user".to_string() => username.clone(),
                            "pass".to_string() => password.clone()
                        })
                    ).await;
                    login_result = login_result_string;

                    if login_result == "Logged in!" {
                        logged_in = true;
                        user = username.clone();
                        pass = password.clone();
                    }

                    register_result = "".to_string();
                }

                if register_button.is_clicked(&rl) {
                    let register_url = register_url.to_owned();

                    let register_result_string = post_request(
                        register_url,
                        Some(hashmap! {
                            "user".to_string() => username.clone(),
                            "pass".to_string() => password.clone()
                        })
                    ).await;
                    register_result = register_result_string;
                    login_result = "".to_string();
                };

                if username_textbox.is_clicked(&rl) {
                    username_textbox.active = true
                }

                if password_textbox.is_clicked(&rl) {
                    password_textbox.active = true
                }

                if username_textbox.is_not_clicked(&rl) {
                    username_textbox.active = false
                }

                if password_textbox.is_not_clicked(&rl) {
                    password_textbox.active = false
                }

                username_textbox.input(&mut username, &rl);
                password_textbox.input(&mut password, &rl);
            }
            GameState::LevelUpload => {
                menu_button.update(&rl, delta_time);
                upload_button.update(&rl, delta_time);

                if menu_button.is_clicked(&rl) {
                    game_state = GameState::CreatorMenu
                }

                if upload_button.is_clicked(&rl) {
                    if logged_in {
                        let level_data = get_level_text(
                            current_song,
                            bg_red,
                            bg_green,
                            bg_blue,
                            ground_red as u8,
                            ground_green as u8,
                            ground_blue as u8,
                            &object_grid
                        );
    
                        level_upload_result = post_request(
                            upload_url.clone(),
                            Some(hashmap! {
                                "name".to_string() => level_name.clone(),
                                "desc".to_string() => level_desc.clone(),
                                "data".to_string() => level_data,
                                "creator".to_string() => user.clone(),
                                "pass".to_string() => pass.clone(),
                                "diff".to_string() => online_level_upload_diff.to_string()
                            })
                        ).await;
                        
                        println!("{}", level_upload_result);
                    } else {
                        level_upload_result = "Not logged in!".to_string();
                    }
                }

                if level_name_textbox.is_clicked(&rl) {
                    level_name_textbox.active = true
                }

                if level_name_textbox.is_not_clicked(&rl) {
                    level_name_textbox.active = false
                }

                if level_desc_textbox.is_clicked(&rl) {
                    level_desc_textbox.active = true
                }

                if level_desc_textbox.is_not_clicked(&rl) {
                    level_desc_textbox.active = false
                }

                level_name_textbox.input(&mut level_name, &rl);
                level_desc_textbox.input(&mut level_desc, &rl);

                if rl.is_key_pressed(KeyboardKey::KEY_LEFT) && online_level_upload_diff > 0 {
                    online_level_upload_diff -= 1;
                }

                if rl.is_key_pressed(KeyboardKey::KEY_RIGHT) && online_level_upload_diff < 5 {
                    online_level_upload_diff += 1;
                }
            }
            GameState::LevelPage => {
                level_play_button.update(&rl, delta_time);
                menu_button.update(&rl, delta_time);
                level_rate_button.update(&rl, delta_time);

                if level_play_button.is_clicked(&rl) {
                    let parts: Vec<&str> = online_level_data.split(";;;").collect();
                    let level_loaded = load_level(
                        &mut parts[0].to_string(),
                        &mut parts[1].to_string(),
                        &mut object_grid,
                        &mut bg_red,
                        &mut bg_green,
                        &mut bg_blue,
                        &mut ground_red,
                        &mut ground_green,
                        &mut ground_blue,
                        song_selected,
                        &mut current_song,
                        true,
                        false
                    );

                    if level_loaded == "ok" {
                        level_music_file = BufReader::new(File::open(format!("{}", main_levels[current_song as usize].song)).expect("Failed to open MP3 file"));
                        _level_music = Decoder::new(level_music_file).expect("Failed to decode MP3 file");
                        sink.stop();
                        sink.append(_level_music);
                        sink.play();
                    }

                    player.y = 500.0;
                    world_offset = 0.0;
                    rotation = 0.0;
                    gravity = default_gravity;
                    jump_force = default_jump_force;
                    current_gamemode = GameMode::Cube;
                    cc_1003 = Color::LIME;
                    in_custom_level = true;
                    velocity_y = 0.0;
                    player_cam_y = 0;
                    movement_speed = default_movement_speed;

                    if level_loaded == "ok" {
                        from_editor = false;
                        player_path.clear();

                        game_state = GameState::Playing;
                    }
                }

                if menu_button.is_clicked(&rl) {
                    game_state = GameState::SearchPage
                }

                if is_mod && level_rate_button.is_clicked(&rl) {
                    game_state = GameState::LevelRate
                }
            }
            GameState::SearchPage => {
                menu_button.update(&rl, delta_time);
                download_level_button.update(&rl, delta_time);

                if menu_button.is_clicked(&rl) {
                    show_level_not_found = false;
                    game_state = GameState::CreatorMenu
                }

                if level_id_textbox.is_clicked(&rl) {
                    level_id_textbox.active = true
                }

                if level_id_textbox.is_not_clicked(&rl) {
                    level_id_textbox.active = false
                }

                level_id_textbox.input(&mut level_id, &rl);

                if download_level_button.is_clicked(&rl) && level_id.len() > 0 {
                    level_download_result = get_request(
                        download_url.clone(),
                        Some(hashmap! {
                            "id".to_string() => level_id.clone()
                        })
                    ).await;

                    if level_download_result != "Level doesn't exist!" {
                        parse_level_download_response(
                            level_download_result.clone(),
                            &mut online_level_name,
                            &mut online_level_desc,
                            &mut online_level_diff,
                            &mut online_level_rated,
                            &mut online_level_creator,
                            &mut online_level_data
                        );

                        show_level_not_found = false;
                        game_state = GameState::LevelPage
                    } else {
                        show_level_not_found = true
                    }
                }
            }
            GameState::LevelRate => {
                menu_button.update(&rl, delta_time);
                submit_rating_button.update(&rl, delta_time);

                if menu_button.is_clicked(&rl) {
                    game_state = GameState::LevelPage
                }

                if rl.is_key_pressed(KeyboardKey::KEY_LEFT) && online_level_rate_diff > 0 {
                    online_level_rate_diff -= 1;
                }

                if rl.is_key_pressed(KeyboardKey::KEY_RIGHT) && online_level_rate_diff < 10 {
                    online_level_rate_diff += 1;
                }

                if submit_rating_button.is_clicked(&rl) {
                    level_rate_result = post_request(
                        rate_url.clone(),
                        Some(hashmap! {
                            "user".to_string() => user.clone(),
                            "pass".to_string() => pass.clone(),
                            "diff".to_string() => format!("{}", online_level_rate_diff),
                            "id".to_string() => level_id.clone()
                        })
                    ).await;

                    if level_rate_result == "Rating applied!" {
                        online_level_diff = online_level_rate_diff
                    }
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

                play_button.draw(false, None, 1.0, false, &mut d);
                editor_button.draw(false, None, 1.0, false, &mut d);

                d.draw_text(&format!("Version: {}", version), 10, 10, 15, Color::WHITE);
                d.draw_text(&format!("Latest Version: {}", *latest_version.lock().unwrap()), 10, 30, 15, Color::WHITE);
                d.draw_text(&format!("Stars: {}", stars), 10, 50, 15, Color::WHITE);

                d.draw_text(&format!("Made by Thepuppet57"), 10, d.get_screen_height() - 30, 25, Color::WHITE);

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

                account_page_button.draw(false, None, 1.0, false, &mut d);
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
                    let object_x = i.x as f32 + world_offset as f32 + 20.0;
                    let object_y = i.y as f32 - player_cam_y as f32 + 20.0;
                    if i.x as f32 + world_offset < d.get_screen_width() as f32 &&
                    i.x as f32 + world_offset > -40.0 && i.hide == 0 {
                        if from_editor || i.id != 15 {
                            if i.id != 17 && i.id != 18 && i.id != 19 && i.id != 20 {
                                d.draw_texture_pro(
                                    &texture_ids.get(&i.id).unwrap(),
                                    Rectangle::new(
                                        0.0,
                                        0.0,
                                        texture_ids.get(&i.id).unwrap().width as f32,
                                        texture_ids.get(&i.id).unwrap().height as f32
                                    ),
                                    Rectangle::new(
                                        object_x,
                                        object_y,
                                        texture_ids.get(&i.id).unwrap().width as f32 * 0.05,
                                        texture_ids.get(&i.id).unwrap().height as f32 * 0.05
                                    ),
                                    Vector2::new(
                                        texture_ids.get(&i.id).unwrap().width as f32 / 2.0 * 0.05,
                                        texture_ids.get(&i.id).unwrap().height as f32 / 2.0 * 0.05
                                    ),
                                    i.rotation as f32,
                                    cc_1004
                                );
                            } else {
                                d.draw_texture_pro(
                                    &texture_ids.get(&i.id).unwrap(),
                                    Rectangle::new(
                                        0.0,
                                        0.0,
                                        texture_ids.get(&i.id).unwrap().width as f32,
                                        texture_ids.get(&i.id).unwrap().height as f32
                                    ),
                                    Rectangle::new(
                                        object_x + 10.0,
                                        object_y,
                                        texture_ids.get(&i.id).unwrap().width as f32 * 0.1,
                                        texture_ids.get(&i.id).unwrap().height as f32 * 0.1
                                    ),
                                    Vector2::new(
                                        texture_ids.get(&i.id).unwrap().width as f32 / 2.0 * 0.1,
                                        texture_ids.get(&i.id).unwrap().height as f32 / 2.0 * 0.1
                                    ),
                                    i.rotation as f32,
                                    cc_1004
                                );
                            }
                        }
                    }
                }

                // Draw obstacles (old)
                // for obstacle in &obstacles {
                //     let actual_x = obstacle.x + world_offset;
                //     d.draw_texture_ex(&texture_ids.get(&1).unwrap(), Vector2::new(actual_x, 480.0), 0.0, 0.05, cc_1004);
                // }

                if show_debug_text {
                    for object in &object_grid {
                        if object.x as f32 + world_offset < d.get_screen_width() as f32 &&
                        object.x as f32 + world_offset > -40.0 {
                            if object.id == 1 {
                                d.draw_rectangle_lines(
                                    object.x + world_offset as i32 + 15,
                                    object.y + 10 - player_cam_y,
                                    10,
                                    20,
                                    Color::RED
                                );
                            }
    
                            if object.id == 2 ||
                            object.id == 10 ||
                            object.id == 11 ||
                            object.id == 12 ||
                            object.id == 13 ||
                            object.id == 14 {
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
                                    object.y + 38 - player_cam_y,
                                    37,
                                    3,
                                    Color::BLUEVIOLET
                                );
    
                                d.draw_rectangle_lines(
                                    object.x + world_offset as i32 + 80,
                                    object.y - player_cam_y + 10,
                                    3,
                                    20,
                                    Color::GREEN
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
    
                            if object.id == 4
                            || object.id == 22 {
                                d.draw_rectangle_lines(
                                    object.x - 10 + world_offset as i32,
                                    object.y - 10 - player_cam_y,
                                    60,
                                    60,
                                    Color::TEAL
                                );
                            }
    
                            if object.id == 5 || object.id == 6 {
                                d.draw_rectangle_lines(
                                    object.x + world_offset as i32 + if object.rotation == 0 || object.rotation == 180 || object.rotation == -180 { 10 } else { -20 },
                                    object.y - if object.rotation == 0 || object.rotation == 180 || object.rotation == -180 { 11 } else { -11 } - player_cam_y,
                                    if object.rotation == 0 || object.rotation == 180 || object.rotation == -180 { 20 } else { 80 },
                                    if object.rotation == 0 || object.rotation == 180 || object.rotation == -180 { 80 } else { 20 },
                                    Color::TEAL
                                );
                            }
    
                            if object.id == 7 {
                                d.draw_rectangle_lines(
                                    object.x + world_offset as i32 + 15,
                                    object.y + if object.rotation > 145 || object.rotation < -145 { 5 } else { 25 } - player_cam_y,
                                    10,
                                    10,
                                    Color::RED
                                );
                            }
    
                            if object.id == 8 || object.id == 9 {
                                d.draw_rectangle_lines(
                                    object.x + world_offset as i32 + if object.rotation == 0 || object.rotation == 180 || object.rotation == -180 { 10 } else { -20 },
                                    object.y - if object.rotation == 0 || object.rotation == 180 || object.rotation == -180 { 11 } else { -11 } - player_cam_y,
                                    if object.rotation == 0 || object.rotation == 180 || object.rotation == -180 { 20 } else { 80 },
                                    if object.rotation == 0 || object.rotation == 180 || object.rotation == -180 { 80 } else { 20 },
                                    Color::TEAL
                                );
                            }
    
                            if object.id == 15 {
                                d.draw_rectangle_lines(
                                    object.x + world_offset as i32,
                                    object.y - player_cam_y,
                                    40,
                                    40,
                                    Color::TEAL
                                );
                            }

                            if object.id == 17 ||
                            object.id == 18 ||
                            object.id == 19 ||
                            object.id == 20 {
                                d.draw_rectangle_lines(
                                    object.x + world_offset as i32 + if object.rotation == 0 || object.rotation == 180 || object.rotation == -180 { 10 } else { -20 },
                                    object.y - if object.rotation == 0 || object.rotation == 180 || object.rotation == -180 { 11 } else { -11 } - player_cam_y,
                                    if object.rotation == 0 || object.rotation == 180 || object.rotation == -180 { 20 } else { 80 },
                                    if object.rotation == 0 || object.rotation == 180 || object.rotation == -180 { 80 } else { 20 },
                                    Color::TEAL
                                );
                            }
                        }
                    }

                    d.draw_rectangle_lines(
                        small_player.x as i32,
                        small_player.y as i32,
                        small_player.width as i32,
                        small_player.height as i32,
                        Color::BLACK
                    );

                    d.draw_rectangle_lines(
                        centered_player.x as i32,
                        centered_player.y as i32,
                        centered_player.width as i32,
                        centered_player.height as i32,
                        Color::WHITE
                    );
                }

                if from_editor {
                    for point in &player_path {
                        if point.x as f32 + world_offset < d.get_screen_width() as f32 &&
                        point.x as f32 + world_offset > 60.0 {
                            d.draw_circle(
                                point.x as i32 + world_offset as i32,
                                point.y as i32 - player_cam_y,
                                5.0,
                                Color::GREEN
                            );
                        }
                    }
                }

                d.draw_text(&format!("Attempt: {}", attempt), 10, 10, 20, Color::WHITE);

                if show_debug_text {
                    d.draw_text(&format!("Velocity Y: {}", velocity_y), 10, 40, 20, Color::LIME);
                    d.draw_text(&format!("On Ground: {}", is_on_ground), 10, 70, 20, Color::LIME);
                    d.draw_text(&format!("Touching block ceiling: {}", touching_block_ceiling), 10, 100, 20, Color::LIME);
                    d.draw_text(&format!("Points in path: {}", player_path.len()), 10, 130, 20, Color::LIME);
                }
            }
            GameState::GameOver => {
                d.clear_background(Color::WHITE);
                d.draw_texture_ex(&menu_bg, Vector2::new(0.0, -100.0), 0.0, 0.8, Color::DARKRED);

                d.draw_text("Game Over!", 250, 150, 50, Color::WHITE);
                d.draw_text(&format!("Attempts: {}", attempt), 330, 250, 20, Color::WHITE);
                
                restart_button.draw(false, None, 1.0, false, &mut d);
            }
            GameState::CreatorMenu => {
                d.clear_background(Color::WHITE);
                d.draw_texture_ex(&menu_bg, Vector2::new(-200.0, -250.0), 0.0, 0.8, Color { r:50, g:50, b:50, a:255 });
                
                // d.draw_text("Editor will be added eventually!", 50, 250, 45, Color::WHITE);
                menu_button.draw(false, None, 1.0, false, &mut d);
                create_button.draw(false, None, 1.0, false, &mut d);
                featured_button.draw(false, None, 1.0, false, &mut d);
                search_button.draw(false, None, 1.0, false, &mut d);
                keybinds_button.draw(false, None, 1.0, false, &mut d);
                clear_level_button.draw(false, None, 1.0, false, &mut d);

                if not_done_yet_text {
                    d.draw_text("This will be added eventually!", 250, 30, 30, Color::WHITE);
                }
            }
            GameState::Editor => {
                d.clear_background(Color::WHITE);
                d.draw_texture_ex(&game_bg, Vector2::new(0.0, -150.0), 0.0, 0.7, cc_1001);

                for i in &object_grid {
                    let object_x = i.x as f32 - cam_pos_x as f32 * 5.0 + 20.0;
                    let object_y = i.y as f32 + cam_pos_y as f32 * 5.0 + 20.0;
                    if i.id != 17 && i.id != 18 && i.id != 19 && i.id != 20 {
                        d.draw_texture_pro(
                            &texture_ids.get(&i.id).unwrap(),
                            Rectangle::new(
                                0.0,
                                0.0,
                                texture_ids.get(&i.id).unwrap().width as f32,
                                texture_ids.get(&i.id).unwrap().height as f32
                            ),
                            Rectangle::new(
                                object_x,
                                object_y,
                                texture_ids.get(&i.id).unwrap().width as f32 * 0.05,
                                texture_ids.get(&i.id).unwrap().height as f32 * 0.05
                            ),
                            Vector2::new(
                                texture_ids.get(&i.id).unwrap().width as f32 / 2.0 * 0.05,
                                texture_ids.get(&i.id).unwrap().height as f32 / 2.0 * 0.05
                            ),
                            i.rotation as f32,
                            if i.selected { Color::LIME } else { cc_1004 }
                        );
                    } else {
                        d.draw_texture_pro(
                            &texture_ids.get(&i.id).unwrap(),
                            Rectangle::new(
                                0.0,
                                0.0,
                                texture_ids.get(&i.id).unwrap().width as f32,
                                texture_ids.get(&i.id).unwrap().height as f32
                            ),
                            Rectangle::new(
                                object_x + 10.0,
                                object_y,
                                texture_ids.get(&i.id).unwrap().width as f32 * 0.1,
                                texture_ids.get(&i.id).unwrap().height as f32 * 0.1
                            ),
                            Vector2::new(
                                texture_ids.get(&i.id).unwrap().width as f32 / 2.0 * 0.1,
                                texture_ids.get(&i.id).unwrap().height as f32 / 2.0 * 0.1
                            ),
                            i.rotation as f32,
                            if i.selected { Color::LIME } else { cc_1004 }
                        );
                    }
                }

                d.draw_line(
                    start_pos as i32 - cam_pos_x * 5,
                    0,
                    start_pos as i32 - cam_pos_x * 5,
                    d.get_screen_height(),
                    Color::WHITE
                );

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

                for point in &player_path {
                    d.draw_circle(
                        point.x as i32 - cam_pos_x * 5,
                        point.y as i32 + cam_pos_y * 5,
                        5.0,
                        Color::GREEN
                    );
                }

                d.draw_rectangle(0, 400, 800, 200, Color { r:30, g:30, b:30, a:100 });

                d.draw_line(175, 400, 175, 600, Color::WHITE);

                build_tab_button.draw(false, None, 1.0, false, &mut d);
                edit_tab_button.draw(false, None, 1.0, false, &mut d);
                delete_tab_button.draw(false, None, 1.0, false, &mut d);
                level_options_button.draw(false, None, 1.0, false, &mut d);
                editor_back.draw(false, None, 1.0, false, &mut d);
                level_save_button.draw(false, None, 1.0, false, &mut d);
                playtest_button.draw(false, None, 1.0, false, &mut d);
                level_upload_button.draw(false, None, 1.0, false, &mut d);

                if edit_not_done_yet {
                    d.draw_text("Click to select!", 270, 490, 40, Color::WHITE);
                }

                // Draw all the object buttons
                if active_tab == EditorTab::Build {
                    let object_button_texture_scale: f32 = 0.04;

                    obj1_button.draw(true, Some(texture_ids.get(&1).unwrap()), object_button_texture_scale, true, &mut d);
                    obj2_button.draw(true, Some(texture_ids.get(&2).unwrap()), object_button_texture_scale, true, &mut d);
                    obj3_button.draw(true, Some(texture_ids.get(&3).unwrap()), object_button_texture_scale, true, &mut d);
                    obj4_button.draw(true, Some(texture_ids.get(&4).unwrap()), object_button_texture_scale, true, &mut d);
                    obj5_button.draw(true, Some(texture_ids.get(&5).unwrap()), object_button_texture_scale, true, &mut d);
                    obj6_button.draw(true, Some(texture_ids.get(&6).unwrap()), object_button_texture_scale, true, &mut d);
                    obj7_button.draw(true, Some(texture_ids.get(&7).unwrap()), object_button_texture_scale, true, &mut d);
                    obj8_button.draw(true, Some(texture_ids.get(&8).unwrap()), object_button_texture_scale, true, &mut d);
                    obj9_button.draw(true, Some(texture_ids.get(&9).unwrap()), object_button_texture_scale, true, &mut d);
                    obj10_button.draw(true, Some(texture_ids.get(&10).unwrap()), object_button_texture_scale, true, &mut d);
                    obj11_button.draw(true, Some(texture_ids.get(&11).unwrap()), object_button_texture_scale, true, &mut d);
                    obj12_button.draw(true, Some(texture_ids.get(&12).unwrap()), object_button_texture_scale, true, &mut d);
                    obj13_button.draw(true, Some(texture_ids.get(&13).unwrap()), object_button_texture_scale, true, &mut d);
                    obj14_button.draw(true, Some(texture_ids.get(&14).unwrap()), object_button_texture_scale, true, &mut d);
                    obj15_button.draw(true, Some(texture_ids.get(&15).unwrap()), object_button_texture_scale, true, &mut d);
                    obj16_button.draw(true, Some(texture_ids.get(&16).unwrap()), object_button_texture_scale, true, &mut d);
                    obj17_button.draw(true, Some(texture_ids.get(&17).unwrap()), object_button_texture_scale, true, &mut d);
                    obj18_button.draw(true, Some(texture_ids.get(&18).unwrap()), object_button_texture_scale, true, &mut d);
                    obj19_button.draw(true, Some(texture_ids.get(&19).unwrap()), object_button_texture_scale, true, &mut d);
                    obj20_button.draw(true, Some(texture_ids.get(&20).unwrap()), object_button_texture_scale, true, &mut d);
                    obj21_button.draw(true, Some(texture_ids.get(&21).unwrap()), object_button_texture_scale, true, &mut d);
                    obj22_button.draw(true, Some(texture_ids.get(&22).unwrap()), object_button_texture_scale, true, &mut d);
                    obj23_button.draw(true, Some(texture_ids.get(&23).unwrap()), object_button_texture_scale, true, &mut d);
                }

                d.draw_text(&format!("Selected Object: {}", objects.get(&current_object).unwrap()), 10, 10, 20, Color::WHITE);
                if show_debug_text {
                    d.draw_text(&format!("Camera pos X: {}", cam_pos_x), 10, 40, 20, Color::LIME);
                    d.draw_text(&format!("Camera pos Y: {}", cam_pos_y), 10, 70, 20, Color::LIME);
                    d.draw_text(&format!("Advanced Page Number: {}", _advanced_page_number), 10, 100, 20, Color::LIME);
                    d.draw_text(&format!("Mouse X On Grid: {}", snapped_x), 10, 130, 20, Color::LIME);
                    d.draw_text(&format!("Mouse Y On Grid: {}", snapped_y), 10, 160, 20, Color::LIME);
                    d.draw_text(&format!("Mouse X: {}", mouse_x), 10, 190, 20, Color::LIME);
                    d.draw_text(&format!("Mouse Y: {}", mouse_y), 10, 220, 20, Color::LIME);

                    d.draw_text(&format!("Object Grid: {:?}", object_grid), 10, 250, 20, Color::LIME);
                }
            }
            GameState::LevelOptions => {
                d.clear_background(Color {r:0, g:0, b:75, a:255});

                level_options_back.draw(false, None, 1.0, false, &mut d);

                d.draw_rectangle(425, 20, 100, 50, Color {r:255, g:0, b:0, a:255});
                d.draw_rectangle(550, 20, 100, 50, Color {r:0, g:255, b:0, a:255});
                d.draw_rectangle(675, 20, 100, 50, Color {r:0, g:0, b:255, a:255});

                d.draw_rectangle_rounded_lines_ex(Rectangle { x:425.0, y:20.0, width:100.0, height:50.0 }, 0.0, 4, 5.0, Color::BLACK);
                d.draw_rectangle_rounded_lines_ex(Rectangle { x:550.0, y:20.0, width:100.0, height:50.0 }, 0.0, 4, 5.0, Color::BLACK);
                d.draw_rectangle_rounded_lines_ex(Rectangle { x:675.0, y:20.0, width:100.0, height:50.0 }, 0.0, 4, 5.0, Color::BLACK);

                d.draw_rectangle(470, 100, 10, 150, Color {r:255, g:0, b:0, a:255});
                d.draw_rectangle(595, 100, 10, 150, Color {r:0, g:255, b:0, a:255});
                d.draw_rectangle(720, 100, 10, 150, Color {r:0, g:0, b:255, a:255});

                d.draw_rectangle_rounded_lines_ex(Rectangle {x: 470.0, y: 100.0, width:10.0, height:150.0}, 0.0, 4, 5.0, Color::BLACK);
                d.draw_rectangle_rounded_lines_ex(Rectangle {x: 595.0, y: 100.0, width:10.0, height:150.0}, 0.0, 4, 5.0, Color::BLACK);
                d.draw_rectangle_rounded_lines_ex(Rectangle {x: 720.0, y: 100.0, width:10.0, height:150.0}, 0.0, 4, 5.0, Color::BLACK);

                d.draw_rectangle(450, red_bg_slider_pos as i32, 50, 50, Color::WHITE);
                d.draw_rectangle(575, green_bg_slider_pos as i32, 50, 50, Color::WHITE);
                d.draw_rectangle(700, blue_bg_slider_pos as i32, 50, 50, Color::WHITE);

                d.draw_rectangle_rounded_lines_ex(Rectangle {x: 450.0, y: red_bg_slider_pos as f32, width:50.0, height:50.0}, 0.0, 4, 5.0, Color::BLACK);
                d.draw_rectangle_rounded_lines_ex(Rectangle {x: 575.0, y: green_bg_slider_pos as f32, width:50.0, height:50.0}, 0.0, 4, 5.0, Color::BLACK);
                d.draw_rectangle_rounded_lines_ex(Rectangle {x: 700.0, y: blue_bg_slider_pos as f32, width:50.0, height:50.0}, 0.0, 4, 5.0, Color::BLACK);

                d.draw_text(&format!("{}", bg_red), 435, 25, 50, Color::BLACK);
                d.draw_text(&format!("{}", bg_green), 560, 25, 50, Color::BLACK);
                d.draw_text(&format!("{}", bg_blue), 685, 25, 50, Color::BLACK);

                d.draw_rectangle(425, 300, 100, 50, Color {r:255, g:0, b:0, a:255});
                d.draw_rectangle(550, 300, 100, 50, Color {r:0, g:255, b:0, a:255});
                d.draw_rectangle(675, 300, 100, 50, Color {r:0, g:0, b:255, a:255});

                d.draw_rectangle_rounded_lines_ex(Rectangle { x:425.0, y:300.0, width:100.0, height:50.0 }, 0.0, 4, 5.0, Color::BLACK);
                d.draw_rectangle_rounded_lines_ex(Rectangle { x:550.0, y:300.0, width:100.0, height:50.0 }, 0.0, 4, 5.0, Color::BLACK);
                d.draw_rectangle_rounded_lines_ex(Rectangle { x:675.0, y:300.0, width:100.0, height:50.0 }, 0.0, 4, 5.0, Color::BLACK);

                d.draw_rectangle(470, 380, 10, 150, Color {r:255, g:0, b:0, a:255});
                d.draw_rectangle(595, 380, 10, 150, Color {r:0, g:255, b:0, a:255});
                d.draw_rectangle(720, 380, 10, 150, Color {r:0, g:0, b:255, a:255});

                d.draw_rectangle_rounded_lines_ex(Rectangle {x: 470.0, y: 380.0, width:10.0, height:150.0}, 0.0, 4, 5.0, Color::BLACK);
                d.draw_rectangle_rounded_lines_ex(Rectangle {x: 595.0, y: 380.0, width:10.0, height:150.0}, 0.0, 4, 5.0, Color::BLACK);
                d.draw_rectangle_rounded_lines_ex(Rectangle {x: 720.0, y: 380.0, width:10.0, height:150.0}, 0.0, 4, 5.0, Color::BLACK);

                d.draw_rectangle(450, red_ground_slider_pos as i32, 50, 50, Color::WHITE);
                d.draw_rectangle(575, green_ground_slider_pos as i32, 50, 50, Color::WHITE);
                d.draw_rectangle(700, blue_ground_slider_pos as i32, 50, 50, Color::WHITE);

                d.draw_rectangle_rounded_lines_ex(Rectangle {x: 450.0, y: red_ground_slider_pos as f32, width:50.0, height:50.0}, 0.0, 4, 5.0, Color::BLACK);
                d.draw_rectangle_rounded_lines_ex(Rectangle {x: 575.0, y: green_ground_slider_pos as f32, width:50.0, height:50.0}, 0.0, 4, 5.0, Color::BLACK);
                d.draw_rectangle_rounded_lines_ex(Rectangle {x: 700.0, y: blue_ground_slider_pos as f32, width:50.0, height:50.0}, 0.0, 4, 5.0, Color::BLACK);

                d.draw_text(&format!("{}", ground_red), 435, 305, 50, Color::BLACK);
                d.draw_text(&format!("{}", ground_green), 560, 305, 50, Color::BLACK);
                d.draw_text(&format!("{}", ground_blue), 685, 305, 50, Color::BLACK);

                d.draw_rectangle(300, 20, 75, 50, cc_1001);
                d.draw_rectangle(300, 300, 75, 50, cc_1002);
            }
            GameState::LevelSelect => {
                d.clear_background(Color::BLACK);
                d.draw_text(&format!("{}", main_levels[current_level].name), d.get_screen_width() / 2 - d.measure_text(&main_levels[current_level].name, 50) / 2, 275, 50, Color::WHITE);
                d.draw_text(&format!("{}", main_levels[current_level].difficulty), 400, 430, 50, Color::WHITE);
                d.draw_text(&format!("{}", main_levels[current_level].artist), d.get_screen_width() / 2 - d.measure_text(&main_levels[current_level].artist, 50) / 2, 500, 50, Color::WHITE);
                d.draw_text(&format!("Level {}", current_level + 1), d.get_screen_width() / 2 - d.measure_text(&format!("Level {}", current_level + 1), 50) / 2, 20, 50, Color::WHITE);
                d.draw_rectangle_rounded_lines_ex(
                    Rectangle {
                        x: 150.0,
                        y: 150.0,
                        width: 500.0,
                        height: 200.0
                    },
                    0.0,
                    4,
                    10.0,
                    Color::WHITE
                );
                d.draw_texture_ex(
                    &difficulties[main_levels[current_level].difficulty as usize],
                    Vector2::new(280.0, 120.0),
                    0.0,
                    0.1,
                    Color::WHITE
                );

                d.draw_texture_ex(
                    &star_texture,
                    Vector2::new(260.0, 350.0),
                    0.0,
                    0.13,
                    Color::WHITE
                );

                d.draw_text(
                    "Press Space to Play!",
                    20,
                    20,
                    20,
                    Color::WHITE
                );
            }
            GameState::LevelComplete => {
                d.clear_background(Color::WHITE);
                d.draw_texture_ex(&menu_bg, Vector2::new(0.0, -100.0), 0.0, 0.8, Color::DARKGREEN);

                d.draw_text(
                    "Level Complete!",
                    250,
                    150,
                    50,
                    Color::WHITE
                );

                level_complete_back_button.draw(false, None, 1.0, false, &mut d);
            }
            GameState::EditorKeybinds => {
                d.clear_background(Color::BLACK);

                d.draw_text(
                    "Editor Keybinds:",
                    d.get_screen_width() / 2 - d.measure_text("Editor Keybinds:", 50) / 2,
                    100 - editor_guide_scroll as i32,
                    50,
                    Color::WHITE
                );

                d.draw_text(
                    "Press 1, 2, and 3 to switch tabs",
                    d.get_screen_width() / 2 - d.measure_text("Press 1, 2, and 3 to switch tabs", 30) / 2,
                    200 - editor_guide_scroll as i32,
                    30,
                    Color::WHITE
                );

                d.draw_text(
                    "Click on an object in the edit tab to select it!",
                    d.get_screen_width() / 2 - d.measure_text("Click on an object in the edit tab to select it!", 30) / 2,
                    250 - editor_guide_scroll as i32,
                    30,
                    Color::WHITE
                );

                d.draw_text(
                    "Use WASD to move selected objects!",
                    d.get_screen_width() / 2 - d.measure_text("Use WASD to move selected objects!", 30) / 2,
                    300 - editor_guide_scroll as i32,
                    30,
                    Color::WHITE
                );

                d.draw_text(
                    "Use Q and E to rotate selected objects!",
                    d.get_screen_width() / 2 - d.measure_text("Use Q and E to rotate selected objects!", 30) / 2,
                    350 - editor_guide_scroll as i32,
                    30,
                    Color::WHITE
                );

                d.draw_text(
                    "Use S on level select to pick the song for your level!",
                    d.get_screen_width() / 2 - d.measure_text("Use S on level select to pick the song for your level!", 28) / 2,
                    400 - editor_guide_scroll as i32,
                    28,
                    Color::WHITE
                );

                d.draw_text(
                    "Use the arrow keys to choose the difficulty of your level on the upload screen!",
                    d.get_screen_width() / 2 - d.measure_text("Use the arrow keys to choose the difficulty of your level on the upload screen!", 19) / 2,
                    450 - editor_guide_scroll as i32,
                    19,
                    Color::WHITE
                );

                d.draw_text(
                    "Use IJKL to move objects off grid!",
                    d.get_screen_width() / 2 - d.measure_text("Use IJKL to move objects off grid!", 30) / 2,
                    500 - editor_guide_scroll as i32,
                    30,
                    Color::WHITE
                );

                d.draw_text(
                    "Use < and > to move the starting position of your level!",
                    d.get_screen_width() / 2 - d.measure_text("Use < and > to move the starting position of your level!", 28) / 2,
                    550 - editor_guide_scroll as i32,
                    28,
                    Color::WHITE
                );

                menu_button.draw(false, None, 1.0, false, &mut d);
            }
            GameState::AccountPage => {
                d.clear_background(Color::BLACK);

                menu_button.draw(false, None, 1.0, false, &mut d);
                login_button.draw(false, None, 1.0, false, &mut d);
                register_button.draw(false, None, 1.0, false, &mut d);

                username_textbox.draw(username.clone(), &mut d);
                password_textbox.draw(password.clone(), &mut d);

                d.draw_text(
                    &register_result,
                    d.get_screen_width() / 2 - d.measure_text(&register_result, 50) / 2,
                    100,
                    50,
                    Color::WHITE
                );

                d.draw_text(
                    &login_result,
                    d.get_screen_width() / 2 - d.measure_text(&login_result, 50) / 2,
                    100,
                    50,
                    Color::WHITE
                );
            }
            GameState::LevelUpload => {
                d.clear_background(Color::BLACK);

                menu_button.draw(false, None, 1.0, false, &mut d);
                upload_button.draw(false, None, 1.0, false, &mut d);
                level_name_textbox.draw(level_name.clone(), &mut d);
                level_desc_textbox.draw(level_desc.clone(), &mut d);

                d.draw_texture_ex(
                    &difficulties[online_level_upload_diff as usize],
                    Vector2::new(
                        d.get_screen_width() as f32 / 2.0 - difficulties[online_level_upload_diff as usize].clone().width as f32 * if online_level_upload_diff == 0 { 0.3 } else { 0.2 } / 2.0,
                        if online_level_upload_diff == 0 { -30.0 } else { -80.0 }
                    ),
                    0.0,
                    if online_level_upload_diff == 0 { 0.3 } else { 0.2 },
                    Color::WHITE
                );

                d.draw_text(
                    &level_upload_result,
                    d.get_screen_width() / 2 - d.measure_text(&level_upload_result, 50) / 2,
                    100,
                    50,
                    Color::WHITE
                );
            }
            GameState::LevelPage => {
                d.clear_background(Color::BLACK);

                d.draw_text(
                    &online_level_name,
                    d.get_screen_width() / 2 - d.measure_text(&online_level_name, 50) / 2,
                    100,
                    50,
                    Color::WHITE
                );

                d.draw_rectangle(
                    d.get_screen_width() / 2 - 394,
                    d.get_screen_height() / 2 + 100,
                    789,
                    50,
                    Color {
                        r: 50,
                        g: 50,
                        b: 50,
                        a: 100
                    }
                );

                d.draw_text(
                    &online_level_desc,
                    d.get_screen_width() / 2 - d.measure_text(&online_level_desc, 30) / 2,
                    d.get_screen_height() / 2 + 100 + 10,
                    30,
                    Color::WHITE
                );

                d.draw_texture_ex(
                    &difficulties[online_level_diff as usize],
                    if online_level_diff == 0 { Vector2::new(10.0, 60.0) } else { Vector2::new(-50.0, 0.0) },
                    0.0,
                    if online_level_diff == 0 { 0.3 } else { 0.2 },
                    Color::WHITE
                );

                if online_level_rated {
                    d.draw_text(
                        &format!("{}", online_level_diff),
                        175,
                        260,
                        50,
                        Color::WHITE
                    );

                    d.draw_texture_ex(
                        &star_texture,
                        Vector2::new(30.0, 175.0),
                        0.0,
                        0.13,
                        Color::WHITE
                    );
                }

                d.draw_text(
                    &online_level_creator,
                    d.get_screen_width() / 2 - d.measure_text(&online_level_creator, 50) / 2,
                    20,
                    50,
                    Color::WHITE
                );

                level_play_button.draw(false, None, 1.0, false, &mut d);
                menu_button.draw(false, None, 1.0, false, &mut d);

                if is_mod {
                    level_rate_button.draw(false, None, 1.0, false, &mut d);
                }
            }
            GameState::SearchPage => {
                d.clear_background(Color::BLACK);

                if show_level_not_found {
                    d.draw_text(
                        &level_download_result,
                        d.get_screen_width() / 2 - d.measure_text(&level_download_result, 50) / 2,
                        d.get_screen_height() / 2 - 25,
                        50,
                        Color::WHITE
                    );
                }

                download_level_button.draw(false, None, 1.0, false, &mut d);
                level_id_textbox.draw(level_id.clone(), &mut d);
                menu_button.draw(false, None, 1.0, false, &mut d);
            }
            GameState::LevelRate => {
                d.clear_background(Color::BLACK);
                menu_button.draw(false, None, 1.0, false, &mut d);
                submit_rating_button.draw(false, None, 1.0, false, &mut d);

                d.draw_texture_ex(
                    &difficulties[online_level_rate_diff as usize],
                    Vector2::new(
                        d.get_screen_width() as f32 / 2.0 - difficulties[online_level_rate_diff as usize].clone().width as f32 * if online_level_rate_diff == 0 { 0.3 } else { 0.2 } / 2.0,
                        if online_level_rate_diff == 0 { -30.0 } else { -80.0 }
                    ),
                    0.0,
                    if online_level_rate_diff == 0 { 0.3 } else { 0.2 },
                    Color::WHITE
                );

                d.draw_text(
                    &level_rate_result,
                    d.get_screen_width() / 2 - d.measure_text(&level_rate_result, 50),
                    d.get_screen_height() - 50,
                    50,
                    Color::WHITE
                );
            }
        }
    }

    if been_to_editor {
        level_string = get_level_text(
            current_song,
            bg_red,
            bg_green,
            bg_blue,
            ground_red as u8,
            ground_green as u8,
            ground_blue as u8,
            &object_grid
        );

        let write_result = fs::write("./save-data/levels/level.txt", level_string);

        println!("{:?}", write_result);
    }

    let mut save_string = format!(
        "stars:{};user:{};pass:{};;;",

        stars,
        user,
        pass
    );

    let mut saving_index: u8 = 0;
    for completed in levels_completed_vec {
        if completed {
            save_string.push_str(&format!("{}:1;", saving_index));
        } else {
            save_string.push_str(&format!("{}:0;", saving_index));
        }

        saving_index += 1
    }

    save_string.push_str(";;0;");
    for id in online_levels_beaten.iter().skip(1) {
        save_string.push_str(&format!("{};", id));
    }

    save_string.pop();

    let write_save_result = fs::write("./save-data/save.txt", save_string);

    println!("{:?}", write_save_result);

    // Print statements to make unused variable warnings go away because rust is stupid
    println!("{:?}", cc_1001);
    println!("{:?}", cc_1002);
}

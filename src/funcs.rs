use raylib::prelude::*;
use crate::types::*;
use reqwest::Client;
use std::collections::HashMap;

impl Button {
    pub fn new(x: f32, y: f32, width: f32, height: f32, text: &str, font_size: i32, is_disabled: bool) -> Self {
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

    pub fn update(&mut self, rl: &RaylibHandle, delta_time: f32) {
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

    pub fn draw(&self, d: &mut RaylibDrawHandle) {
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

    pub fn is_hovered(&self, mouse_pos: Vector2) -> bool {
        self.rect.check_collision_point_rec(mouse_pos)
    }

    pub fn is_clicked(&self, rl: &RaylibHandle) -> bool {
        let mouse_pos = rl.get_mouse_position();
        self.is_hovered(mouse_pos) && rl.is_mouse_button_released(MouseButton::MOUSE_BUTTON_LEFT)
    }
}

impl TextBox {
    pub fn is_clicked(&self, rl: &RaylibHandle) -> bool {
        let mouse_pos = rl.get_mouse_position();
        self.rect.check_collision_point_rec(mouse_pos) && rl.is_mouse_button_pressed(MouseButton::MOUSE_BUTTON_LEFT)
    }

    pub fn is_not_clicked(&self, rl: &RaylibHandle) -> bool {
        let mouse_pos = rl.get_mouse_position();
        !self.rect.check_collision_point_rec(mouse_pos) && rl.is_mouse_button_pressed(MouseButton::MOUSE_BUTTON_LEFT)
    }

    pub fn draw(&self, text: String, d: &mut RaylibDrawHandle) {
        d.draw_rectangle(
            self.rect.x as i32,
            self.rect.y as i32,
            self.rect.width as i32,
            self.rect.height as i32,
            Color {
                r: 50,
                g: 50,
                b: 50,
                a: if self.active { 100 } else { 200 }
            }
        );

        d.draw_text(
            if !text.is_empty() { text.as_str() } else { self.text.as_str() },
            self.rect.x as i32 + 10,
            self.rect.y as i32 + self.rect.height as i32 / 2 - self.text_size as i32 / 2,
            self.text_size as i32,
            if !text.is_empty() { Color::WHITE } else { Color::GRAY }
        );
    }

    pub fn input(&self, text: &mut String, rl: &RaylibHandle) {
        if rl.is_key_pressed(KeyboardKey::KEY_BACKSPACE) && text.len() > 0 {
            text.pop();
        }

        if self.active && text.len() < self.max_length as usize {
            if rl.is_key_pressed(KeyboardKey::KEY_A) {
                text.push(if rl.is_key_down(KeyboardKey::KEY_LEFT_SHIFT) { 'A' } else { 'a' });
            }

            else if rl.is_key_pressed(KeyboardKey::KEY_B) {
                text.push(if rl.is_key_down(KeyboardKey::KEY_LEFT_SHIFT) { 'B' } else { 'b' });
            }

            else if rl.is_key_pressed(KeyboardKey::KEY_C) {
                text.push(if rl.is_key_down(KeyboardKey::KEY_LEFT_SHIFT) { 'C' } else { 'c' });
            }

            else if rl.is_key_pressed(KeyboardKey::KEY_D) {
                text.push(if rl.is_key_down(KeyboardKey::KEY_LEFT_SHIFT) { 'D' } else { 'd' });
            }

            else if rl.is_key_pressed(KeyboardKey::KEY_E) {
                text.push(if rl.is_key_down(KeyboardKey::KEY_LEFT_SHIFT) { 'E' } else { 'e' });
            }

            else if rl.is_key_pressed(KeyboardKey::KEY_F) {
                text.push(if rl.is_key_down(KeyboardKey::KEY_LEFT_SHIFT) { 'F' } else { 'f' });
            }

            else if rl.is_key_pressed(KeyboardKey::KEY_G) {
                text.push(if rl.is_key_down(KeyboardKey::KEY_LEFT_SHIFT) { 'G' } else { 'g' });
            }

            else if rl.is_key_pressed(KeyboardKey::KEY_H) {
                text.push(if rl.is_key_down(KeyboardKey::KEY_LEFT_SHIFT) { 'H' } else { 'h' });
            }

            else if rl.is_key_pressed(KeyboardKey::KEY_I) {
                text.push(if rl.is_key_down(KeyboardKey::KEY_LEFT_SHIFT) { 'I' } else { 'i' });
            }

            else if rl.is_key_pressed(KeyboardKey::KEY_J) {
                text.push(if rl.is_key_down(KeyboardKey::KEY_LEFT_SHIFT) { 'J' } else { 'j' });
            }

            else if rl.is_key_pressed(KeyboardKey::KEY_K) {
                text.push(if rl.is_key_down(KeyboardKey::KEY_LEFT_SHIFT) { 'K' } else { 'k' });
            }

            else if rl.is_key_pressed(KeyboardKey::KEY_L) {
                text.push(if rl.is_key_down(KeyboardKey::KEY_LEFT_SHIFT) { 'L' } else { 'l' });
            }

            else if rl.is_key_pressed(KeyboardKey::KEY_M) {
                text.push(if rl.is_key_down(KeyboardKey::KEY_LEFT_SHIFT) { 'M' } else { 'm' });
            }

            else if rl.is_key_pressed(KeyboardKey::KEY_N) {
                text.push(if rl.is_key_down(KeyboardKey::KEY_LEFT_SHIFT) { 'N' } else { 'n' });
            }

            else if rl.is_key_pressed(KeyboardKey::KEY_O) {
                text.push(if rl.is_key_down(KeyboardKey::KEY_LEFT_SHIFT) { 'O' } else { 'o' });
            }

            else if rl.is_key_pressed(KeyboardKey::KEY_P) {
                text.push(if rl.is_key_down(KeyboardKey::KEY_LEFT_SHIFT) { 'P' } else { 'p' });
            }

            else if rl.is_key_pressed(KeyboardKey::KEY_Q) {
                text.push(if rl.is_key_down(KeyboardKey::KEY_LEFT_SHIFT) { 'Q' } else { 'q' });
            }

            else if rl.is_key_pressed(KeyboardKey::KEY_R) {
                text.push(if rl.is_key_down(KeyboardKey::KEY_LEFT_SHIFT) { 'R' } else { 'r' });
            }

            else if rl.is_key_pressed(KeyboardKey::KEY_S) {
                text.push(if rl.is_key_down(KeyboardKey::KEY_LEFT_SHIFT) { 'S' } else { 's' });
            }

            else if rl.is_key_pressed(KeyboardKey::KEY_T) {
                text.push(if rl.is_key_down(KeyboardKey::KEY_LEFT_SHIFT) { 'T' } else { 't' });
            }

            else if rl.is_key_pressed(KeyboardKey::KEY_U) {
                text.push(if rl.is_key_down(KeyboardKey::KEY_LEFT_SHIFT) { 'U' } else { 'u' });
            }

            else if rl.is_key_pressed(KeyboardKey::KEY_V) {
                text.push(if rl.is_key_down(KeyboardKey::KEY_LEFT_SHIFT) { 'V' } else { 'v' });
            }

            else if rl.is_key_pressed(KeyboardKey::KEY_W) {
                text.push(if rl.is_key_down(KeyboardKey::KEY_LEFT_SHIFT) { 'W' } else { 'w' });
            }

            else if rl.is_key_pressed(KeyboardKey::KEY_X) {
                text.push(if rl.is_key_down(KeyboardKey::KEY_LEFT_SHIFT) { 'X' } else { 'x' });
            }

            else if rl.is_key_pressed(KeyboardKey::KEY_Y) {
                text.push(if rl.is_key_down(KeyboardKey::KEY_LEFT_SHIFT) { 'Y' } else { 'y' });
            }

            else if rl.is_key_pressed(KeyboardKey::KEY_Z) {
                text.push(if rl.is_key_down(KeyboardKey::KEY_LEFT_SHIFT) { 'Z' } else { 'z' });
            }

            else if rl.is_key_pressed(KeyboardKey::KEY_SPACE) && self.spaces_allowed {
                text.push(' ');
            }

            else if rl.is_key_pressed(KeyboardKey::KEY_ONE) {
                text.push('1');
            }

            else if rl.is_key_pressed(KeyboardKey::KEY_TWO) {
                text.push('2');
            }

            else if rl.is_key_pressed(KeyboardKey::KEY_THREE) {
                text.push('3');
            }

            else if rl.is_key_pressed(KeyboardKey::KEY_FOUR) {
                text.push('4');
            }

            else if rl.is_key_pressed(KeyboardKey::KEY_FIVE) {
                text.push('5');
            }

            else if rl.is_key_pressed(KeyboardKey::KEY_SIX) {
                text.push('6');
            }

            else if rl.is_key_pressed(KeyboardKey::KEY_SEVEN) {
                text.push('7');
            }

            else if rl.is_key_pressed(KeyboardKey::KEY_EIGHT) {
                text.push('8');
            }

            else if rl.is_key_pressed(KeyboardKey::KEY_NINE) {
                text.push('9');
            }

            else if rl.is_key_pressed(KeyboardKey::KEY_ZERO) {
                text.push('0');
            }
        }
    }
}

pub async fn get_request(url: String, params: Option<HashMap<String, String>>) -> String {
    let client = Client::builder()
        .build()
        .expect("Failed to build client");

    let mut request = client.get(url);

    if let Some(p) = params {
        request = request.query(&p);
    }

    let res = request
        .send()
        .await
        .expect("Failed to send request");

    let text = res.text().await.unwrap();
    return format!("{}", text);
}

pub async fn post_request(url: String, params: Option<HashMap<String, String>>) -> String {
    let client = Client::builder()
        .build()
        .expect("Failed to build client");

    let mut request = client.post(url);

    if let Some(p) = params {
        request = request.form(&p);
    }

    let res = request
        .send()
        .await
        .expect("Failed to send request");

    let text = res.text().await.unwrap();
    return format!("{}", text);
}

#[macro_export]
macro_rules! hashmap {
    ( $( $x:expr => $x1:expr ),* ) => {
        {
            let mut _temp_hashmap = HashMap::new();
            $(
                _temp_hashmap.insert($x, $x1);
            )*
            _temp_hashmap
        }
    };

    () => {
        HashMap::new()
    }
}

pub fn get_level_text(current_song: u8, bg_red: u8, bg_green: u8, bg_blue: u8, ground_red: u8, ground_green: u8, ground_blue: u8, object_grid: &Vec<ObjectStruct>) -> String {
    let mut level_string = format!(
        "version:BETA;name:hi;desc:testing level loading;song:{};c1001:{},{},{};c1002:{},{},{};c1004:255,255,255;bg:1;grnd:1;;;",

        current_song,

        bg_red,
        bg_green,
        bg_blue,

        ground_red,
        ground_green,
        ground_blue
    ).to_string();

    for object in object_grid {
        level_string.push_str( &format!("{}:{}:{}:{};", object.y, object.x, object.rotation, object.id));
    }

    level_string.pop();

    level_string
}

pub fn load_level(
    _level_metadata: &mut String,
    _object_string: &mut String,
    object_grid: &mut Vec<ObjectStruct>,

    bg_red: &mut u8,
    bg_green: &mut u8,
    bg_blue: &mut u8,

    ground_red: &mut i32,
    ground_green: &mut i32,
    ground_blue: &mut i32,

    song_selected: bool,
    current_song: &mut u8,
    load_song: bool,
    song_if_song_not_selected: bool
) {
    object_grid.clear();
    let metadata_pairs: Vec<&str> = _level_metadata.split(';').collect();
    for pair in metadata_pairs {
        let key_value: Vec<&str> = pair.split(':').collect();
        let key = key_value[0];
        let value = key_value[1];

        if key == "version" {
            if value == "ALPHA" {
                println!("Old level version detected.");
                println!("Please pick a level thats a newer version as that version isnt supported anymore.");
                break;
            } else if value == "BETA" {
                println!("Loading level...");
            } else {
                println!("Level version not recognized.");
                println!("Is this level made in a newer version?");
                break;
            }
        } else if key == "c1001" {
            let colors: Vec<&str> = value.split(',').collect();

            *bg_red = colors[0].parse::<u8>().unwrap();
            *bg_green = colors[1].parse::<u8>().unwrap();
            *bg_blue = colors[2].parse::<u8>().unwrap();
        } else if key == "c1002" {
            let colors: Vec<&str> = value.split(',').collect();

            *ground_red = colors[0].parse::<i32>().unwrap();
            *ground_green = colors[1].parse::<i32>().unwrap();
            *ground_blue = colors[2].parse::<i32>().unwrap();
        } else if key == "song" {
            if song_if_song_not_selected {
                if !song_selected && load_song {
                    *current_song = value.parse::<u8>().unwrap();
                }
            } else {
                if load_song {
                    *current_song = value.parse::<u8>().unwrap();
                }
            }
        }
    }

    let object_list: Vec<&str> = _object_string.split(';').collect();
    for object in object_list {
        let xyrid: Vec<&str> = object.split(':').collect();

        object_grid.push(ObjectStruct {
            y:xyrid[0].parse::<i32>().unwrap(),
            x:xyrid[1].parse::<i32>().unwrap(),
            rotation:xyrid[2].parse::<i16>().unwrap(),
            id:xyrid[3].parse::<u32>().unwrap(),
            selected:false
        });
    }
}

pub fn parse_level_download_response(
    level_download_result: String,
    online_level_name: &mut String,
    online_level_desc: &mut String,
    online_level_diff: &mut u8,
    online_level_rated: &mut bool,
    online_level_creator: &mut String,
    online_level_data: &mut String
) {
    let level_download_result_parts: Vec<&str> = level_download_result.split(";;;;;").collect();
    let name_desc: Vec<&str> = level_download_result_parts[0].split(";").collect();

    *online_level_name = name_desc[0].to_string();
    *online_level_desc = name_desc[1].to_string();
    *online_level_diff = name_desc[2].parse().unwrap();
    *online_level_creator = name_desc[4].parse().unwrap();
    *online_level_data = level_download_result_parts[1].to_string();

    if name_desc[3] == "0" {
        *online_level_rated = false
    } else {
        *online_level_rated = true
    }
}
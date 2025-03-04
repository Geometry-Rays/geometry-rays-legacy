use raylib::prelude::*;
use crate::types::*;
use reqwest::Client;

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

pub async fn make_request(url: String) -> String {
    let client = Client::builder()
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
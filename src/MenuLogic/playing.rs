use crate::types::GameMode;
use raylib::prelude::{RaylibHandle, KeyboardKey, Rectangle};

pub fn physics_handle(
    player: &mut Rectangle,
    current_gamemode: GameMode,
    is_on_ground: &mut bool,
    space_down: bool,
    mouse_down: bool,
    velocity_y: &mut f32,
    jump_force: f32,
    gravity: f32,
    touching_block_ceiling: bool,
    ship_power: f32,
    ship_falling_speed: f32,
    current_mode: String,
    world_offset: &mut f32,
    movement_speed: f32,
    moving_direction: &mut u8,
    rotation: &mut f32,
    player_cam_y: &mut i32,

    rl: &RaylibHandle
) {
    if current_gamemode == GameMode::Cube {
        if *is_on_ground && (space_down || mouse_down) {
            *velocity_y = jump_force;
            *is_on_ground = false;
        }
    } else if current_gamemode == GameMode::Ship {
        if !touching_block_ceiling {
            if mouse_down || space_down {
                if gravity > 0.0 {
                    if *velocity_y > -10.0 {
                        *velocity_y -= ship_power
                    }
                } else {
                    if *velocity_y < 10.0 {
                        *velocity_y += ship_power
                    }
                }
            } else {
                if gravity > 0.0 {
                    if *velocity_y < 10.0 {
                        *velocity_y += ship_falling_speed
                    }
                } else {
                    if *velocity_y > -10.0 {
                        *velocity_y -= ship_falling_speed
                    }
                }
            }
        } else {
            *velocity_y = 0.0
        }
    }

    if current_mode == "1" {
        *world_offset -= movement_speed;
    } else if current_mode == "2" {
        if rl.is_key_down(KeyboardKey::KEY_RIGHT) {
            *world_offset -= movement_speed;
            *moving_direction = 1
        } else if rl.is_key_down(KeyboardKey::KEY_LEFT) {
            *world_offset += movement_speed;
            *moving_direction = 2
        } else {
            *moving_direction = 0
        }
    }
    if current_gamemode == GameMode::Cube && *velocity_y < 20.0 && *velocity_y > -20.0 {
        *velocity_y += gravity;
    }
    player.y += *velocity_y as f32;

    if player.y >= 500.0 - *player_cam_y as f32 {
        player.y = 500.0;
        *velocity_y = 0.0;
        *is_on_ground = true;
        *rotation = 0.0;
    } else {
        if gravity > 0.0 {
            if *moving_direction == 1
            || current_mode == "1" {
                *rotation += 5.0;
            } else if *moving_direction == 2 {
                *rotation -= 5.0;
            } else {
                *rotation = 0.0;
            }
        } else {
            if *moving_direction == 1
            || current_mode == "1" {
                *rotation -= 5.0;
            } else if *moving_direction == 2 {
                *rotation += 5.0;
            } else {
                *rotation = 0.0;
            }
        }
    }

    if player.y >= 501.0 {
        *player_cam_y += *velocity_y as i32;
        player.y = 502.0
    }

    if player.y <= 50.0 {
        *player_cam_y += *velocity_y as i32;
        player.y = 49.0
    }
}
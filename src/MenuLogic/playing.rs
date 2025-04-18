use crate::types::{GameMode, GameState, ObjectStruct, MainLevel};
use raylib::prelude::{RaylibHandle, KeyboardKey, Rectangle, Color};

pub fn physics_handle(
    player: &mut Rectangle,
    current_gamemode: GameMode,
    is_on_ground: &mut bool,
    space_down: bool,
    mouse_down: bool,
    velocity_y: &mut f32,
    jump_force: f32,
    gravity: &mut f32,
    touching_block_ceiling: bool,
    ship_power: f32,
    ship_falling_speed: f32,
    wave_velocity: f32,
    current_mode: String,
    world_offset: &mut f32,
    movement_speed: f32,
    moving_direction: &mut u8,
    rotation: &mut f32,
    player_cam_y: &mut i32,

    rl: &RaylibHandle
) {
    if current_gamemode == GameMode::Cube {
        // This is what handles jumping if your in the cube
        if *is_on_ground && (space_down || mouse_down) {
            *velocity_y = jump_force;
            *is_on_ground = false;
        }
    } else if current_gamemode == GameMode::Ship {
        // This is what handles flying up and down in the ship
        // Back before I made the ship I planned on your gravity just changing if your holding
        // But I didn't go with that because then the ship physics would suck
        if !touching_block_ceiling {
            if mouse_down || space_down {
                if *gravity > 0.0 {
                    if *velocity_y > -10.0 {
                        *velocity_y -= ship_power
                    }
                } else {
                    if *velocity_y < 10.0 {
                        *velocity_y += ship_power
                    }
                }
            } else {
                if *gravity > 0.0 {
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
    } else if current_gamemode == GameMode::Ball {
        // This is what handles changing gravity if your in the ball
        if *is_on_ground && (space_down || mouse_down) {
            *gravity = -*gravity;
            *is_on_ground = false;
        }
    } else if current_gamemode == GameMode::Wave {
        if *gravity > 0.0 {
            if space_down || mouse_down {
                *velocity_y = -(wave_velocity * movement_speed)
            } else {
                *velocity_y = wave_velocity * movement_speed
            }
        } else {
            if space_down || mouse_down {
                *velocity_y = wave_velocity * movement_speed
            } else {
                *velocity_y = -(wave_velocity * movement_speed)
            }
        }
    }

    // This handles moving forward and backward
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

    // This handles making the player fall down
    if (current_gamemode == GameMode::Cube || current_gamemode == GameMode::Ball) && *velocity_y < 20.0 && *velocity_y > -20.0 {
        if current_gamemode == GameMode::Cube {
            *velocity_y += *gravity;
        } else {
            *velocity_y += *gravity - if *gravity > 0.0 { 0.2 } else { -0.2 };
        }
    }
    player.y += *velocity_y as f32;

    // This handles the ground logic and the player rotation
    if player.y >= 500.0 - *player_cam_y as f32 {
        player.y = 500.0 - *player_cam_y as f32;
        *velocity_y = 0.0;
        *is_on_ground = true;
        *rotation = 0.0;
    } else {
        // If in platformer the player only rotates if they are moving
        // They rotate different directions based on gravity and direction
        if *gravity > 0.0 {
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

    // This handles moving the camera up or down if the player is in a certain spot on the screen
    if player.y >= 501.0 {
        *player_cam_y += *velocity_y as i32;
        player.y = 502.0
    }

    if player.y <= 50.0 {
        *player_cam_y += *velocity_y as i32;
        player.y = 49.0
    }
}

// This is the function for checking hitboxes
pub fn hitbox_collision(
    object: &ObjectStruct,
    player: &mut Rectangle,
    centered_player: Rectangle,
    small_player: Rectangle,

    velocity_y: &mut f32,
    movement_speed: &mut f32,
    default_movement_speed: f32,
    gravity: &mut f32,
    default_gravity: f32,
    rotation: &mut f32,
    jump_force: &mut f32,
    default_jump_force: f32,
    cc_1003: &mut Color,

    kill_player: &mut bool,
    is_on_ground: &mut bool,
    on_orb: &mut bool,
    touching_block_ceiling: &mut bool,

    world_offset: &mut f32,
    player_cam_y: i32,

    current_gamemode: &mut GameMode,
    current_mode: String,

    mouse_down: bool,
    space_down: bool,
    touching_color_trigger: &mut bool,

    bg_red: &mut u8,
    bg_green: &mut u8,
    bg_blue: &mut u8,
    ground_red: &mut i32,
    ground_green: &mut i32,
    ground_blue: &mut i32,

    game_state: &mut GameState,
    in_custom_level: bool,
    stars: &mut u32,
    main_levels: &Vec<MainLevel>,
    current_level: usize,
    levels_completed_vec: &mut Vec<bool>,
    online_levels_beaten: &mut Vec<u16>,
    level_id: String,
    online_level_rated: bool,
    online_level_diff: u8
) {
    if object.id == 1 {
        *kill_player |= centered_player.check_collision_recs(&Rectangle {
            x: object.x as f32 + *world_offset + 20.0,
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
        if current_mode == "1" {
            *kill_player |= small_player.check_collision_recs(&Rectangle {
                x: object.x as f32 + *world_offset,
                y: object.y as f32 + 10.0 - player_cam_y as f32,
                width: 3.0,
                height: 20.0
            });
        } else if centered_player.check_collision_recs(&Rectangle {
            x: object.x as f32 + *world_offset,
            y: object.y as f32 + 20.0 - player_cam_y as f32,
            width: 3.0,
            height: 3.0
        }) {
            *world_offset = -(object.x as f32 - 220.0)
        } else if centered_player.check_collision_recs(&Rectangle {
            x: object.x as f32 + 40.0 + *world_offset,
            y: object.y as f32 + 20.0 - player_cam_y as f32,
            width: 3.0,
            height: 3.0
        }) {
            *world_offset = -(object.x as f32 - 140.0)
        }

        if centered_player.check_collision_recs(&Rectangle {
            x: object.x as f32 + *world_offset + 3.0,
            y: object.y as f32 + 1.0 - player_cam_y as f32,
            width: 37.0,
            height: 3.0
        }) {
            *is_on_ground = true;
            *rotation = 0.0;
            if !mouse_down {
                player.y = object.y as f32 - 19.0 - player_cam_y as f32;
                *velocity_y = 0.0;
            } else {
                if *gravity < 0.0 {
                    *touching_block_ceiling = true;
                    player.y = object.y as f32 - 21.0 - player_cam_y as f32;
                }
            }
        } else {
            *touching_block_ceiling = false;
        }

        if centered_player.check_collision_recs(&Rectangle {
            x: object.x as f32 + *world_offset + 3.0,
            y: object.y as f32 + 38.0 - player_cam_y as f32,
            width: 37.0,
            height: 3.0
        }) {
            *is_on_ground = true;
            *rotation = 0.0;
            if !mouse_down {
                player.y = object.y as f32 + 61.0 - player_cam_y as f32;
                *velocity_y = 0.0;
            } else {
                if *gravity > 0.0 {
                    *touching_block_ceiling = true;
                    player.y = object.y as f32 + 61.0 - player_cam_y as f32;
                }
            }
        } else {
            *touching_block_ceiling = false;
        }

        if centered_player.check_collision_recs(&Rectangle {
            x: object.x as f32 + *world_offset + 80.0,
            y: object.y as f32 - player_cam_y as f32 + 10.0,
            width: 3.0,
            height: 20.0,
        }) {
            *is_on_ground = false;
        }
    }

    if object.id == 3
    || object.id == 21 {
        if centered_player.check_collision_recs(&Rectangle {
            x: object.x as f32 + *world_offset,
            y: object.y as f32 + 35.0 - player_cam_y as f32,
            width: 40.0,
            height: 5.0
        }) {
            if object.id == 3 {
                if *gravity > 0.0 {
                    *velocity_y = -15.0;
                } else {
                    *velocity_y = 15.0
                }
            } else if object.id == 21 {
                if *gravity > 0.0 {
                    *velocity_y = -7.0;
                    *gravity = -default_gravity
                } else {
                    *velocity_y = 7.0;
                    *gravity = default_gravity
                }
            }
            *is_on_ground = false;
        }
    }

    if object.id == 4
    || object.id == 22 {
        if centered_player.check_collision_recs(&Rectangle {
            x: object.x as f32 - 10.0 + *world_offset,
            y: object.y as f32 - 10.0 - player_cam_y as f32,
            width: 60.0,
            height: 60.0
        }) {
            if *on_orb && (mouse_down || space_down) {
                if object.id == 4 {
                    if *gravity > 0.0 {
                        *velocity_y = -13.0;
                    } else {
                        *velocity_y = 13.0
                    }
                } else if object.id == 22 {
                    if *gravity > 0.0 {
                        *velocity_y = -7.0;
                        *gravity = -default_gravity
                    } else {
                        *velocity_y = 7.0;
                        *gravity = default_gravity
                    }
                }
                *on_orb = false
            }

            *is_on_ground = false
        }
    }

    if object.id == 5 || object.id == 6 {
        if centered_player.check_collision_recs(&Rectangle {
            x: object.x as f32 + *world_offset + if object.rotation == 0 || object.rotation == 180 || object.rotation == -180 { 10.0 } else { -20.0 },
            y: object.y as f32 - if object.rotation == 0 || object.rotation == 180 || object.rotation == -180 { 11.0 } else { -11.0 } - player_cam_y as f32,
            width: if object.rotation == 0 || object.rotation == 180 || object.rotation == -180 { 20.0 } else { 80.0 },
            height: if object.rotation == 0 || object.rotation == 180 || object.rotation == -180 { 80.0 } else { 20.0 }
        }) {
            if object.id == 5 {
                *jump_force = -default_jump_force;
                *gravity = -default_gravity;
            } else {
                *jump_force = default_jump_force;
                *gravity = default_gravity;
            }

            *is_on_ground = false
        }
    }

    if object.id == 7 {
        *kill_player |= centered_player.check_collision_recs(&Rectangle {
            x: object.x as f32 + *world_offset + 20.0,
            y: object.y as f32 + if object.rotation > 145 || object.rotation < -145 { 5.0 } else { 25.0 } - player_cam_y as f32,
            width: 10.0,
            height: 10.0
        });
    }

    if object.id == 8
    || object.id == 9
    || object.id == 24
    || object.id == 25 {
        if centered_player.check_collision_recs(&Rectangle {
            x: object.x as f32 + *world_offset + if object.rotation == 0 || object.rotation == 180 || object.rotation == -180 { 10.0 } else { -20.0 },
            y: object.y as f32 - if object.rotation == 0 || object.rotation == 180 || object.rotation == -180 { 11.0 } else { -11.0 } - player_cam_y as f32,
            width: if object.rotation == 0 || object.rotation == 180 || object.rotation == -180 { 20.0 } else { 80.0 },
            height: if object.rotation == 0 || object.rotation == 180 || object.rotation == -180 { 80.0 } else { 20.0 }
        }) {
            if object.id == 8 {
                *current_gamemode = GameMode::Cube;
                *cc_1003 = Color::LIME;
                *is_on_ground = false
            } else if object.id == 9 {
                *current_gamemode = GameMode::Ship;
                *cc_1003 = Color::MAGENTA;
                *is_on_ground = false
            } else if object.id == 24 {
                *current_gamemode = GameMode::Ball;
                *cc_1003 = Color::RED;
                *is_on_ground = false
            } else if object.id == 25 && current_mode == "1" {
                *current_gamemode = GameMode::Wave;
                *cc_1003 = Color::CYAN;
                *is_on_ground = false
            }
        }
    }

    if object.id == 15 {
        if centered_player.check_collision_recs(&Rectangle {
            x: object.x as f32 + *world_offset,
            y: object.y as f32 - player_cam_y as f32,
            width: 40.0,
            height: 40.0
        }) {
            if !in_custom_level && !levels_completed_vec[current_level] {
                *stars += main_levels[current_level].difficulty as u32;
                levels_completed_vec[current_level] = true
            } else if online_level_rated && in_custom_level {
                if !online_levels_beaten.contains(&level_id.parse().unwrap()) {
                    *stars += online_level_diff as u32;
                    online_levels_beaten.push(level_id.parse().unwrap());
                }
            }
            *game_state = GameState::LevelComplete;
        }
    }

    if object.id == 17 ||
    object.id == 18 ||
    object.id == 19 ||
    object.id == 20 {
        if centered_player.check_collision_recs(&Rectangle {
            x: object.x as f32 + *world_offset + if object.rotation == 0 || object.rotation == 180 || object.rotation == -180 { 10.0 } else { -20.0 },
            y: object.y as f32 - if object.rotation == 0 || object.rotation == 180 || object.rotation == -180 { 11.0 } else { -11.0 } - player_cam_y as f32,
            width: if object.rotation == 0 || object.rotation == 180 || object.rotation == -180 { 20.0 } else { 80.0 },
            height: if object.rotation == 0 || object.rotation == 180 || object.rotation == -180 { 80.0 } else { 20.0 }
        }) {
            *movement_speed = if object.id == 17 {
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
            x: object.x as f32 + *world_offset,
            y: object.y as f32 - player_cam_y as f32,
            width: 40.0,
            height: 40.0
        }) {
            let color_trigger_red: u8 = object.properties.clone().unwrap()[0].clone().parse().unwrap();
            let color_trigger_green: u8 = object.properties.clone().unwrap()[1].clone().parse().unwrap();
            let color_trigger_blue: u8 = object.properties.clone().unwrap()[2].clone().parse().unwrap();
            let color_trigger_type: u8 = object.properties.clone().unwrap()[3].clone().parse().unwrap();

            // println!("{:?}", object.properties.clone().unwrap());

            // let og_red = bg_red;
            // let og_green = bg_green;
            // let og_blue = bg_blue;

            if !*touching_color_trigger {
                if color_trigger_type == 1 {
                    *bg_red = color_trigger_red;
                    *bg_green = color_trigger_green;
                    *bg_blue = color_trigger_blue;
                } else if color_trigger_type == 2 {
                    *ground_red = color_trigger_red as i32;
                    *ground_green = color_trigger_green as i32;
                    *ground_blue = color_trigger_blue as i32;
                }
            }

            // cc_1001 = Color {
            //     r: ((og_red as u16 + color_trigger_red as u16) / 2) as u8,
            //     g: ((og_green as u16 + color_trigger_green as u16) / 2) as u8,
            //     b: ((og_blue as u16 + color_trigger_blue as u16) / 2) as u8,
            //     a: 255
            // };

            // if !touching_color_trigger {
            //     bg_red = ((og_red as u16 + color_trigger_red as u16) / 2) as u8;
            //     bg_green = ((og_green as u16 + color_trigger_green as u16) / 2) as u8;
            //     bg_blue = ((og_blue as u16 + color_trigger_blue as u16) / 2) as u8;
            //     touching_color_trigger = true;
            // }

            // Color {
            //     r: ((c1.r as u16 + c2.r as u16) / 2) as u8,
            //     g: ((c1.g as u16 + c2.g as u16) / 2) as u8,
            //     b: ((c1.b as u16 + c2.b as u16) / 2) as u8,
            // }

            // let mut index = color_trigger_fade as i16;
            // for _i in 0..color_trigger_fade as i32 {
            //     bg_red = ((og_red as i16 - index + color_trigger_red as i16) / 2) as u8;
            //     bg_green = ((og_green as i16 - index + color_trigger_green as i16) / 2) as u8;
            //     bg_blue = ((og_blue as i16 - index + color_trigger_blue as i16) / 2) as u8;

            //     index -= 1;
            // }
        }
    } else {
        *touching_color_trigger = false;
    }
}
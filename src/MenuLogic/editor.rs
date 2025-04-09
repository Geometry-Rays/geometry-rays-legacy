use raylib::prelude::{ RaylibHandle, KeyboardKey };

use crate::types::{ EditorTab, ObjectStruct, Button };

// PED stands for place, edit, delete
pub fn object_ped(
    object_grid: &mut Vec<ObjectStruct>,
    active_tab: EditorTab,
    snapped_x: i32,
    snapped_y: i32,
    current_object: u32,
    selected_object: &mut u16,

    no_touch_toggle: &mut Button,
    hide_toggle: &mut Button,
    object_settings: &mut Button,
    rl: &RaylibHandle
) {
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
                    "1".to_string()
                ]
            )} else {
                None
            }
        });
    } else if active_tab == EditorTab::Delete {
        let mut obj_index = 0;
        while obj_index < object_grid.len() {
            if object_grid[obj_index].x == if snapped_x < 0 { snapped_x - 40 } else { snapped_x }
            && object_grid[obj_index].y == if snapped_y < 0 { snapped_y - 40 } else { snapped_y } {
                object_grid.remove(obj_index);
                break;
            } else {
                obj_index += 1;
            }
        }
    } else if active_tab == EditorTab::Edit {
        let mut obj_index = 0;
        while obj_index < object_grid.len() {
            if object_grid[obj_index].x == if snapped_x < 0 { snapped_x - 40 } else { snapped_x }
            && object_grid[obj_index].y == if snapped_y < 0 { snapped_y - 40 } else { snapped_y }
            && !object_grid[obj_index].selected {
                if rl.is_key_up(KeyboardKey::KEY_LEFT_SHIFT) {
                    let mut objj_index = 0;
                    while objj_index < object_grid.len() {
                        object_grid[objj_index].selected = false;
                        objj_index += 1
                    }
                }

                object_grid[obj_index].selected = true;
                *selected_object = object_grid[obj_index].id as u16;

                if object_grid[obj_index].no_touch == 1 {
                    no_touch_toggle.is_disabled = false;
                } else {
                    no_touch_toggle.is_disabled = true;
                }

                if object_grid[obj_index].hide == 1 {
                    hide_toggle.is_disabled = false;
                } else {
                    hide_toggle.is_disabled = true;
                }

                if object_grid[obj_index].id == 23 {
                    object_settings.is_disabled = false
                } else {
                    object_settings.is_disabled = true
                }
                break;
            } else {
                obj_index += 1;
            }
        }
    }
}

pub fn keybinds_manager(
    object_grid: &mut Vec<ObjectStruct>,
    rl: &RaylibHandle,
    start_pos: &mut u16
) {
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

    if rl.is_key_down(KeyboardKey::KEY_PERIOD) {
        if rl.is_key_down(KeyboardKey::KEY_LEFT_CONTROL) {
            *start_pos += 25
        } else {
            *start_pos += 5;
        }
    }

    if rl.is_key_down(KeyboardKey::KEY_COMMA) && *start_pos > 0 {
        if rl.is_key_down(KeyboardKey::KEY_LEFT_CONTROL) {
            *start_pos -= 25
        } else {
            *start_pos -= 5;
        }
    }
}
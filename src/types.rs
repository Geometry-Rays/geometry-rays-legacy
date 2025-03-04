use raylib::prelude::*;

pub enum GameState {
    Menu,
    Playing,
    GameOver,
    CreatorMenu,
    Editor,
    LevelOptions,
    LevelSelect,
    LevelComplete,
    EditorKeybinds,
}

pub struct Button {
    pub rect: Rectangle,
    pub text: String,
    pub font_size: i32,
    pub base_color: Color,
    pub hover_scale: f32,
    pub press_offset: f32,
    pub is_pressed: bool,
    pub animation_timer: f32,
    pub is_disabled: bool,
}

pub struct MainLevel {
    pub name: String,
    pub difficulty: u8,
    pub song: String,
    pub artist: String,
    pub data: String
}

#[derive(PartialEq)]
pub enum GameMode {
    Cube,
    Ship
}

// Enums, Structs, And functions that are used by the editor
#[derive(PartialEq)]
pub enum EditorTab {
    Build,
    Edit,
    Delete
}

#[derive(Debug)]
#[allow(dead_code)]
pub struct ObjectStruct {
    pub y: i32,
    pub x: i32,
    pub rotation: i16,
    pub id: u32,
    pub selected: bool
}
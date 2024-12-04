use raylib::prelude::*;

struct Player {
    x: f32,
    y: f32,
    size: f32,
    velocity: f32,
}

impl Player {
    fn new(x: f32, y: f32, size: f32) -> Self {
        Self {
            x,
            y,
            size,
            velocity: 0.0, // Player speed
        }
    }

    fn update(&mut self, is_jumping: bool) {
        if is_jumping && self.y == 300.0 {
            self.velocity = -5.0; // Slower jump velocity
        }

        self.y += self.velocity;
        self.velocity += 0.25; // Slower gravity effect

        if self.y > 300.0 {
            self.y = 300.0;
            self.velocity = 0.0;
        }
    }

    fn draw(&self, d: &mut RaylibDrawHandle) {
        d.draw_rectangle(self.x as i32, self.y as i32, self.size as i32, self.size as i32, Color::BLUE);
    }
}

struct Obstacle {
    x: f32,
    y: f32,
    width: f32,
    height: f32,
    speed: f32,
}

impl Obstacle {
    fn new(x: f32, y: f32, width: f32, height: f32, speed: f32) -> Self {
        Self {
            x,
            y,
            width,
            height,
            speed: 0.05,
        }
    }

    fn update(&mut self) {
        self.x -= self.speed;
        if self.x < -self.width {
            self.x = 800.0;
        }
    }

    fn draw(&self, d: &mut RaylibDrawHandle) {
        d.draw_rectangle(self.x as i32, self.y as i32, self.width as i32, self.height as i32, Color::RED);
    }
}

fn reset_game(player: &mut Player, obstacle: &mut Obstacle) {
    player.x = 100.0;
    player.y = 300.0;
    player.velocity = 0.0;
    obstacle.x = 800.0;
    obstacle.y = 320.0;
}

fn main() {
    let (mut rl, thread) = raylib::init()
        .size(800, 450)
        .title("Geometry Rays")
        .build();

    let mut player = Player::new(100.0, 300.0, 30.0);
    let mut obstacle = Obstacle::new(800.0, 320.0, 30.0, 30.0, 2.0); // Slower obstacle speed
    let mut game_running = false;

    while !rl.window_should_close() {
        if !game_running {
            if rl.is_key_pressed(KeyboardKey::KEY_ENTER) {
                reset_game(&mut player, &mut obstacle);
                game_running = true;
            }

            let mut d = rl.begin_drawing(&thread);
            d.clear_background(Color::WHITE);
            d.draw_text("Geometry Rays", 280, 150, 40, Color::BLACK);
            d.draw_text("Press ENTER to Start", 270, 250, 30, Color::GRAY);
        } else {
            let is_jumping = rl.is_key_down(KeyboardKey::KEY_SPACE);

            player.update(is_jumping);
            obstacle.update();

            let mut d = rl.begin_drawing(&thread);
            d.clear_background(Color::WHITE);

            player.draw(&mut d);
            obstacle.draw(&mut d);

            if player.x < obstacle.x + obstacle.width
                && player.x + player.size > obstacle.x
                && player.y < obstacle.y + obstacle.height
                && player.y + player.size > obstacle.y
            {
                game_running = false;
            }

            d.draw_text("Press ESC to Quit", 10, 10, 20, Color::GRAY);
        }
    }
}
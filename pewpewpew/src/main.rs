use macroquad::prelude::*;

/// The speed at which UFOs move.
const SPEED: f32 = 1.0;
/// The threshold for spawning a new UFO.
const SPAWN_THRESHOLD: f32 = 994.0;
/// The number of after-images to keep track of.
const AFTER_IMAGE_COUNT: usize = 100;

/// Movement strategy for UFOs.
enum Strategy {
    /// Player-controlled UFO.
    Player,
    /// UFO moves linearly.
    Linear,
    /// UFO moves in a sine wave pattern.
    Sine,
    /// UFO moves randomly.
    Random,
}

impl Strategy {
    /// Returns a random enemy strategy.
    fn random_enemy() -> Self {
        match rand::gen_range(0, 3) {
            0 => Strategy::Linear,
            1 => Strategy::Sine,
            _ => Strategy::Random,
        }
    }

    /// Returns the color associated with the strategy.
    fn color(&self) -> Color {
        match self {
            Strategy::Player => BLUE,
            Strategy::Linear => GREEN,
            Strategy::Sine => YELLOW,
            Strategy::Random => RED,
        }
    }
}

/// A UFO is a flying object that moves according to a specific strategy.
struct Ufo {
    /// The x-coordinate of the UFO.
    x: f32,
    /// The y-coordinate of the UFO.
    y: f32,
    /// The color of the UFO.
    color: Color,
    /// The x-speed of the UFO.
    xspeed: f32,
    /// The y-speed of the UFO.
    yspeed: f32,
    /// The movement strategy of the UFO.
    movement: Strategy,
    /// The after-images of the UFO.
    after_images: Vec<(f32, f32, Color)>,
}

impl Ufo {
    /// Spawns a new UFO with a random enemy strategy.
    fn enemy() -> Self {
        let movement = Strategy::random_enemy();
        Ufo {
            x: screen_width(),
            y: screen_height() * 0.25 + rand::gen_range(0.0, screen_height() * 0.5),
            color: movement.color(),
            xspeed: -SPEED,
            yspeed: 0.0,
            movement,
            after_images: vec![],
        }
    }

    /// Spawns a new UFO controlled by the player.
    fn player() -> Self {
        Ufo {
            x: 100.0,
            y: 100.0,
            color: BLUE,
            xspeed: 0.0,
            yspeed: 0.0,
            movement: Strategy::Player,
            after_images: vec![],
        }
    }
}

/// The main game loop.
#[macroquad::main("pewpewpew")]
async fn main() {
    let mut ufos = vec![Ufo::enemy(), Ufo::player()];

    loop {
        spawn_enemy(&mut ufos);
        steer_ufos(&mut ufos);
        move_ufos(&mut ufos);
        check_collision(&mut ufos);
        draw_ufos(&ufos);
        next_frame().await;
    }
}

/// Spawns a new enemy with a certain probability.
fn spawn_enemy(ufos: &mut Vec<Ufo>) {
    if rand::gen_range(0.0, 1000.0) >= SPAWN_THRESHOLD {
        ufos.push(Ufo::enemy());
    }
}

/// Adjusts the movement of UFOs based on their strategy.
fn steer_ufos(ufos: &mut Vec<Ufo>) {
    for ufo in ufos {
        match ufo.movement {
            Strategy::Player => {
                // the player UFO follows the mouse cursor
                let (mouse_x, mouse_y) = mouse_position();
                let dx = mouse_x - ufo.x;
                let dy = mouse_y - ufo.y;
                let angle = dy.atan2(dx);
                ufo.xspeed = angle.cos() * SPEED;
                ufo.yspeed = angle.sin() * SPEED;
            }
            Strategy::Linear => {
                ufo.xspeed = -SPEED;
                ufo.yspeed = 0.0;
            }
            Strategy::Sine => {
                ufo.xspeed = -SPEED;
                ufo.yspeed = SPEED * (ufo.x as f32 / 100.0).sin();
            }
            Strategy::Random => {
                ufo.xspeed = rand::gen_range(-SPEED, SPEED);
                ufo.yspeed = rand::gen_range(-SPEED, SPEED);
            }
        }
    }
}

/// Moves UFOs based on their current speed.
fn move_ufos(ufos: &mut Vec<Ufo>) {
    for ufo in ufos {
        ufo.x += ufo.xspeed;
        ufo.y += ufo.yspeed;
        ufo.after_images.push((ufo.x, ufo.y, ufo.color));
        if ufo.after_images.len() > AFTER_IMAGE_COUNT {
            ufo.after_images.remove(0);
        }
    }
}

/// Draws UFOs on the screen.
fn draw_ufos(ufos: &[Ufo]) {
    for ufo in ufos {
        // draw the UFO itself using its color
        match ufo.movement {
            Strategy::Player => draw_poly(ufo.x, ufo.y, 5, 20.0, 0.0, ufo.color),
            _ => draw_circle(ufo.x, ufo.y, 20.0, ufo.color),
        }
        // draw the after images of the UFO
        for (x, y, color) in &ufo.after_images {
            draw_circle(*x, *y, 5.0, *color);
        }
    }
}

/// Checks for collisions between UFOs and updates their colors.
fn check_collision(ufos: &mut Vec<Ufo>) {
    for i in 0..ufos.len() {
        for j in i + 1..ufos.len() {
            let dx = ufos[i].x - ufos[j].x;
            let dy = ufos[i].y - ufos[j].y;
            let distance = (dx * dx + dy * dy).sqrt();
            if distance < 40.0 {
                ufos[i].color = Color::new(
                    rand::gen_range(0.0, 1.0),
                    rand::gen_range(0.0, 1.0),
                    rand::gen_range(0.0, 1.0),
                    1.0,
                );
                ufos[j].color = Color::new(
                    rand::gen_range(0.0, 1.0),
                    rand::gen_range(0.0, 1.0),
                    rand::gen_range(0.0, 1.0),
                    1.0,
                );
            }
        }
    }
}

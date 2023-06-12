use rand::Rng;

const MIN_STAR_X: i32 = 800;
const MAX_STAR_X: i32 = 1800;

const MIN_STAR_Y: i32 = 0;
const MAX_STAR_Y: i32 = 800;

const REMOVE_STAR_X: i32 = -100;

const MIN_ANGLE: f64 = -5.0;
const MAX_ANGLE: f64 = 5.0;

pub struct Star {
    pub x: i32,
    pub y: i32,
    pub width: u32,
    pub height: u32,
    pub angle: f64,
    pub exist: bool,
    pub path: &'static str
}

impl Star {
    pub fn new(width: u32, height: u32, path: &'static str) -> Star {
        Star {
            x: 0,
            y: 0,
            width,
            height,
            angle: 0.0,
            exist: false,
            path
        }
    }

    pub fn set(&mut self, speed: i32) {
        self.x -= speed;
        if !self.exist || self.x < REMOVE_STAR_X {
            let x = rand::thread_rng().gen_range(MIN_STAR_X..MAX_STAR_X);
            let y = rand::thread_rng().gen_range(MIN_STAR_Y..MAX_STAR_Y);
            let angle = rand::thread_rng().gen_range(MIN_ANGLE..MAX_ANGLE);

            self.x = x;
            self.y = y;
            self.angle = angle;
            self.exist = true;
        }
    }

    pub fn reset(&mut self) {
        self.x = 0;
        self.y = 0;
        self.angle = 0.0;
        self.exist = false;
    }
}
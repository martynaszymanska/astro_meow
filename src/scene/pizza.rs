use rand::Rng;

const MIN_Y: i32 = 100;
const MAX_Y: i32 = 700;

const MIN_X: i32 = 1000;
const MAX_X: i32 = 1500;

const REMOVE_POINT_X: i32 = -100;

pub struct Pizza {
    pub x: i32,
    pub y: i32,
    pub angle: f64,
    pub alive: bool,
    pub exist: bool
}

impl Pizza {
    pub fn new() -> Pizza {
        Pizza {
            x: 0,
            y: 0,
            angle: 0.0,
            alive: true,
            exist: false
        }
    }

    pub fn set(&mut self, speed: i32, factor: i32) {
        self.x -= speed;
        self.angle += speed as f64;
        if !self.exist || self.x < REMOVE_POINT_X {
            let y = rand::thread_rng().gen_range(MIN_Y..MAX_Y);
            let x = rand::thread_rng().gen_range((MIN_X + factor)..(MAX_X - factor));

            self.x = x;
            self.y = y;
            self.alive = true;
            self.exist = true;

        }
    }

    pub fn eat(&mut self) {
        self.alive = false;
    }

    pub fn reset(&mut self) {
        self.x = 0;
        self.y = 0;
        self.angle = 0.0;
        self.alive = true;
        self.exist = false;
    }
}
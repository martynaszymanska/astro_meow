use std::ops::Range;
use::rand::Rng;

const MIN_WIDTH: u32 = 50;
const MAX_WIDTH: u32 = 150;

const MIN_HEIGHT: u32 = 100;
const MAX_HEIGHT: u32 = 400;

const MIN_Y: i32 = 0;
const MAX_Y: i32 = 400;

const CREATE_POINT_X: i32 = 900;
const REMOVE_POINT_X: i32 = -100;

const MIN_ANGLE: f64 = -15.0;
const MAX_ANGLE: f64 = 15.0;

#[derive(PartialEq)]
pub struct BlackHole {
    pub x: i32,
    pub y: i32,
    pub width: u32,
    pub height: u32,
    pub angle: f64,
    pub exist: bool
}

impl BlackHole {
    pub fn new() -> BlackHole {
        BlackHole {
            x: 0,
            y: 0,
            width: 0,
            height: 0,
            angle: 0.0,
            exist: false
        }
    }

    pub fn set(&mut self, speed: i32) {
        self.x -= speed;
        if !self.exist || self.x < REMOVE_POINT_X {
            let y = rand::thread_rng().gen_range(MIN_Y..MAX_Y);
            let width = rand::thread_rng().gen_range(MIN_WIDTH..MAX_WIDTH);
            let height = rand::thread_rng().gen_range(MIN_HEIGHT..MAX_HEIGHT);
            let angle = rand::thread_rng().gen_range(MIN_ANGLE..MAX_ANGLE);

            self.x = CREATE_POINT_X;
            self.y = y;
            self.width = width;
            self.height = height;
            self.angle = angle;
            self.exist = true;
        }
    }

    pub fn reset(&mut self) {
        self.x = 0;
        self.y = 0;
        self.width = 0;
        self.height = 0;
        self.angle = 0.0;
        self.exist = false;
    }

    pub fn get_x_range(&self) -> Range<i32> {
        let half_width = (self.width / 2) as i32;
        let centre_point_x = self.x + half_width;
        (centre_point_x - half_width / 2)..(centre_point_x + half_width / 2)
    }

    pub fn get_y_range(&self) -> Range<i32> {
        let half_height = (self.height / 2) as i32;
        let centre_point_y = self.y + half_height;
        (centre_point_y - half_height / 2)..(centre_point_y + half_height / 2)
    }
}
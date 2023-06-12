use std::ops::Range;

const START_X: i32 = 200;
const START_Y: i32 = 200;

const JUMP_AMOUNT: i32 = 60;
const GRAVITY_AMOUNT: i32 = 5;

const TOP_BORDER: i32 = 50;
const BOTTOM_BORDER: i32 = 750;

const STARSHIP_HALF_SIZE: i32 = 40;

pub struct Starship {
    pub x: i32,
    pub y: i32,
    pub alive: bool,
}

impl Starship {
    pub fn new() -> Starship {
        Starship {
            x: START_X,
            y: START_Y,
            alive: true
        }
    }

    pub fn change_position_up(&mut self) {
        if self.y > TOP_BORDER {
            self.y -= JUMP_AMOUNT;
        }
    }

    pub fn change_position_down(&mut self) {
        self.y += GRAVITY_AMOUNT;
        if self.y > BOTTOM_BORDER {
            self.dead();
        }
    }

    pub fn dead(&mut self) {
        self.alive = false;
    }

    pub fn reset(&mut self) {
        self.x = START_X;
        self.y = START_Y;
        self.alive = true;
    }

    pub fn get_x_range(&self) -> Range<i32> {
        let centre_point_x = self.x + STARSHIP_HALF_SIZE;
        (centre_point_x - STARSHIP_HALF_SIZE)..(centre_point_x + STARSHIP_HALF_SIZE)
    }

    pub fn get_y_range(&self) -> Range<i32> {
        let centre_point_y = self.y + STARSHIP_HALF_SIZE;
        (centre_point_y - STARSHIP_HALF_SIZE)..(centre_point_y + STARSHIP_HALF_SIZE)
    }
}
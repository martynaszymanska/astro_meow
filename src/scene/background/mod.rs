mod star;

use crate::scene::background::star::Star;

const STAR_1_WIDTH: u32 = 30;
const STAR_1_HEIGHT: u32 = 30;
const STAR_1_PATH: &str = "src/res/img/star_1.png";

const STAR_2_WIDTH: u32 = 25;
const STAR_2_HEIGHT: u32 = 50;
const STAR_2_PATH: &str = "src/res/img/star_2.png";

const STAR_3_WIDTH: u32 = 30;
const STAR_3_HEIGHT: u32 = 30;
const STAR_3_PATH: &str = "src/res/img/star_3.png";

const STAR_4_WIDTH: u32 = 25;
const STAR_4_HEIGHT: u32 = 25;
const STAR_4_PATH: &str = "src/res/img/star_4.png";

const STAR_5_WIDTH: u32 = 30;
const STAR_5_HEIGHT: u32 = 30;
const STAR_5_PATH: &str = "src/res/img/star_5.png";

const STAR_6_WIDTH: u32 = 20;
const STAR_6_HEIGHT: u32 = 20;
const STAR_6_PATH: &str = "src/res/img/star_6.png";

const STAR_7_WIDTH: u32 = 10;
const STAR_7_HEIGHT: u32 = 10;
const STAR_7_PATH: &str = "src/res/img/star_7.png";

pub struct Background {
    pub stars: Vec<Star>
}

impl Background {
    pub fn new() -> Background {
        let mut background = Background {
            stars: Vec::new()
        };
        for _ in 0..5 {
            background.stars.push(Star::new(STAR_1_WIDTH, STAR_1_HEIGHT, STAR_1_PATH));
            background.stars.push(Star::new(STAR_2_WIDTH, STAR_2_HEIGHT, STAR_2_PATH));
            background.stars.push(Star::new(STAR_3_WIDTH, STAR_3_HEIGHT, STAR_3_PATH));
            background.stars.push(Star::new(STAR_4_WIDTH, STAR_4_HEIGHT, STAR_4_PATH));
            background.stars.push(Star::new(STAR_5_WIDTH, STAR_5_HEIGHT, STAR_5_PATH));
            background.stars.push(Star::new(STAR_6_WIDTH, STAR_6_HEIGHT, STAR_6_PATH));
            background.stars.push(Star::new(STAR_7_WIDTH, STAR_7_HEIGHT, STAR_7_PATH));
        }
        return background;
    }

    pub fn set(&mut self, speed: i32) {
        for star in self.stars.iter_mut() {
            star.set(speed);
        }
    }

    pub fn reset(&mut self) {
        for star in self.stars.iter_mut() {
            star.reset();
        }
    }
}
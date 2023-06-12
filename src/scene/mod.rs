mod starship;
mod black_hole;
mod pizza;
mod background;

use crate::scene::starship::Starship;
use crate::scene::black_hole::BlackHole;
use crate::scene::pizza::Pizza;
use crate::scene::background::Background;

const START_SPEED: i32 = 7;

pub struct Scene {
    pub starship: Starship,
    pub black_hole: BlackHole,
    pub pizza: Pizza,
    pub background: Background,
    pub fly: bool,
    pub points: i32,
    speed: i32,
    speed_more: bool
}

impl Scene {
    pub fn create() -> Scene {
        Scene {
            starship: Starship::new(),
            black_hole: BlackHole::new(),
            pizza: Pizza::new(),
            background: Background::new(),
            fly: false,
            points: 0,
            speed: START_SPEED,
            speed_more: true
        }
    }

    pub fn fly(&mut self) {
        self.fly = true
    }

    pub fn update(&mut self) {
        self.background.set(self.speed);
        self.black_hole.set(self.speed);
        self.set_starship();
        self.pizza.set(self.speed, self.black_hole.width as i32);
        self.fly = false;
        self.update_speed();
    }

    pub fn reset(&mut self) {
        self.background.reset();
        self.black_hole.reset();
        self.pizza.reset();
        self.starship.reset();
        self.points = 0;
        self.speed = START_SPEED;
        self.speed_more = true;
    }

    pub fn is_alive(&self) -> bool {
        self.starship.alive
    }
}

impl Scene {
    fn set_starship(&mut self) {
        if self.fly {
            self.starship.change_position_up();
        } else {
            self.starship.change_position_down();
        }
        self.check_collision();
        self.check_pizza();
    }

    fn check_collision(&mut self) {
        let mut is_x_collision = false;
        let mut is_y_collision = false;

        for x in self.black_hole.get_x_range() {
            if self.starship.get_x_range().contains(&x) {
                is_x_collision = true;
            }
        }
        for y in self.black_hole.get_y_range() {
            if self.starship.get_y_range().contains(&y) {
                is_y_collision = true;
            }
        }

        if is_x_collision && is_y_collision {
            self.starship.dead();
        }
    }

    fn check_pizza(&mut self) {
        if self.starship.get_x_range().contains(&self.pizza.x) && self.starship.get_y_range().contains(&self.pizza.y) {
            if self.pizza.alive {
                self.points += 1;
            }
            self.pizza.eat();
        }
    }

    fn update_speed(&mut self) {
        if self.points > 0 && self.points% 2 == 0 && self.speed_more {
            self.speed += 1;
            self.speed_more = false;
        }
        if self.points > 0 && self.points% 2 != 0 {
            self.speed_more = true
        }
    }
}
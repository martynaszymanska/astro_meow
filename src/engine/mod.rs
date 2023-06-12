extern crate sdl2;

pub(crate) mod game_state;
mod values;

use std::path::Path;

use sdl2::image::LoadTexture;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::WindowCanvas;
use sdl2::video::Window;

use crate::engine::game_state::GameState;
use crate::scene::Scene;

pub struct Engine {
    canvas: WindowCanvas,
    state: GameState
}

impl Engine {
    pub fn create(window: Window) -> Result<Engine, String> {
        let canvas = window.into_canvas().software().build().map_err(|e| e.to_string())?;
        Ok(Engine { canvas, state: GameState::New })
    }

    pub fn set_state(&mut self, new_state: GameState) {
        self.state = new_state;
    }
    pub fn should_update(&mut self) -> bool {
        self.state == GameState::Play
    }
    pub fn should_quit(&mut self) -> bool { self.state == GameState::Quit }

    pub fn display(&mut self, scene: &Scene) -> Result<(), String> {
        self.clear();
        match self.state {
            GameState::New => {
                self.welcome()?;
            }
            GameState::Play => {
                self.play(scene)?;
            }
            GameState::Pause => {
                self.pause()?;
            }
            GameState::GameOver => {
                self.game_over()?;
            }
            GameState::Quit => {
                self.quit()?;
            }
        }
        self.present();
        Ok(())
    }
}

impl Engine {
    fn welcome(&mut self) -> Result<(), String> {
        self.add_img(values::WELCOME_PATH,
                     values::WELCOME_X,
                     values::WELCOME_Y,
                     values::WELCOME_WIDTH,
                     values::WELCOME_HEIGHT,
                     values::NO_ANGLE)?;
        Ok(())
    }

    fn play(&mut self, scene: &Scene) -> Result<(), String> {
        for star in scene.background.stars.iter() {
           self.add_img(star.path, star.x, star.y, star.width, star.height, star.angle)?;
        }

        self.add_img(values::BLACK_HOLE_PATH, scene.black_hole.x, scene.black_hole.y, scene.black_hole.width, scene.black_hole.height, scene.black_hole.angle)?;
        self.add_img(values::STARSHIP_PATH, scene.starship.x, scene.starship.y, values::STARSHIP_WIDTH, values::STARSHIP_HEIGHT, values::STARSHIP_ANGLE)?;
        if scene.pizza.alive {
            self.add_img(values::PIZZA_PATH, scene.pizza.x, scene.pizza.y, values::PIZZA_WIDTH, values::PIZZA_HEIGHT, scene.pizza.angle)?;
        }
        self.add_text(scene.points.to_string(), values::SCORE_TEXT_X, values::SCORE_TEXT_Y, values::SCORE_TEXT_WIDTH, values::SCORE_TEXT_HEIGHT)?;
        Ok(())
    }

    fn pause(&mut self) -> Result<(), String> {
        self.add_img(values::PAUSE_PATH,
                     values::PAUSE_X,
                     values::PAUSE_Y,
                     values::PAUSE_WIDTH,
                     values::PAUSE_HEIGHT,
                     values::NO_ANGLE)?;
        Ok(())
    }

    fn game_over(&mut self) -> Result<(), String> {
        self.add_img(values::GAME_OVER_PATH,
                     values::GAME_OVER_X,
                     values::GAME_OVER_Y,
                     values::GAME_OVER_WIDTH,
                     values::GAME_OVER_HEIGHT,
                     values::NO_ANGLE)?;
        Ok(())
    }

    fn quit(&mut self) -> Result<(), String> {
        self.add_img(values::QUIT_PATH,
                     values::QUIT_X,
                     values::QUIT_Y,
                     values::QUIT_WIDTH,
                     values::QUIT_HEIGHT,
                     values::NO_ANGLE)?;
        Ok(())
    }
}

impl Engine {
    fn clear(&mut self) {
        self.canvas.clear();
    }
    fn present(&mut self) {
        self.canvas.present();
    }

    fn add_img(&mut self, path: &str, x: i32, y: i32, width: u32, height: u32, angle: f64) -> Result<(), String> {
        let texture_creator = self.canvas.texture_creator();
        let texture = texture_creator.load_texture(Path::new(path))?;
        let target =  Rect::new(x, y, width, height);
        self.canvas.copy_ex(&texture, None, Some(target), angle, None, false, false)?;
        Ok(())
    }

    fn add_text(&mut self, text: String, x: i32, y: i32, width: u32, height: u32) -> Result<(), String> {
        let ttf_context = sdl2::ttf::init().map_err(|e| e.to_string())?;
        let mut font = ttf_context.load_font(values::FONT_PATH, 128)?;
        font.set_style(sdl2::ttf::FontStyle::BOLD);
        let surface = font
            .render(text.as_str())
            .blended(Color::WHITE)
            .map_err(|e| e.to_string())?;
        let texture_creator = self.canvas.texture_creator();
        let texture = texture_creator.create_texture_from_surface(&surface).map_err(|e| e.to_string())?;
        let target =  Rect::new(x, y, width, height);
        self.canvas.copy(&texture, None, Some(target))?;
        Ok(())
    }
}
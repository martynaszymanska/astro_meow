extern crate sdl2;

mod engine;
mod scene;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::image::InitFlag;

use crate::engine::Engine;
use crate::engine::game_state::GameState;
use crate::scene::Scene;

const WINDOW_SIZE: u32 = 800;
const WINDOW_NAME: &str = "Astro Meow";

pub fn main() -> Result<(), String> {
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;
    let _image_context = sdl2::image::init(InitFlag::PNG)?;
    let window = video_subsystem
        .window(WINDOW_NAME, WINDOW_SIZE, WINDOW_SIZE)
        .position_centered()
        .opengl()
        .build()
        .map_err(|e| e.to_string())?;

    let mut engine = Engine::create(window)?;
    let mut scene = Scene::create();

    let mut event_pump = sdl_context.event_pump()?;
    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. } => {
                    break 'running;
                }
                Event::KeyDown { keycode: Some(Keycode::Escape), .. }
                | Event::KeyDown { keycode: Some(Keycode::Q), .. } => {
                    scene.reset();
                    engine.set_state(GameState::Quit);
                }
                Event::KeyDown { keycode: Some(Keycode::N), .. } => {
                    engine.set_state(GameState::New);
                    scene.reset();
                }
                Event::KeyDown { keycode: Some(Keycode::P), .. } => {
                    engine.set_state(GameState::Pause);
                }
                Event::KeyDown { keycode: Some(Keycode::Space), .. } => {
                    engine.set_state(GameState::Play);
                    scene.fly();
                    if !scene.is_alive() {
                        engine.set_state(GameState::New);
                        scene.reset();
                    }
                }
                Event::KeyDown { keycode: Some(Keycode::Y), .. } => {
                    if engine.should_quit() {
                        break 'running;
                    }
                }
                _ => {}
            }
        }
        if engine.should_update() {
            scene.update();
        }
        if !scene.is_alive() {
            engine.set_state(GameState::GameOver);
        }
        engine.display(&scene)?;
    }

    Ok(())
}

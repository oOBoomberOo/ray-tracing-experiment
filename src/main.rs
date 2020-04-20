#![feature(clamp)]
#![feature(iterator_fold_self)]

mod camera;
mod component;
mod scene;
mod serialize;

use serialize::Config;

use anyhow::Result;
use log::*;
use scene::Scene;
use winit::event::{Event, VirtualKeyCode};
use winit::event_loop::{ControlFlow, EventLoop};
use winit_input_helper::WinitInputHelper;
use std::fs::File;

fn main() -> Result<()> {
    simple_logger::init()?;

    let config = read_config()?;

    let scene = Scene::new(config)?;
    let event_loop = EventLoop::new();
    let mut input = WinitInputHelper::new();
    let window = scene.window().build(&event_loop)?;
    let mut pixels = scene.pixels(&window)?;

    event_loop.run(move |event, _, control| {
        if let Event::RedrawRequested(_) = event {
            scene.render(pixels.get_frame());

            if let Err(e) = pixels.render() {
                error!("pixels render error: {}", e);
                *control = ControlFlow::Exit;
                return;
            }
        }

        if input.update(event) {
            if input.key_pressed(VirtualKeyCode::Escape) || input.quit() {
                *control = ControlFlow::Exit;
                return;
            }

            if let Some(size) = input.window_resized() {
                pixels.resize(size.width, size.height);
            }

            scene.update(&input);
            window.request_redraw();
        }
    });
}

fn read_config() -> anyhow::Result<Config> {
    let config_file = File::open("assets/world.json")?;
    let result: Config = serde_json::from_reader(config_file)?;
    Ok(result)
}
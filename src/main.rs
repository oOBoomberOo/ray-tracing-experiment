#![feature(clamp)]
#![feature(iterator_fold_self)]

mod camera;
mod component;
mod scene;
mod serialize;

use serialize::Config;

use anyhow::Result;
use scene::Scene;
use std::fs::File;
use winit::{
	event::{Event, VirtualKeyCode},
	event_loop::{ControlFlow, EventLoop},
};
use winit_input_helper::WinitInputHelper;

fn main() -> Result<()> {
	let config = read_config()?;

	let mut scene = Scene::new(config)?;
	let event_loop = EventLoop::new();
	let mut input = WinitInputHelper::new();
	let window = scene.window().build(&event_loop)?;
	let mut pixels = scene.pixels(&window)?;

	event_loop.run(move |event, _, control| {
		if let Event::RedrawRequested(_) = event {
			scene.render(pixels.get_frame());

			if let Err(e) = pixels.render() {
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

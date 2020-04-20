use super::*;
use image::{DynamicImage, GenericImageView, Rgba};
use std::{fmt, path::PathBuf};

#[derive(Clone)]
pub struct Texture {
	path: PathBuf,
	texture: DynamicImage,
}

impl Texture {
	pub fn new(path: String) -> Texture {
		let path = PathBuf::from(path);
		let texture = image::open(&path).unwrap_or_else(|_| DynamicImage::new_rgb8(1, 1));
		Texture { path, texture }
	}

	pub fn color(&self, texture_coord: &TextureCoord) -> Color {
		let (x, y) = self.wrap(texture_coord);
		let pixel: Rgba<u8> = self.texture.get_pixel(x, y);
		let [red, green, blue, _] = pixel.0;
		let (red, green, blue) = (
			red as f32 / 256.0,
			green as f32 / 256.0,
			blue as f32 / 256.0,
		);
		Color::new(red, green, blue)
	}

	fn wrap(&self, texture_coord: &TextureCoord) -> (u32, u32) {
		let x = self.wrap_value(texture_coord.x, self.texture.width());
		let y = self.wrap_value(texture_coord.y, self.texture.width());
		(x, y)
	}

	fn wrap_value(&self, val: f32, bound: u32) -> u32 {
		let signed_bound = bound as i32;
		let float_coord = val * bound as f32;
		let wrapped_coord = (float_coord as i32) % signed_bound;

		if wrapped_coord < 0 {
			(wrapped_coord + signed_bound) as u32
		}
		else {
			wrapped_coord as u32
		}
	}
}

impl fmt::Debug for Texture {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(f, "{}", self.path.display())
	}
}

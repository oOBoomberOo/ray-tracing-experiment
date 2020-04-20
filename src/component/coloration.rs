use super::*;
use crate::serialize::ColorConfig;

#[derive(Debug, Clone)]
pub enum Coloration {
	Color(Color),
	Texture(Texture),
	Other,
}

impl Coloration {
	pub fn color(&self, texture_coord: &TextureCoord) -> Color {
		match self {
			Self::Color(color) => *color,
			Self::Texture(tex) => tex.color(texture_coord),
			Self::Other => Color::BACKGROUND,
		}
	}
}

impl From<ColorConfig> for Coloration {
	fn from(color: ColorConfig) -> Self {
		match color {
			ColorConfig::Color(c) => Coloration::Color(c.into()),
			ColorConfig::Texture { texture } => Coloration::Texture(Texture::new(texture)),
		}
	}
}

impl Default for Coloration {
	fn default() -> Self {
		Coloration::Other
	}
}

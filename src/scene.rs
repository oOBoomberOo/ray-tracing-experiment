use crate::camera::*;
use crate::serialize::Config;
use crate::component::*;
use pixels::wgpu::Surface;
use pixels::{Error, Pixels, SurfaceTexture};
use ultraviolet::Vec2;
use winit::dpi::LogicalSize;
use winit::window::{Window, WindowBuilder};
use winit_input_helper::WinitInputHelper;
use anyhow::Result;

pub const SHADOW_BIAS: f32 = 1e-3;
pub const RECURSION_DEPTH: usize = 20;

pub struct Scene {
    pub width: u32,
    pub height: u32,
    entities: Entities,
    lights: Lights,
    camera: Camera
}

impl Scene {
    pub fn new(config: Config) -> Result<Scene> {
        let (entities, lights, camera) = config.compile()?;
        let (width, height) = camera.screen();
        let (width, height) = (width as u32, height as u32);
        
        let result = Scene { width, height, entities, lights, camera };
        Ok(result)
    }

    pub fn render(&self, frame: &mut [u8]) {
        for (index, pixel) in frame.chunks_exact_mut(4).enumerate() {
            let pos = self.from_index(index as u32);
            let ray = Ray::create_prime(pos, &self.camera);
            let color = if let Some(intersection) = self.compute_pixel(&ray) {
                self.compute_light(intersection, 0)
            } else {
                Color::BACKGROUND
            };

            let color = color.display();
            pixel.copy_from_slice(&color);
        }
    }

    pub fn update(&mut self, input: &WinitInputHelper) {
        self.camera.update(input)
    }
}

impl Scene {
    fn from_index(&self, index: u32) -> Vec2 {
        let width = self.width;

        let x = index % width;
        let y = index / width;

        Vec2::new(x as f32, y as f32)
    }

    pub fn compute_pixel(&self, ray: &Ray) -> Option<Intersection> {
        self.entities
            .iter()
            .filter_map(|x| x.intersect(ray))
            .min()
    }

    pub fn compute_light(&self, intersection: Intersection, depth: usize) -> Color {
        self.lights
            .iter()
            .map(|light| light.compute_light(&intersection, &self, depth))
            .fold_first(|a, b| a + b)
            .map_or(Color::BACKGROUND, |color| color.clamp())
    }
}

impl Scene {
    pub fn window(&self) -> WindowBuilder {
        WindowBuilder::new()
            .with_title("Raycasting da yo")
            .with_inner_size(self.size())
    }

    fn size(&self) -> LogicalSize<f32> {
        self.camera.get_size()
    }

    fn surface(&self, window: &Window) -> Surface {
        Surface::create(window)
    }

    fn surface_texture(&self, window: &Window) -> SurfaceTexture {
        let width = self.width;
        let height = self.height;
        let surface = self.surface(window);
        SurfaceTexture::new(width, height, surface)
    }

    pub fn pixels(&self, window: &Window) -> Result<Pixels, Error> {
        let width = self.width;
        let height = self.height;
        let surface_texture = self.surface_texture(window);
        Pixels::new(width, height, surface_texture)
    }
}

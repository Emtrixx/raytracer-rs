use image::{DynamicImage, GenericImage};
use glam::{Vec3};

pub struct Color {
    pub red: f32,
    pub green: f32,
    pub blue: f32,
}

pub trait Intersectable {
    fn intersect(&self, ray: &Ray) -> bool;
}

pub struct Sphere {
    pub center: Vec3,
    pub radius: f32,
    pub color: Color,
}

impl Intersectable for Sphere {
    fn intersect(&self, ray: &Ray) -> bool {
        //from origin to center of sphere
        let line_to_center = self.center - ray.origin;
        let ray_section_length_squared = line_to_center.dot(ray.direction);
        //Faster than standard Pythagoras: h**2 - adj**2
        let connection_squared = line_to_center.dot(line_to_center) - ray_section_length_squared;

        connection_squared < (self.radius * self.radius)

    }
}

pub struct Scene {
    pub width: u32,
    pub height: u32,
    pub fov: f32,
    pub sphere: Sphere
}

pub struct Ray {
    pub origin: Vec3,
    pub direction: Vec3,
}

impl Ray {
    pub fn create_prime(x: u32, y: u32, scene: &Scene) -> Ray {
        let fov_adjustement = to_radians(scene.fov)
        let aspect_ratio = (scene.width as f32) / (scene.height as f32);
        let viewport_x = (((x as f32 + 0.5) / scene.width as f32) * 2.0 - 1.0) * aspect_ratio;
        let viewport_y = 1.0 - ((y as f32 + 0.5) / scene.height as f32) * 2.0;

        Ray {
            origin: Vec3::ZERO, 
            direction: Vec3::new(viewport_x, viewport_y, -1.0).normalize(),
        }
    }
}

pub fn render(scene: &Scene) -> DynamicImage {
    DynamicImage::new_rgb8(scene.width, scene.height)
}

pub fn to_radians(x: f32) -> f32 {
    x / 57.296
}
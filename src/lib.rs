use image::{DynamicImage, GenericImage};
use glam::{Vec3};

#[derive(Debug, Clone, Copy)]
pub struct Color {
    pub red: f32,
    pub green: f32,
    pub blue: f32,
}


impl Color {
    pub fn to_rgba(&self) -> image::Rgba<u8> {
        image::Rgba::<u8>([self.red as u8,self.green as u8,self.blue as u8,0])
    }
}

pub trait Intersectable {
    fn intersect(&self, ray: &Ray) -> Option<f32>;
}

pub enum Element {
    Sphere(Sphere),
    Plane(Plane),
}

impl Intersectable for Element {
    fn intersect(&self ,ray: &Ray) -> Option<f32> {
        match *self {
            Element::Sphere(ref s) => s.intersect(ray),
            Element::Plane(ref p) => p.intersect(ray),
        }
    }
}

impl Element {
    pub fn color(&self) -> Color {
        match *self {
            Element::Sphere(ref s) => s.color,
            Element::Plane(ref p) => p.color,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Sphere {
    pub center: Vec3,
    pub radius: f32,
    pub color: Color,
}

impl Intersectable for Sphere {
    fn intersect(&self, ray: &Ray) -> Option<f32> {
        //from origin to center of sphere
        let line_to_center = self.center - ray.origin;
        let ray_section_length = line_to_center.dot(ray.direction);
        //Faster than standard Pythagoras: h**2 - adj**2
        let connection_squared = line_to_center.dot(line_to_center) - ray_section_length * ray_section_length;
        let radius_squared = self.radius * self.radius;

        if connection_squared > radius_squared {
            return None;
        }
        
        let thic = ((self.radius * self.radius) - connection_squared).sqrt();
        
        // let connection = connection_squared.sqrt();

        // let right_angle_point = self.center + connection;

        let distance_intersection1 = ray_section_length - thic;
        let distance_intersection2 = ray_section_length + thic;

        if distance_intersection1 < 0. && distance_intersection2 < 0. {
            return None;
        };

        let dist = if distance_intersection1 < distance_intersection2 { distance_intersection1 } else { distance_intersection2 };
        Some(dist)
    }
}

pub struct Plane {
    pub point: Vec3,
    pub normal: Vec3,
    pub color: Color
}

impl Intersectable for Plane {
    fn intersect(&self, ray: &Ray) -> Option<f32> {
        Some(3.)
    }
}

// FROM TUTORIAL MAYBE USELESS
// pub struct Intersection<'a> {
//     pub distance: f32,
//     pub object: &'a Sphere,
// }

// impl<'a> Intersection<'a> {
//     pub fn new<'b>(distance: f32, object: &'b Sphere) -> Intersection<'b> {
//         //
//     }
// }
// impl Scene {
//     pub fn trace(&self, ray: &Ray) -> Option<Intersection> {
//         self.spheres
//     }
// }

pub struct Scene {
    pub width: u32,
    pub height: u32,
    pub fov: f32,
    pub elements: Vec<Element>,
}


pub struct Ray {
    pub origin: Vec3,
    pub direction: Vec3,
}

impl Ray {
    pub fn create_prime(x: u32, y: u32, scene: &Scene) -> Ray {
        let fov_adjustement = (to_radians(scene.fov) / 2.).tan() ;
        let aspect_ratio = (scene.width as f32) / (scene.height as f32);
        let viewport_x = ((((x as f32 + 0.5) / scene.width as f32) * 2.0 - 1.0) * aspect_ratio) * fov_adjustement;
        let viewport_y = (1.0 - ((y as f32 + 0.5) / scene.height as f32) * 2.0) * fov_adjustement;
        
        Ray {
            origin: Vec3::ZERO, 
            direction: Vec3::new(viewport_x, viewport_y, -1.0).normalize(),
        }
    }
}

pub fn render(scene: &Scene) -> DynamicImage {
    let mut image = DynamicImage::new_rgb8(scene.width, scene.height);
    let black = image::Rgba::<u8>([0,0,0,0]);

    for x in 0..scene.width {
        for y in 0..scene.height {
            //standard pixel black
            image.put_pixel(x, y, black);
            let ray = Ray::create_prime(x, y, &scene);

            let mut nearest = f32::INFINITY;
            for sphere in &scene.elements {
                if let Some(dist) = sphere.intersect(&ray) {
                    if dist < nearest {
                        nearest = dist;
                        image.put_pixel(x, y, sphere.color().to_rgba());
                    }
                } 
            }
        };
    };
    image
}

pub fn to_radians(x: f32) -> f32 {
    x / 57.296
}
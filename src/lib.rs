use image::{DynamicImage, GenericImage};
use glam::{Vec3};

//Scene
pub struct Scene {
    pub width: u32,
    pub height: u32,
    pub fov: f32,
    pub elements: Vec<Element>,
    pub lights: Vec<Light>,
}

impl Scene {
    fn trace(&self, ray: &Ray) -> Option<Intersection> {
        let mut intersection: Option<Intersection> = None;
        let mut nearest = f32::INFINITY;
        for element in &self.elements {
            if let Some(dist) = element.intersect(ray) {
                if dist < nearest {
                    nearest = dist;
                    intersection = Some(Intersection {
                        element,
                        distance: nearest
                    });
                }
            } 
        }
        intersection
    }
}


//Color
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
    pub fn add(&self, color: Color) -> Color {
        let red = self.red + color.red;
        let green = self.green + color.green;
        let blue = self.blue + color.blue;
        Color {red, green, blue}
    }
    pub fn multiply(&self, color: Color) -> Color {
        let red = self.red * color.red;
        let green = self.green * color.green;
        let blue = self.blue * color.blue;
        Color {red, green, blue}
    }
    pub fn multiply_scalar(&self, scalar: f32) -> Color {
        Color {
            red: self.red * scalar,
            green: self.green * scalar,
            blue: self.blue * scalar,
        }
    }
    pub fn clamp(self) -> Color {
        let red = (self.red).min(255.);
        let green = (self.green).min(255.);
        let blue = (self.blue).min(255.);
        Color {red, green, blue}
    }
}

//Material
pub struct Material {
    pub color: Color,
    pub albedo: f32,
    pub specular: f32,
    pub reflectivity: f32,
}


//Element
pub trait Intersectable {
    fn intersect(&self, ray: &Ray) -> Option<f32>;
    fn surface_normal(&self, hit_point: &Vec3) -> Vec3;
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
    fn surface_normal(&self ,hit_point: &Vec3) -> Vec3 {
        match *self {
            Element::Sphere(ref s) => s.surface_normal(hit_point),
            Element::Plane(ref p) => p.surface_normal(hit_point),
        }
    }
}

impl Element {
    pub fn color(&self) -> Color {
        match *self {
            Element::Sphere(ref s) => s.material.color,
            Element::Plane(ref p) => p.material.color,
        }
    }
    pub fn albedo(&self) -> f32 {
        match *self {
            Element::Sphere(ref s) => s.material.albedo,
            Element::Plane(ref p) => p.material.albedo,
        }
    }
    pub fn specular(&self) -> f32 {
        match *self {
            Element::Sphere(ref s) => s.material.specular,
            Element::Plane(ref p) => p.material.specular,
        }
    }
    pub fn reflectivity(&self) -> f32 {
        match *self {
            Element::Sphere(ref s) => s.material.reflectivity,
            Element::Plane(ref p) => p.material.reflectivity,
        }
    }
}


//Sphere
pub struct Sphere {
    pub center: Vec3,
    pub radius: f32,
    pub material: Material,
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
    fn surface_normal(&self, hit_point: &Vec3) -> Vec3 {
        (*hit_point - self.center).normalize()
    }
}

//Plane
pub struct Plane {
    pub origin: Vec3,
    pub normal: Vec3,
    pub material: Material,
}

impl Intersectable for Plane {
    fn intersect(&self, ray: &Ray) -> Option<f32> {
        let normal = self.normal;
        let denom = normal.dot(ray.direction);
        if denom > 1e-6 {
            let v = self.origin - ray.origin;
            let distance = v.dot(normal) / denom;
            if distance >= 0.0 {
                return Some(distance);
            }
        }
        None
    }
    fn surface_normal(&self, _: &Vec3) -> Vec3 {
        -self.normal
    }
}

//Ray
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

// Intersection
struct Intersection<'a> {
    distance: f32,
    element: &'a Element,
}

// impl<'a> Intersection<'a> {
//     fn new<'b>(distance: f32, element: &'b Element) -> Intersection<'b> {
//         Intersection {
//             distance,
//             element,
//         }
//     }
// }

//Light
pub struct Light {
    pub kind: LightKind,
    pub color: Color,
    pub intensity: f32,
}

pub enum LightKind {
    Ambient,
    Point {
        position: Vec3,
    },
    Directional {
        direction: Vec3,
    }
}


// 
// FUNCTIONS
// 
pub fn render(scene: &Scene) -> DynamicImage {
    let mut image = DynamicImage::new_rgb8(scene.width, scene.height);
    let background = image::Rgba::<u8>([40,40,60,0]);
    let recursion_depth = 1;

    for x in 0..scene.width {
        for y in 0..scene.height {
            //standard pixel background
            image.put_pixel(x, y, background);
            let ray = Ray::create_prime(x, y, &scene);
            let intersection = scene.trace(&ray);
            
            if let Some(inter) = intersection {
                let color = get_color(&scene, &ray, inter, recursion_depth);
                image.put_pixel(x, y, color.to_rgba());
            }
        };
    };
    image
}

fn get_color(scene: &Scene, ray: &Ray, intersection: Intersection, recursion_depth: u32) -> Color {
    let Intersection { element, distance } = intersection;
    let hit_point = ray.origin + (ray.direction * distance);
    let surface_normal = element.surface_normal(&hit_point);

    let mut color = element.color();

    let mut intensity = 0.0;
    for light in &scene.lights {
        color = color.multiply(light.color);
        let mut light_intensity = light.intensity;
        match light.kind {
            LightKind::Ambient => { intensity += light.intensity; }
            LightKind::Point { position } => { 
                let mut impact_to_light =  position - hit_point;
                let distance_sqared = impact_to_light.dot(impact_to_light);
                light_intensity = light_intensity / distance_sqared;
                impact_to_light = impact_to_light.normalize();

                let normal_dot_impact = surface_normal.dot(impact_to_light);

                //Shadow
                let shadow_ray = Ray {
                    origin: hit_point +  (surface_normal * 1e-4),
                    direction: impact_to_light,
                };
                let shadow_intersection = scene.trace(&shadow_ray);
                let in_light =  shadow_intersection.is_none() || shadow_intersection.unwrap().distance.powi(2) > distance_sqared;

                if normal_dot_impact > 0. && in_light {
                    // intensity += light_intensity * normal_dot_impact / (surface_normal.dot(surface_normal) * impact_vector.dot(impact_vector));
                    //old version
                    //Funktioniert weil Vektoren normalized
                    intensity += normal_dot_impact * light_intensity;
                };

                //Specular
                if element.specular() != -1. {
                    let light_exit = (2. * surface_normal * impact_to_light.dot(surface_normal) - impact_to_light).normalize();
                    let resamblence = light_exit.dot(-ray.direction);
                    if resamblence > 0. {
                        intensity += light_intensity * (resamblence * resamblence as f32).powf(element.specular());
                    };
                }

                

            }
            LightKind::Directional { direction } => {
                let impact_to_light = - direction;
                let normal_dot_impact = surface_normal.dot(impact_to_light);

                //Shadow
                let shadow_ray = Ray {
                    origin: hit_point +  (surface_normal * 1e-4),
                    direction: impact_to_light,
                };
                let in_light = scene.trace(&shadow_ray).is_none();

                if normal_dot_impact > 0. && in_light {
                    intensity += normal_dot_impact * light_intensity;
                };

                //Specular
                if element.specular() != -1. {
                    let light_exit = (2. * surface_normal * impact_to_light.dot(surface_normal) - impact_to_light).normalize();
                    let resamblence = light_exit.dot(-ray.direction.normalize());
                    if resamblence > 0. {
                        intensity += light_intensity * (resamblence * resamblence as f32).powf(element.specular());
                    };
                }
            }
        };
    }
    let light_reflected = element.albedo() / std::f32::consts::PI;
    color = color
        .multiply_scalar(light_reflected)
        .multiply_scalar(intensity)
        .clamp();

    //Reflection
    if recursion_depth > 0 {
        let ray_exit = (2. * surface_normal * ray.direction.dot(surface_normal) - ray.direction).normalize();
        let ray = Ray {
            origin: hit_point * (surface_normal * 1e-3),
            direction: - ray_exit
        };
        let intersection = scene.trace(&ray);
        
        if let Some(inter) = intersection {
            let reflection_color = get_color(&scene, &ray, inter, recursion_depth - 1).multiply_scalar(element.reflectivity());
            color = color.multiply_scalar(1. - element.reflectivity());
            color = color.add(reflection_color.multiply_scalar(element.reflectivity()));
        }
    }

    color
    
    // //TODO Understand formula. Albedo is parameter for how much light is reflected by this element


    // OLD
    // let direction_to_light = -lights.light.direction;
    // //Funktioniert weil Vektoren normalized
    // let light_power = surface_normal.dot(direction_to_light) * lights.light.intensity;
    // //TODO Understand formula. Albedo is parameter for how much light is reflected by this element
    // let light_reflected = element.albedo() / std::f32::consts::PI;

    // element.color()
    //     .multiply(lights.light.color)
    //     .multiply_scalar(light_reflected)
    //     .multiply_scalar(light_power)
    //     .clamp() 
}

pub fn to_radians(x: f32) -> f32 {
    x / 57.296
}
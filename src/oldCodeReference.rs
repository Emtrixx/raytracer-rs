use image::{DynamicImage, GenericImage};

#[derive(Debug, Clone, Copy)]
pub struct Color (pub u8, pub u8,pub u8);

impl Color {
    fn multiply_scalar(&self, scalar: f64) -> Color {
        let r = self.0 as f64 * scalar;
        let g = self.1 as f64 * scalar;
        let b = self.2 as f64 * scalar;
        Color (
            r as u8, 
            g as u8, 
            b as u8, 
        )
    }
}

#[derive(Debug, Clone)]
pub struct Sphere {
    pub center: Point,
    pub radius: f64,
    pub color: Color,
}

#[derive(Debug, Clone)]
pub enum LightKind {
    Ambient,
    Point {
        position: Point,
    },
    Directional {
        direction: Point,
    }
}

#[derive(Debug, Clone)]
pub struct Light {
    pub kind: LightKind,
    pub intensity: f64,
}

pub struct Scene {
    pub width: u32,
    pub height: u32,
    pub spheres: Vec<Sphere>,
    pub lights: Vec<Light>,
}

#[derive(Debug, Clone, Copy)]
pub struct Point {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Point {
    fn dot(&self, p2: &Point) -> f64 {
        self.x * p2.x + self.y * p2.y + self.z * p2.z
    }
    fn subtract(&self, p2: &Point) -> Point {
        Point {
            x: self.x - p2.x, 
            y: self.y - p2.y,
            z: self.z - p2.z,
        }
    }
    fn add(&self, p2: &Point) -> Point {
        Point {
            x: self.x + p2.x, 
            y: self.y + p2.y,
            z: self.z + p2.z,
        }
    }
    fn multiply(&self, p2: &Point) -> Point {
        Point {
            x: self.x * p2.x, 
            y: self.y * p2.y,
            z: self.z * p2.z,
        }
    }
    fn add_scalar(&self, scalar: f64) -> Point {
        Point {
            x: self.x + scalar, 
            y: self.y + scalar,
            z: self.z + scalar,
        }
    }
    fn divide_scalar(&self, scalar: f64) -> Point {
        Point {
            x: self.x / scalar, 
            y: self.y / scalar,
            z: self.z / scalar,
        }
    }
}


pub fn render(scene: &Scene) {
    let mut img = DynamicImage::new_rgb8(scene.width, scene.height);

    for x in 0..scene.width {
        for y in 0..scene.height {

            let d = canvas_to_viewport(x,y, &scene);

            let color = trace_ray(
                Point {x: 0., y: 0., z: 0.},
                d,
                10.,
                f64::INFINITY,
                &scene.spheres,
                &scene.lights,
            );

            let pixel: image::Rgba::<u8> = image::Rgba::<u8>([color.0,color.1,color.2,0],);

            img.put_pixel(x, y, pixel);
        }
    }

    img.save("test.png").unwrap();
}

fn canvas_to_viewport(x: u32, y: u32, scene: &Scene) -> Point {
    let x: f64 = x as f64 - (scene.width as f64 / 2.) ;
    let y: f64 = y as f64 - (scene.height as f64 / 2.) ;

    // x * viewport width / camera width
    // y * viewport height / camera height
    Point {
        x: x  * 1.,
        y: y  * 1.,
        z: 10.
    }
}

fn trace_ray(camera: Point, viewport: Point, t_min: f64, _t_max: f64, spheres: &Vec<Sphere>, lights: &Vec<Light>) -> Color {
    let mut closest_t = 1000.;
    let mut closest_sphere: Option<Sphere> = None;

    for sphere in spheres {
        let (t1, t2) = intersect_ray_sphere(camera, viewport, &sphere);
        if t1 > t_min && t1 < closest_t {
            closest_t = t1;
            closest_sphere = Some(sphere.clone());
        };
        if t2 > t_min && t2 < closest_t {
            closest_t = t2;
            closest_sphere = Some(sphere.clone());
        };
    }
    
    if let Some(x) = closest_sphere {
        let point = camera.add_scalar(closest_t).multiply(&viewport);
        let normal = point.subtract(&x.center);
        let normal = normal.divide_scalar(normal.dot(&normal));
        let color = x.color.multiply_scalar(compute_lighting(point, normal, lights));
        // println!("{:?}", color);
        color
    } else {
        Color (255,255,255)
    }
}

fn intersect_ray_sphere(camera: Point,  viewport: Point, sphere: &Sphere) -> (f64, f64) {
    let r = sphere.radius;

    let co = camera.subtract(&sphere.center);

    let a = viewport.dot(&viewport);
    let b = 2. * co.dot(&viewport);
    let c = co.dot(&co) - r*r;

    let discriminant = b*b - 4.*a*c;
    if discriminant < 0. {
        return (f64::INFINITY, f64::INFINITY);
    };

    let t1 = (-b + discriminant.sqrt()) / (2.*a);
    let t2 = (-b - discriminant.sqrt()) / (2.*a);
    return (t1, t2)
}

fn compute_lighting(point: Point, normal: Point, lights: &Vec<Light>) -> f64 {
    let mut intensity = 0.0;
    let mut impact_vector: Point = Point {x: 1., y: 1., z: 1.};
    for light in lights {
        match light.kind {
            LightKind::Ambient => { intensity += light.intensity; }
            _ => {
                match light.kind {
                    LightKind::Point { position } => { 
                        impact_vector = position.subtract(&point);
                    },
                    LightKind::Directional { direction } => {
                        impact_vector = direction;
                    },
                    _ => {}
                };
                let normal_dot_impact = normal.dot(&impact_vector);

                if normal_dot_impact > 0. {
                    intensity += light.intensity * normal_dot_impact / (normal.dot(&normal) * impact_vector.dot(&impact_vector));
                };
            }
        };
    }
    intensity
}

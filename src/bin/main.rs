use raytracer::*;
use glam::{Vec3};

fn main() {
    // Objects
    let sphere_red = Element::Sphere( Sphere {
        center: Vec3::new(-1.,-1.,-5.5),
        radius: 1.,
        material: Material {
            color: Color {
                red: 255.,
                green: 0.,
                blue: 0.
            },
            albedo: 1.,
            specular: 50., 
            reflectivity: 0.4,
        }
    });
    let sphere_green = Element::Sphere( Sphere {
        center: Vec3::new(1.,0.5,-2.6),
        radius: 1.,
        material: Material {
            color: Color {
                red: 0.,
                green: 255.,
                blue: 0.
            },
            albedo: 1.,
            specular: 50., 
            reflectivity: 0.4,
        } 
    });
    let sphere_blue = Element::Sphere( Sphere {
        center: Vec3::new(-4.,-2.,-7.5),
        radius: 1.,
        material: Material {
            color: Color {
                red: 0.,
                green: 0.,
                blue: 255.
            },
            albedo: 1., 
            specular: 10., 
            reflectivity: 0.4,
        } 
    });

    //Plane
    let plane = Element::Plane(Plane {
        origin: Vec3::new(0.,-4.,0.),
        normal: Vec3::new(0.,-1.,0.),
        material: Material {
            color: Color {
                red: 60.,
                green: 60.,
                blue: 60.
            },
            albedo: 1.,
            specular: -1.,
            reflectivity: 0.1, 
        }
    });

    // let spheres = vec![sphere_red, sphere_green, sphere_blue];
    let elements = vec![sphere_red, sphere_green, sphere_blue, plane];


    // Lights

    let brightness = 1.5;
    let ambient = Light {
        kind: LightKind::Ambient,
        intensity: 0.06 * brightness,
        color: Color {
            red: 1.,
            green: 1.,
            blue: 1.,
        }
    };

    let point = Light {
        kind: LightKind::Point {
            position: Vec3::new(-2., -1.,-4.5)
        },
        intensity: 6.2 * brightness,
        color: Color {
            red: 1.,
            green: 1.,
            blue: 1.,
        }
    };

    let directional = Light {
        kind: LightKind::Directional {
            direction: Vec3::new(0., -1., -2.).normalize(),
        },
        intensity: 0.8 * brightness,
        color: Color {
            red: 1.,
            green: 1.,
            blue: 1.,
        }
    };

    let lights = vec![ambient, directional, point];

    let scene = Scene {
        width: 800,
        height: 600,
        elements,
        fov: 60.,
        lights,
    };

    let img = render(&scene);

    img.save("test.png").unwrap();
}

use raytracer::{Sphere, Element, Scene, Color, Plane, Light, LightKind, render};
use glam::{Vec3};

fn main() {

    // Objects
    let sphere_red = Element::Sphere( Sphere {
        center: Vec3::new(0.,0.,-7.5),
        radius: 1.,
        color: Color {
            red: 255.,
            green: 0.,
            blue: 0.
        }, 
    });
    let sphere_green = Element::Sphere( Sphere {
        center: Vec3::new(1.,0.5,-8.6),
        radius: 1.,
        color: Color {
            red: 0.,
            green: 255.,
            blue: 0.
        }, 
    });
    let sphere_blue = Element::Sphere( Sphere {
        center: Vec3::new(-4.,-2.,-7.5),
        radius: 1.,
        color: Color {
            red: 0.,
            green: 0.,
            blue: 255.
        }, 
    });

    //Plane
    let plane = Element::Plane(Plane {
        origin: Vec3::new(0.,-4.,0.),
        normal: Vec3::new(0.,-1.,0.),
        color: Color {
            red: 60.,
            green: 60.,
            blue: 60.,
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
            position: Vec3::new(-6., 7.,-2.5)
        },
        intensity: 0.8 * brightness,
        color: Color {
            red: 1.,
            green: 1.,
            blue: 1.,
        }
    };

    let directional = Light {
        kind: LightKind::Directional {
            direction: Vec3::new(-0., -1., -0.).normalize(),
        },
        intensity: 0.58 * brightness,
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

    img.save("images/shadow_acne.png").unwrap();
}

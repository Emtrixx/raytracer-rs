use raytracer::{Sphere, Element, Scene, Color, render};
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
    let elements = vec![sphere_red, sphere_green, sphere_blue];

    // let spheres = vec![sphere_red, sphere_green, sphere_blue];

    // Lights

    // let ambient = Light {
    //     kind: LightKind::Ambient,
    //     intensity: 0.2,
    // };

    // let point = Light {
    //     kind: LightKind::Point {
    //         position: Point {
    //             x: 100.,
    //             y: -300.,
    //             z: 100.,
    //         }
    //     },
    //     intensity: 0.6
    // };

    // let directional = Light {
    //     kind: LightKind::Directional {
    //         direction: Point {
    //             x: 100.,
    //             y: 300.,
    //             z: 400.,
    //         }
    //     },
    //     intensity: 0.4
    // };

    // let lights = vec![ambient, directional];

    let scene = Scene {
        width: 800,
        height: 600,
        elements,
        fov: 60.
        // lights: lights,
    };

    let img = render(&scene);

    img.save("test.png").unwrap();
}

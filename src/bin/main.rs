use raytracer::{Sphere, Scene, Point, Color, Light, LightKind, render};

fn main() {

    // Objects
    let z = 90.;
    let sphere_red = Sphere {
        center: Point {
            x: 0.,
            y: 2000.,
            z: 200.,
        },
        radius: 2000.,
        color: Color (255, 0, 0), 
    };
    
    let sphere_green = Sphere {
        center: Point {
            x: -100.,
            y: 45.,
            z,
        },
        radius: 80.,
        color: Color (0, 255, 0), 
    };

    let sphere_blue = Sphere {
        center: Point {
            x: 100.,
            y: 45.,
            z,
        },
        radius: 80.,
        color: Color (0, 0, 255), 
    };

    let spheres = vec![sphere_red, sphere_green, sphere_blue];

    // Lights

    let ambient = Light {
        kind: LightKind::Ambient,
        intensity: 0.2,
    };

    let point = Light {
        kind: LightKind::Point {
            position: Point {
                x: 100.,
                y: -300.,
                z: 100.,
            }
        },
        intensity: 0.6
    };

    let directional = Light {
        kind: LightKind::Directional {
            direction: Point {
                x: 100.,
                y: 300.,
                z: 400.,
            }
        },
        intensity: 0.4
    };

    let lights = vec![ambient, directional];

    let scene = Scene {
        width: 1920,
        height: 1080,
        spheres: spheres,
        lights: lights,
    };

    render(&scene);
}

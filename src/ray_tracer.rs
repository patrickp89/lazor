use crate::vector_arithmetic::*;
use std::cmp;
use web_sys::console;

pub fn render(
    spheres: &[Sphere],
    planes: &[Plane],
    light: &Light,
    canvas: &web_sys::HtmlCanvasElement,
) {
    console::log_1(&"Rendering the scene...".into());

    let huge_value = 1000000.0;
    let tiny_value = 0.1;

    let height: u32 = canvas.height();
    let width: u32 = canvas.width();

    console::log_2(&"Canvas height: ".into(), &height.into());
    console::log_2(&"Canvas width: ".into(), &width.into());

    let camera_direction = Vector3 {
        x: 0.0,
        y: 0.0,
        z: 1.0,
    };
    let camera_up = Vector3 {
        x: 0.0,
        y: 1.0,
        z: 0.0,
    };
    let camera_right = cross_product(&camera_direction, &camera_up);

    for y in 0..height {
        for x in 00..width {
            let normalized_x: f64 = ((x / width) as f64) - 0.5;
            let normalized_y: f64 = ((y / height) as f64) - 0.5;

            let t1 = scale_vector(normalized_x, &camera_right);
            let t2 = scale_vector(normalized_y, &camera_up);
            let t3 = sum(&t1, &t2);

            let ray_direction = sum(&t3, &camera_direction);
            let r = Ray {
                origin: Vector3 {
                    x: 0.0,
                    y: 1.0,
                    z: 0.0,
                },
                direction: ray_direction,
            };
            let c = trace_ray(&r, 1);
            set_pixel_color(x, y, &c);
        }
    }
}

fn trace_ray(r: &Ray, depth: u32) -> Color {
    // TODO: ...
    return scale_color(2, &Color { r: 0, g: 0, b: 0 });
}

fn scale_color(a: i64, c: &Color) -> Color {
    // TODO: the min-max magic should be a function of its own!
    let red = cmp::min(cmp::max(c.r * a, 0), 255);
    let green = cmp::min(cmp::max(c.g * a, 0), 255);
    let blue = cmp::min(cmp::max(c.b * a, 0), 255);
    return Color {
        r: red,
        g: green,
        b: blue,
    };
}

fn set_pixel_color(x: u32, y: u32, c: &Color) {
    // TODO: ...
}

pub struct Sphere {
    pub pos: Vector3,
    pub r: i64,
    pub color: Color,
    pub reflect: bool,
}

pub struct Plane {
    pub n: Vector3,
    pub d: i64,
    pub color: Color,
    pub reflect: bool,
}

pub struct Color {
    pub r: i64,
    pub g: i64,
    pub b: i64,
}

pub struct Light {
    pub pos: Vector3,
}

pub struct Ray {
    pub origin: Vector3,
    pub direction: Vector3,
}

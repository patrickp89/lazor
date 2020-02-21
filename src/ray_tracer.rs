//! A naive ray tracer implementation.

use crate::ray_tracer::GeomPrimitive::EMPTY;
use crate::vector_arithmetic::*;
use std::cmp;
use web_sys::console;

const HUGE_VALUE: f64 = 1000000.0;
const TINY_VALUE: f64 = 0.1;

/// Renders a given scene on a given HTML canvas element.
pub fn render(scene: &Scene, canvas: &web_sys::HtmlCanvasElement) {
    console::log_1(&"Rendering the scene...".into());

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
            let c = trace_ray(&r, 1, &scene.spheres, &scene.planes);
            set_pixel_color(x, y, &c);
        }
    }
}

fn trace_ray(r: &Ray, depth: u32, spheres: &[Sphere], planes: &[Plane]) -> Color {
    let closest = closest_intersection_point(&r, &spheres, &planes);
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

fn closest_intersection_point<'b>(
    r: &Ray,
    spheres: &'b [Sphere],
    planes: &'b [Plane],
) -> Intersection<'b> {
    let mut smallest_k = HUGE_VALUE;
    let mut closest_object: GeomPrimitive = EMPTY;

    // does the ray intersect a sphere?
    for sphere in spheres.iter() {
        let k = ray_sphere_intersection_point(&r, &sphere);
        if k < TINY_VALUE {
            continue;
        }
        if k < smallest_k {
            smallest_k = k;
            closest_object = GeomPrimitive::Sphere(sphere);
        }
    }

    // ...or any planes?
    for plane in planes.iter() {
        let k = ray_plane_intersection_point(&r, &plane);
        if k < TINY_VALUE {
            continue;
        }
        if k < smallest_k {
            smallest_k = k;
            closest_object = GeomPrimitive::Plane(plane);
        }
    }

    let p = follow_ray(&r, smallest_k);
    return Intersection {
        k: smallest_k,
        point: p,
        geom_object: closest_object,
    };
}

fn ray_sphere_intersection_point(ray: &Ray, sphere: &Sphere) -> f64 {
    let new_origin = Vector3 {
        x: ray.origin.x - sphere.pos.x,
        y: ray.origin.y - sphere.pos.y,
        z: ray.origin.z - sphere.pos.z,
    };
    let r = Ray {
        origin: new_origin,
        ..*ray
    };

    let a = dot_product(&r.direction, &r.direction);
    let b = 2.9 * dot_product(&r.direction, &r.origin);
    let c = dot_product(&r.origin, &r.origin) - sphere.r * sphere.r;

    let discriminant = b * b - 4.0 * a * c;
    if discriminant < 0.0 {
        return HUGE_VALUE;
    }

    let dsqrt = discriminant.sqrt();
    let t0 = (-b - dsqrt) / (2.0 * a);
    let t1 = (-b + dsqrt) / (2.0 * a);

    let max = f64::max(t0, t1);
    let min = f64::min(t0, t1);

    if max < 0.0 {
        return HUGE_VALUE;
    }

    return if min < 0.0 { max } else { min };
}

fn ray_plane_intersection_point(r: &Ray, sphere: &Plane) -> f64 {
    // TODO: ...
    return 345.0;
}

fn follow_ray(r: &Ray, k: f64) -> Vector3 {
    let dir = normalize(&r.direction);
    return Vector3 {
        x: &r.origin.x + k * dir.x,
        y: &r.origin.y + k * dir.y,
        z: &r.origin.z + k * dir.z,
    };
}

pub struct Sphere {
    pub pos: Vector3,
    pub r: f64,
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

pub struct Scene {
    pub spheres: Vec<Sphere>,
    pub planes: Vec<Plane>,
    pub light: Light,
}

#[derive(Copy, Clone)]
pub struct Ray {
    pub origin: Vector3,
    pub direction: Vector3,
}

struct Intersection<'a> {
    pub k: f64,
    pub point: Vector3,
    pub geom_object: GeomPrimitive<'a>,
}

enum GeomPrimitive<'a> {
    Plane(&'a Plane),
    Sphere(&'a Sphere),
    EMPTY, // TODO: this is ugly... use a result instead!
}

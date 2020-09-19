//! A naive ray tracer implementation.

use crate::ray_tracer::GeomPrimitive::UNKNOWN;
use crate::vector_arithmetic::*;
use std::cmp;
use wasm_bindgen::{Clamped, JsCast, JsValue};
use web_sys::{console, CanvasRenderingContext2d, HtmlCanvasElement, ImageData};

const HUGE_VALUE: f64 = 1000000.0;
const TINY_VALUE: f64 = 0.1;

/// Renders a given scene on a given HTML canvas element.
pub fn render(scene: &Scene, canvas: &HtmlCanvasElement) -> Result<(), JsValue> {
    console::log_1(&"Rendering the scene...".into());

    let height: u32 = canvas.height();
    let width: u32 = canvas.width();

    let ctx: CanvasRenderingContext2d = canvas
        .get_context("2d")
        .unwrap()
        .unwrap()
        .dyn_into::<CanvasRenderingContext2d>()
        .unwrap();

    let image_data: ImageData = ctx
        .create_image_data_with_sw_and_sh(width as f64, height as f64)
        .unwrap()
        .dyn_into::<ImageData>()
        .unwrap();

    let mut updated_data = render_with_data_array(&scene, &image_data.data(), width, height);

    let updated_image_data =
        ImageData::new_with_u8_clamped_array_and_sh(Clamped(&mut updated_data), width, height)
            .unwrap();
    return ctx.put_image_data(&updated_image_data, 0.0, 0.0);
}

fn render_with_data_array(
    scene: &Scene,
    imagedata_data: &Clamped<Vec<u8>>,
    width: u32,
    height: u32,
) -> Clamped<Vec<u8>> {
    let mut data: Clamped<Vec<u8>> = imagedata_data.clone();

    // TODO: the camera direction should be part of the scene struct?
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
        for x in 0..width {
            let c = compute_and_trace_ray(
                x,
                y,
                width,
                height,
                &camera_direction,
                &camera_up,
                &camera_right,
                &scene,
            );
            set_pixel_color(&mut data, width, x, y, &c);
        }
    }

    return data;
}

fn compute_and_trace_ray(
    x: u32,
    y: u32,
    width: u32,
    height: u32,
    camera_direction: &Vector3,
    camera_up: &Vector3,
    camera_right: &Vector3,
    scene: &Scene,
) -> Color {
    let r = compute_ray(
        x,
        y,
        width,
        height,
        &camera_direction,
        &camera_up,
        &camera_right,
    );
    return trace_ray(&r, 1, &scene.spheres, &scene.planes, &scene.light);
}

fn compute_ray(
    x: u32,
    y: u32,
    width: u32,
    height: u32,
    camera_direction: &Vector3,
    camera_up: &Vector3,
    camera_right: &Vector3,
) -> Ray {
    let p = normalize_x_y(x, y, width, height);
    let t1 = scale_vector(p.x, &camera_right);
    let t2 = scale_vector(p.y, &camera_up);
    let t3 = sum(&t1, &t2);

    let ray_direction = sum(&t3, &camera_direction);
    return Ray {
        origin: Vector3 {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        },
        direction: ray_direction,
    };
}

fn normalize_x_y(x: u32, y: u32, width: u32, height: u32) -> Point {
    let normalized_x: f64 = ((x as f64) / (width as f64)) - 0.5;
    let normalized_y: f64 = ((y as f64) / (height as f64)) - 0.5;
    return Point {
        x: normalized_x,
        y: normalized_y,
    };
}

fn trace_ray(r: &Ray, depth: u32, spheres: &[Sphere], planes: &[Plane], light: &Light) -> Color {
    // compute the closest point that our ray intersects:
    let closest_point = closest_intersection_point(&r, &spheres, &planes);
    let intersection_point = &closest_point.point;
    let closest_object = &closest_point.geom_object;

    let a: f64 = &closest_point.k - HUGE_VALUE;
    return if a.abs() < TINY_VALUE {
        Color { r: 0, g: 0, b: 0 }
    } else {
        let normal = closest_object.compute_normal(&intersection_point);
        let direction_to_light = normalize(&difference(&light.pos, &intersection_point));
        let shadow_ray = Ray {
            origin: *intersection_point,
            direction: direction_to_light,
        };

        // compute the closest point in the direction of light:
        let closest_point_l = closest_intersection_point(&shadow_ray, &spheres, &planes);

        // if there is no intersection or the closest one is behind the light source:
        let b: f64 = closest_point_l.k - HUGE_VALUE;
        let d = distance(&intersection_point, &light.pos);
        let raw_intensity = if b.abs() < TINY_VALUE || d < closest_point_l.k {
            dot_product(&normal, &direction_to_light)
        } else {
            0.0
        };

        let ambient_intensity = f64::max(raw_intensity, 0.2);
        let ambient_color = scale_color(ambient_intensity, &closest_object.color());

        if depth > 0 && closest_object.reflects() {
            let q = 2.0 * dot_product(&r.direction, &normal);
            let p = scale_vector(q, &normal);
            let reflection_vector = difference(&r.direction, &p);
            let reflection_ray = Ray {
                origin: *intersection_point,
                direction: reflection_vector,
            };
            let reflected_color = trace_ray(&reflection_ray, depth - 1, spheres, planes, light);
            let mixed_color = mix_colors(0.25, &reflected_color, 0.75, &ambient_color);
            mixed_color
        } else {
            ambient_color
        }
    };
}

fn scale_color(a: f64, c: &Color) -> Color {
    // RGB values are integers:
    let scaled_red = ((c.r as f64) * a).round() as u8;
    let scaled_green = ((c.g as f64) * a).round() as u8;
    let scaled_blue = ((c.b as f64) * a).round() as u8;

    let red = cmp::min(cmp::max(scaled_red, 0), 255);
    let green = cmp::min(cmp::max(scaled_green, 0), 255);
    let blue = cmp::min(cmp::max(scaled_blue, 0), 255);

    return Color {
        r: red,
        g: green,
        b: blue,
    };
}

fn mix_colors(a: f64, c1: &Color, b: f64, c2: &Color) -> Color {
    let red = (a * (c1.r as f64) + b * (c2.r as f64)).round() as u8;
    let green = (a * (c1.g as f64) + b * (c2.g as f64)).round() as u8;
    let blue = (a * (c1.b as f64) + b * (c2.b as f64)).round() as u8;

    return Color {
        r: red,
        g: green,
        b: blue,
    };
}

fn set_pixel_color(data: &mut Clamped<Vec<u8>>, width: u32, x: u32, y: u32, c: &Color) {
    let i = compute_image_data_array_index(width, x, y);
    data[i as usize] = c.r;
    data[(i + 1) as usize] = c.g;
    data[(i + 2) as usize] = c.b;
    data[(i + 3) as usize] = 255;
}

/// Computes the index of pixel (x,y) in the image data's underlying array.
fn compute_image_data_array_index(width: u32, x: u32, y: u32) -> u32 {
    return (x + y * width) * 4;
}

fn closest_intersection_point<'b>(
    r: &Ray,
    spheres: &'b [Sphere],
    planes: &'b [Plane],
) -> Intersection<'b> {
    let mut smallest_k = HUGE_VALUE;
    let mut closest_object: GeomPrimitive = UNKNOWN;

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
    let b = 2.0 * dot_product(&r.direction, &r.origin);
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

fn ray_plane_intersection_point(r: &Ray, plane: &Plane) -> f64 {
    let d = dot_product(&r.direction, &plane.n);

    return if d.abs() < TINY_VALUE {
        HUGE_VALUE
    } else {
        (-plane.d - dot_product(&r.origin, &plane.n)) / d
    };
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
    pub d: f64,
    pub color: Color,
    pub reflect: bool,
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

pub struct Light {
    pub pos: Vector3,
}

pub struct Scene {
    pub spheres: Vec<Sphere>,
    pub planes: Vec<Plane>,
    pub light: Light,
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Ray {
    pub origin: Vector3,
    pub direction: Vector3,
}

struct Intersection<'a> {
    pub k: f64,
    pub point: Vector3,
    pub geom_object: GeomPrimitive<'a>,
}

struct Point {
    pub x: f64,
    pub y: f64,
}

enum GeomPrimitive<'a> {
    Plane(&'a Plane),
    Sphere(&'a Sphere),
    UNKNOWN,
}

impl GeomPrimitive<'_> {
    fn compute_normal(&self, point: &Vector3) -> Vector3 {
        let v: Vector3 = match self {
            GeomPrimitive::Plane(plane) => plane.n,
            GeomPrimitive::Sphere(sphere) => difference(&point, &sphere.pos),
            GeomPrimitive::UNKNOWN => panic!("Unknown geom. primitive!"),
        };
        return normalize(&v);
    }

    fn reflects(&self) -> bool {
        return match self {
            GeomPrimitive::Plane(plane) => plane.reflect,
            GeomPrimitive::Sphere(sphere) => sphere.reflect,
            GeomPrimitive::UNKNOWN => panic!("Unknown geom. primitive!"),
        };
    }

    fn color(&self) -> Color {
        return match self {
            GeomPrimitive::Plane(plane) => plane.color.clone(), // TODO: should work without the clone trait, see compute_normal()!
            GeomPrimitive::Sphere(sphere) => sphere.color.clone(), // TODO: should work without the clone trait, see compute_normal()!
            GeomPrimitive::UNKNOWN => panic!("Unknown geom. primitive!"),
        };
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // TODO: the camera direction should be part of the scene struct?
    const TEST_CAMERA_DIRECTION: Vector3 = Vector3 {
        x: 0.0,
        y: 0.0,
        z: 1.0,
    };

    const TEST_CAMERA_UP: Vector3 = Vector3 {
        x: 0.0,
        y: 1.0,
        z: 0.0,
    };

    const ZERO_VECTOR3: Vector3 = Vector3 {
        x: 0.0,
        y: 0.0,
        z: 0.0,
    };

    #[test]
    fn test_normalize_x_y() {
        let width = 12;
        let height = 12;

        let p1 = normalize_x_y(0, 0, width, height);
        assert_eq!(p1.x, -0.5);
        assert_eq!(p1.y, -0.5);

        let p2 = normalize_x_y(6, 9, width, height);
        assert_eq!(p2.x, 0.0);
        assert_eq!(p2.y, 0.25);
    }

    #[test]
    fn test_compute_ray() {
        let camera_right = cross_product(&TEST_CAMERA_DIRECTION, &TEST_CAMERA_UP);
        let width = 12;
        let height = 12;

        // compute some rays:
        let r1 = compute_ray(
            0,
            0,
            width,
            height,
            &TEST_CAMERA_DIRECTION,
            &TEST_CAMERA_UP,
            &camera_right,
        );
        let er1 = Ray {
            origin: ZERO_VECTOR3,
            direction: Vector3 {
                x: 0.5,
                y: -0.5,
                z: 1.0,
            },
        };
        assert_eq!(r1, er1);

        let r2 = compute_ray(
            6,
            9,
            width,
            height,
            &TEST_CAMERA_DIRECTION,
            &TEST_CAMERA_UP,
            &camera_right,
        );
        let er2 = Ray {
            origin: ZERO_VECTOR3,
            direction: Vector3 {
                x: 0.0,
                y: 0.25,
                z: 1.0,
            },
        };
        assert_eq!(r2, er2);
    }

    #[test]
    fn test_compute_and_trace_ray() {
        let camera_right = cross_product(&TEST_CAMERA_DIRECTION, &TEST_CAMERA_UP);
        let width = 12;
        let height = 12;

        // a test scene:
        let scene = create_small_test_scene();

        // trace some rays:
        let c1 = compute_and_trace_ray(
            0,
            0,
            width,
            height,
            &TEST_CAMERA_DIRECTION,
            &TEST_CAMERA_UP,
            &camera_right,
            &scene,
        );
        assert_eq!(c1, Color { r: 0, g: 0, b: 0 });

        let c2 = compute_and_trace_ray(
            6,
            9,
            width,
            height,
            &TEST_CAMERA_DIRECTION,
            &TEST_CAMERA_UP,
            &camera_right,
            &scene,
        );
        assert_eq!(
            c2,
            Color {
                r: 148,
                g: 148,
                b: 148,
            }
        );

        let c3 = compute_and_trace_ray(
            11,
            11,
            width,
            height,
            &TEST_CAMERA_DIRECTION,
            &TEST_CAMERA_UP,
            &camera_right,
            &scene,
        );
        assert_eq!(
            c3,
            Color {
                r: 112,
                g: 112,
                b: 112,
            }
        );
    }

    fn create_small_test_scene() -> Scene {
        let test_spheres = vec![Sphere {
            pos: Vector3 {
                x: -50.0,
                y: -40.0,
                z: 250.0,
            },
            r: 25.0,
            color: Color { r: 255, g: 0, b: 0 },
            reflect: true,
        }];

        let test_planes = vec![Plane {
            n: Vector3 {
                x: 0.0,
                y: -1.0,
                z: 0.0,
            },
            d: 60.0,
            color: Color {
                r: 200,
                g: 200,
                b: 200,
            },
            reflect: false,
        }];

        let light1 = Light {
            pos: Vector3 {
                x: 0.0,
                y: 0.0,
                z: 180.0,
            },
        };

        return Scene {
            spheres: test_spheres,
            planes: test_planes,
            light: light1,
        };
    }
}

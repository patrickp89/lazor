//! lazor's library file that exposes the WebAssembly entry point.

mod ray_tracer;
mod utils;
mod vector_arithmetic;

use crate::ray_tracer::*;
use crate::vector_arithmetic::*;

use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::console;

#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

/// Renders a test scene.
#[wasm_bindgen]
pub fn render_scene() {
    console::log_1(&"Creating spheres, planes etc. for the scene...".into());
    let scene = create_test_scene();

    console::log_1(&"Looking up the canvas DOM object...".into());
    let document = web_sys::window().unwrap().document().unwrap();
    let canvas: web_sys::HtmlCanvasElement = document
        .get_element_by_id("result_canvas")
        .unwrap()
        .dyn_into::<web_sys::HtmlCanvasElement>()
        .map_err(|_| ())
        .unwrap();

    console::log_1(&"Okay, done. Calling ray_tracer::render()...".into());
    ray_tracer::render(&scene, &canvas);

    console::log_1(&"Done!".into());
}

/// A test scene.
pub fn create_test_scene() -> Scene {
    // all spheres that should be drawn:
    let sphere1 = Sphere {
        pos: Vector3 {
            x: -50.0,
            y: -40.0,
            z: 250.0,
        },
        r: 25.0,
        color: Color { r: 255, g: 0, b: 0 },
        reflect: true,
    };
    let sphere2 = Sphere {
        pos: Vector3 {
            x: -50.0,
            y: -40.0,
            z: 250.0,
        },
        r: 25.0,
        color: Color { r: 255, g: 0, b: 0 },
        reflect: true,
    };
    let test_spheres = vec![sphere1, sphere2];

    // ...and all the planes:
    let plane1 = Plane {
        n: Vector3 {
            x: 0.0,
            y: -1.0,
            z: 0.0,
        },
        d: 25,
        color: Color { r: 255, g: 0, b: 0 },
        reflect: true,
    };
    let plane2 = Plane {
        n: Vector3 {
            x: 0.0,
            y: -1.0,
            z: 0.0,
        },
        d: 25,
        color: Color { r: 255, g: 0, b: 0 },
        reflect: true,
    };
    let test_planes = vec![plane1, plane2];

    // a light source:
    let light1 = Light {
        pos: Vector3 {
            x: -50.0,
            y: -40.0,
            z: 250.0,
        },
    };

    return Scene {
        spheres: test_spheres,
        planes: test_planes,
        light: light1,
    };
}

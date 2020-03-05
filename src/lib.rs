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
    let image_data = ray_tracer::render(&scene, &canvas);
    match image_data {
        Ok(_) => console::log_1(&"Done!".into()),
        Err(e) => console::log_2(&"An error occurred: ".into(), &e),
    }
}

/// A test scene.
fn create_test_scene() -> Scene {
    // spheres and planes:
    let test_spheres = create_test_spheres();
    let test_planes = create_test_planes();

    // a light source:
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

/// Some test spheres.
fn create_test_spheres() -> Vec<Sphere> {
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
            x: 35.0,
            y: -40.0,
            z: 300.0,
        },
        r: 50.0,
        color: Color {
            r: 255,
            g: 255,
            b: 255,
        },
        reflect: true,
    };

    let sphere3 = Sphere {
        pos: Vector3 {
            x: -40.0,
            y: 30.0,
            z: 300.0,
        },
        r: 25.0,
        color: Color {
            r: 50,
            g: 50,
            b: 255,
        },
        reflect: true,
    };

    let sphere4 = Sphere {
        pos: Vector3 {
            x: 50.0,
            y: 30.0,
            z: 200.0,
        },
        r: 30.0,
        color: Color { r: 0, g: 255, b: 0 },
        reflect: true,
    };

    return vec![sphere1, sphere2, sphere3, sphere4];
}

/// Some test planes.
fn create_test_planes() -> Vec<Plane> {
    let light_grey = Color {
        r: 200,
        g: 200,
        b: 200,
    };

    let plane1 = Plane {
        n: Vector3 {
            x: 0.0,
            y: -1.0,
            z: 0.0,
        },
        d: 60.0,
        color: light_grey,
        reflect: false,
    };

    let plane2 = Plane {
        n: Vector3 {
            x: 0.0,
            y: 0.0,
            z: -1.0,
        },
        d: 400.0,
        color: light_grey,
        reflect: false,
    };

    let plane3 = Plane {
        n: Vector3 {
            x: 1.0,
            y: 0.0,
            z: 0.0,
        },
        d: 110.0,
        color: light_grey,
        reflect: false,
    };

    let plane4 = Plane {
        n: Vector3 {
            x: -1.0,
            y: 0.0,
            z: 0.0,
        },
        d: 120.0,
        color: light_grey,
        reflect: false,
    };

    let plane5 = Plane {
        n: Vector3 {
            x: 0.0,
            y: 1.0,
            z: 0.0,
        },
        d: 110.0,
        color: light_grey,
        reflect: false,
    };

    let plane6 = Plane {
        n: Vector3 {
            x: 0.0,
            y: 0.0,
            z: 1.0,
        },
        d: 5.0,
        color: light_grey,
        reflect: false,
    };

    return vec![plane1, plane2, plane3, plane4, plane5, plane6];
}

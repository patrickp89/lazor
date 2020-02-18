mod ray_tracer;
mod utils;

use crate::ray_tracer::*;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::console;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
pub fn render_scene() {
    console::log_1(&"Creating spheres, planes etc. for the scene...".into());

    // all spheres that should be drawn:
    let sphere1 = Sphere {
        pos: Vector3 {
            x: -50,
            y: -40,
            z: 250,
        },
        r: 25,
        color: Color { r: 255, g: 0, b: 0 },
        reflect: true,
    };
    let sphere2 = Sphere {
        pos: Vector3 {
            x: -50,
            y: -40,
            z: 250,
        },
        r: 25,
        color: Color { r: 255, g: 0, b: 0 },
        reflect: true,
    };
    let spheres = [sphere1, sphere2];

    // ...and all the planes:
    let plane1 = Plane {
        n: Vector3 { x: 0, y: -1, z: 0 },
        d: 25,
        color: Color { r: 255, g: 0, b: 0 },
        reflect: true,
    };
    let plane2 = Plane {
        n: Vector3 { x: 0, y: -1, z: 0 },
        d: 25,
        color: Color { r: 255, g: 0, b: 0 },
        reflect: true,
    };
    let planes = [plane1, plane2];

    // a light source:
    let light1 = Light {
        pos: Vector3 {
            x: -50,
            y: -40,
            z: 250,
        },
    };

    console::log_1(&"Looking up the canvas DOM object...".into());
    let document = web_sys::window().unwrap().document().unwrap();
    let canvas: web_sys::HtmlCanvasElement = document
        .get_element_by_id("result_canvas")
        .unwrap()
        .dyn_into::<web_sys::HtmlCanvasElement>()
        .map_err(|_| ())
        .unwrap();

    console::log_1(&"Okay, done. Calling ray_tracer::render()...".into());

    // trace 'em:
    ray_tracer::render(&spheres, &planes, &light1, &canvas);
    console::log_1(&"Done!".into());
}

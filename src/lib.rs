mod utils;
mod ray_tracer;

use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
pub fn render_scene() {
    // all spheres that should be drawn:
    let sphere1 = ray_tracer::Sphere {
        pos: ray_tracer::Vector3{ x: -50, y: -40, z: 250 },
        r: 25,
        color: ray_tracer::Color{r: 255, g: 0, b: 0 },
        reflect: true
    };
    let sphere2 = ray_tracer::Sphere {
        pos: ray_tracer::Vector3{ x: -50, y: -40, z: 250 },
        r: 25,
        color: ray_tracer::Color{r: 255, g: 0, b: 0 },
        reflect: true
    };
    let spheres = [sphere1, sphere2];

    // ...and all the planes:
    let plane1 = ray_tracer::Plane {
        n: ray_tracer::Vector3{ x: 0, y: -1, z: 0 },
        d: 25,
        color: ray_tracer::Color{r: 255, g: 0, b: 0 },
        reflect: true
    };
    let plane2 = ray_tracer::Plane {
        n: ray_tracer::Vector3{ x: 0, y: -1, z: 0 },
        d: 25,
        color: ray_tracer::Color{r: 255, g: 0, b: 0 },
        reflect: true
    };
    let planes = [plane1, plane2];

    // a light source:
    let light1 = ray_tracer::Light {
        pos: ray_tracer::Vector3{ x: -50, y: -40, z: 250 }
    };

    // trace 'em:
    ray_tracer::render();
}

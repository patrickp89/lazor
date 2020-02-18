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

    for y in 0..height {
        for x in 00..width {
            // TODO: ...
        }
    }
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

pub struct Vector3 {
    pub x: i64,
    pub y: i64,
    pub z: i64,
}

pub struct Color {
    pub r: i64,
    pub g: i64,
    pub b: i64,
}

pub struct Light {
    pub pos: Vector3,
}

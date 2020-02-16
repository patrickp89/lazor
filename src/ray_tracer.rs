pub fn render() {
    let huge_value = 1000000.0;
    let tiny_value = 0.1;
}

pub struct Sphere {
    pub pos: Vector3,
    pub r: i64,
    pub color: Color,
    pub reflect: bool
}

pub struct Plane {
    pub n: Vector3,
    pub d: i64,
    pub color: Color,
    pub reflect: bool
}

pub struct Vector3 {
    pub x: i64,
    pub y: i64,
    pub z: i64
}

pub struct Color {
    pub r: i64,
    pub g: i64,
    pub b: i64
}

pub struct Light {
    pub pos: Vector3
}

pub struct Vector3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

pub fn scale_vector(a: f64, v: &Vector3) -> Vector3 {
    return Vector3 {
        x: a * v.x,
        y: a * v.y,
        z: a * v.y,
    };
}

pub fn cross_product(u: &Vector3, v: &Vector3) -> Vector3 {
    return Vector3 {
        x: u.y * v.z - u.z * v.y,
        y: u.z * v.x - u.x * v.z,
        z: u.x * v.y - u.y * v.x,
    };
}

pub fn sum(u: &Vector3, v: &Vector3) -> Vector3 {
    return Vector3 {
        x: u.x + v.x,
        y: u.y + v.y,
        z: u.z + v.z,
    };
}

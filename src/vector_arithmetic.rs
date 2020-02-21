//! A collection of simple vector arithmetic functions.

/// An euclidean vector.
#[derive(Copy, Clone)]
pub struct Vector3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

/// Scale a vector v by a scalar a.
pub fn scale_vector(a: f64, v: &Vector3) -> Vector3 {
    return Vector3 {
        x: a * v.x,
        y: a * v.y,
        z: a * v.y,
    };
}

/// Computes the cross product of two given vectors u and v.
pub fn cross_product(u: &Vector3, v: &Vector3) -> Vector3 {
    return Vector3 {
        x: u.y * v.z - u.z * v.y,
        y: u.z * v.x - u.x * v.z,
        z: u.x * v.y - u.y * v.x,
    };
}

/// Computes the dot product of two given vectors u and v.
pub fn dot_product(u: &Vector3, v: &Vector3) -> f64 {
    return u.x * v.x + u.y * v.y + u.z * v.z;
}

/// Computes the sum of two given vectors u and v.
pub fn sum(u: &Vector3, v: &Vector3) -> Vector3 {
    return Vector3 {
        x: u.x + v.x,
        y: u.y + v.y,
        z: u.z + v.z,
    };
}

/// Normalizes a given vector.
pub fn normalize(v: &Vector3) -> Vector3 {
    let l = length(v);
    return Vector3 {
        x: v.x / l,
        y: v.y / l,
        z: v.z / l,
    };
}

/// Computes the length of a given vector.
pub fn length(v: &Vector3) -> f64 {
    let s = v.x * v.x + v.y * v.y + v.z * v.z;
    return s.sqrt();
}

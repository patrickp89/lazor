//! A collection of simple vector arithmetic functions.

/// An euclidean vector.
#[derive(Copy, Clone, Debug, PartialEq)]
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
        z: a * v.z,
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

/// Computes the difference of two given vectors u and v.
pub fn difference(u: &Vector3, v: &Vector3) -> Vector3 {
    return Vector3 {
        x: u.x - v.x,
        y: u.y - v.y,
        z: u.z - v.z,
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

/// Computes the distance between two vectors u and v.
pub fn distance(u: &Vector3, v: &Vector3) -> f64 {
    return length(&difference(&u, &v));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sum() {
        let u = Vector3 {
            x: 1.0,
            y: 2.0,
            z: 3.0,
        };
        let v = Vector3 {
            x: 4.0,
            y: 5.0,
            z: 6.0,
        };
        let w = sum(&u, &v);
        let e = Vector3 {
            x: 5.0,
            y: 7.0,
            z: 9.0,
        };
        assert_eq!(w, e);
    }

    #[test]
    fn test_diff() {
        let u = Vector3 {
            x: 1.0,
            y: 2.0,
            z: 3.0,
        };
        let v = Vector3 {
            x: 4.0,
            y: 5.0,
            z: 6.0,
        };
        let w = difference(&u, &v);
        let e = Vector3 {
            x: -3.0,
            y: -3.0,
            z: -3.0,
        };
        assert_eq!(w, e);
    }

    #[test]
    fn test_scaling() {
        let u = Vector3 {
            x: 1.0,
            y: 2.0,
            z: 3.0,
        };
        let w = scale_vector(4.0, &u);
        let e = Vector3 {
            x: 4.0,
            y: 8.0,
            z: 12.0,
        };
        assert_eq!(w, e);
    }

    #[test]
    fn test_cross_product() {
        let u = Vector3 {
            x: 1.0,
            y: 2.0,
            z: 3.0,
        };
        let v = Vector3 {
            x: 4.0,
            y: 5.0,
            z: 6.0,
        };
        let w = cross_product(&u, &v);
        let e = Vector3 {
            x: -3.0,
            y: 6.0,
            z: -3.0,
        };
        assert_eq!(w, e);
    }

    #[test]
    fn test_dot_product() {
        let u = Vector3 {
            x: 1.0,
            y: 2.0,
            z: 3.0,
        };
        let v = Vector3 {
            x: 4.0,
            y: 5.0,
            z: 6.0,
        };
        let p = dot_product(&u, &v);
        assert_eq!(p, 32.0);
    }

    #[test]
    fn test_normalizing() {
        let v = Vector3 {
            x: 4.0,
            y: 5.0,
            z: 6.0,
        };
        let w = normalize(&v);

        let ex = 0.4558423058385518;
        let ey = 0.5698028822981898;
        let ez = 0.6837634587578276;
        let e = Vector3 {
            x: ex,
            y: ey,
            z: ez,
        };

        assert_eq!(w, e);
    }

    #[test]
    fn test_length() {
        let v = Vector3 {
            x: 4.0,
            y: 5.0,
            z: 6.0,
        };
        let l = length(&v);

        let sqrt77 = 8.774964387392123;
        assert_eq!(l, sqrt77);
    }
}

#[cfg(test)]
use assert_approx_eq::assert_approx_eq;

struct Point<T> {
    x: T,
    y: T,
}

impl Point<f32> {
    fn distance_from_origin(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}

#[test]
fn test_distance_from_origin() {
    let point = Point { x: 22.1, y: 22.3 };
    assert_approx_eq!(point.distance_from_origin(), 31.39, 0.01f32)
}

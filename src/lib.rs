pub struct Circle {
    x: f64,
    y: f64,
    radius: f64
}

impl Circle {
    pub fn area(&self) -> f64 {
        std::f64::consts::PI * (self.radius * self.radius)
    }

    pub fn new(x: f64, y: f64, radius: f64) -> Circle {
        Circle {
            x: x,
            y: y,
            radius: radius
        }
    }
}

fn add_three(x: int) -> int { x + 3 }

fn times_four(x: int) -> int { x * 4 }

pub fn add_one(x: &int) -> int { *x + 1 }

pub fn add_three_times_four(x: int) -> int {
    times_four(add_three(x))
}

#[cfg(test)]
mod test {
    use super::add_three;
    use super::times_four;
    use super::add_one;

    #[test]
    fn test_add_three() {
        let result = add_three(5i);
        assert_eq!(8i, result);
    }

    #[test]
    fn test_times_four() {
        let result = times_four(5i);
        assert_eq!(20i, result);
    }

    #[test]
    fn test_add_one() {
        let result = add_one(&100i);
        assert_eq!(101i, result);
    }
}

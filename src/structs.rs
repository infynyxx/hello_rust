// https://doc.rust-lang.org/book/ch05-03-method-syntax.html
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other: Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }

    fn square(size: u32) -> Rectangle {
        Rectangle {
            width: size,
            height: size
        }
    }
}

#[test]
fn test_area() {
    let rect = Rectangle {
        width: 200,
        height: 100
    };
    assert_eq!(rect.area(), 20000);
}

#[test]
fn test_can_hold_true() {
    let rect = Rectangle{
        width: 200, height: 100
    };
    let other = Rectangle {
        width: 22,
        height: 99
    };
    assert_eq!(rect.can_hold(other), true)
}

#[test]
fn test_can_hold_false() {
    let other = Rectangle{
        width: 200, height: 100
    };
    let rect = Rectangle {
        width: 22,
        height: 99
    };
    assert_eq!(rect.can_hold(other), false)
}

#[test]
fn test_square() {
    assert_eq!(Rectangle::square(10).area(), 100)
}
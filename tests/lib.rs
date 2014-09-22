extern crate hello_rust;
use hello_rust::add_three_times_four;
use hello_rust::Circle;

#[test]
fn math_checks_out() {
    let result = add_three_times_four(5i);
    assert_eq!(32i, result);
}

#[test]
fn test_circle_area() {
    let c = Circle::new(0.0, 0.0, 2.0);
    let expected = 12.566372;
    assert!(expected > c.area());
}

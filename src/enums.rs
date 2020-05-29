#[derive(Debug)]
enum IpAddress {
    V4(u8, u8, u8, u8),
    V6(String),
}

impl IpAddress {
    fn call(&self) {
        //self.V4
    }
}

#[test]
fn test_V4() {
    let x = IpAddress::V4(127, 0, 0, 1);
    let tuple = match x {
        IpAddress::V4(value1, value2, value3, value4) => Some((value1, value2, value3, value4)),
        _ => None,
    };
    assert_eq!(tuple, Some((127, 0, 0, 1)))
}

#[test]
fn test_V6() {
    let x = IpAddress::V6(String::from("::1"));
    let matched = match x {
        IpAddress::V6(value) => Some(value),
        _ => None,
    };
    assert_eq!(matched, Some(String::from("::1")))
}

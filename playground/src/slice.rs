// https://doc.rust-lang.org/book/ch04-03-slices.html
pub fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();
    for (i, &ch) in bytes.iter().enumerate() {
        if ch == b' ' {
            return &s[0..i];
        }
    }
    &s[..]
}

pub fn slices(s: String) -> (String, String) {
    //let s = String::from("hello world");
    println!("{}", s.len());

    let slice1: &str = &s[0..2]; // same as &s[..2]
    let slice2: &str = &s[6..s.len()]; // same as &s[2..]

    println!("slice1={:?}, slice2={:?}", slice1, slice2);
    return (slice1.to_owned(), slice2.to_owned());
}

#[test]
fn test_first_word_with_space() {
    assert_eq!(first_word(&String::from("hello world")).to_owned(), "hello");
}

#[test]
fn test_first_word_without_space() {
    assert_eq!(
        first_word(&String::from("hellO_world")).to_owned(),
        "hellO_world"
    );
}

#[test]
fn test_slices() {
    assert_eq!(
        slices(String::from("hello world")),
        (String::from("he"), String::from("world"))
    )
}

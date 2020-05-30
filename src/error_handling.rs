use std::fs::File;
use std::io;
use std::io::Read;

fn read_from_file(file_name: &String) -> Result<String, io::Error> {
    let file_path = File::open(file_name);
    let mut f = match file_path {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    let mut s = String::new();
    match f.read_to_string(&mut s) {
        Ok(_) => Ok(s),
        Err(e) => Err(e),
    }
}

#[test]
fn test_read_from_file_non_existent() {
    let file_name = String::from("hello_world.txt");
    let rs = read_from_file(&file_name);
    assert_eq!(rs.is_ok(), false, "{}", rs.unwrap_err().to_string());
}

#[test]
fn test_read_from_file() {
    use std::io::{self, Write};
    use tempfile::tempdir;
    let dir = tempdir().unwrap();

    let file_path = dir.path().join("hello_world.txt");
    let mut temp_file = File::create(&file_path).unwrap();

    let file_content = String::from("Hello Rust!");
    writeln!(temp_file, "{}", file_content);

    //let file_path_str = file_path.display().to_string();

    let rs = read_from_file(&file_path.display().to_string());
    assert_eq!(rs.unwrap().trim(), file_content);
}
use std::fs::File;
use std::io::Read;
#[derive(Debug)]
enum UserFile {
    Real(File),
    Test(String),
}
fn open_file() -> UserFile {
    if cfg!(test) {
        UserFile::Test(String::from("Just a test file"))
    } else {
        UserFile::Real(File::open("src/main.rs").unwrap())
    }
}
fn get_file_content() -> String {
    let mut content = String::new();
    let file = open_file();
    match file {
        UserFile::Real(mut f) => {
            f.read_to_string(&mut content).unwrap();
            content
        }
        UserFile::Test(s) => s,
    }
}
#[test]
fn test_file() {
    let content = get_file_content();
    println!("Content is: {content}");
    assert_eq!(content, "Just a test file");
}
fn main() {
    let content = get_file_content();
    println!("{content}");
}

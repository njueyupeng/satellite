use satellite;
use std::fs;
use std::path::Path;

fn getTleSuites() {
    let path = Path::new("./tests/tle.txt");
    let contents = fs::read_to_string(path).expect("Unable to read file");
    let lines: Vec<&str> = contents.lines().collect();
    println!("{}", lines.len())
    // let a  =s.
}

#[test]
pub fn sgp4catalog() {
    getTleSuites()
}
extern crate reqwest;

#[macro_use]
extern crate lazy_static;

use std::fs::File;
use std::io::Read;

pub const RUST_WWW_CODE_OF_CONDUCT: &str =
    "https://raw.githubusercontent.com/rust-lang/rust-www/master/en-US/conduct.md";

lazy_static! {
    pub static ref BASE: String = fetch(RUST_WWW_CODE_OF_CONDUCT);
}

pub fn fetch(url: &str) -> String {
    let mut resp = reqwest::get(url).unwrap();
    assert!(resp.status().is_success());
    resp.text().unwrap()
}

pub fn read_expected(filename: &str) -> String {
    let mut f = File::open(filename).expect("file not found");

    let mut contents = String::new();
    f.read_to_string(&mut contents)
        .expect("something went wrong reading the file");
    contents
}

pub fn make_expected_satellite() -> String {
    String::from("foo")
}

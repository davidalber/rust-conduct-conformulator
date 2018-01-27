extern crate reqwest;

#[macro_use]
extern crate lazy_static;

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

pub fn make_expected_satellite() -> String {
    String::from("foo")
}

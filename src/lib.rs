#![allow(dead_code)]

extern crate reqwest;

const RUST_WWW_CODE_OF_CONDUCT: &str =
    "https://raw.githubusercontent.com/rust-lang/rust-www/master/en-US/conduct.md";

fn fetch(url: &str) -> String {
    let mut resp = reqwest::get(url).unwrap();
    assert!(resp.status().is_success());
    resp.text().unwrap()
}

pub struct CodeOfConduct<'a> {
    url: &'a str,
}

impl<'a> CodeOfConduct<'a> {
    pub fn get_base_code_of_conduct() -> CodeOfConduct<'a> {
        CodeOfConduct {
            url: RUST_WWW_CODE_OF_CONDUCT,
        }
    }

    pub fn fetch(&self) -> String {
        fetch(self.url)
    }
}

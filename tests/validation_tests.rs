extern crate code_of_conduct_conformulator;

#[macro_use]
extern crate lazy_static;

use code_of_conduct_conformulator::{fetch, make_expected_satellite, BASE, RUST_WWW_CODE_OF_CONDUCT};

use std::fs::File;
use std::io::Read;

lazy_static! {
    static ref EXPECTED_SATELLITE: String = make_expected_satellite();
}

fn read_file(filename: &str) -> String {
    let mut f = File::open(filename).expect("file not found");

    let mut contents = String::new();
    f.read_to_string(&mut contents)
        .expect("something went wrong reading the file");
    contents
}

#[test]
fn validate_base_file() {
    let expected_base = read_file("tests/expected/base.md");

    assert!(
        expected_base == *BASE,
        format!(
            "Rust WWW code of conduct ({}) does not match expected value",
            RUST_WWW_CODE_OF_CONDUCT
        )
    );
}

// Test that satellite_code_of_conduct.md (in this repository) matches the
// current EXPECTED_SATELLITE.
#[test]
fn validate_local_satellite_file() {
    const LOCAL_SATELLITE: &str = "satellite_code_of_conduct.md";
    let local_satellite = read_file(LOCAL_SATELLITE);

    assert!(
        local_satellite == *EXPECTED_SATELLITE,
        format!(
            "The local satellite file ({}) does not match expected value",
            LOCAL_SATELLITE
        )
    );
}

#[test]
fn validate_satellite_files() {
    let urls = vec![
        "https://raw.githubusercontent.com/rust-lang/rust/master/CODE_OF_CONDUCT.md",
        "https://raw.githubusercontent.com/rust-lang-nursery/rustfmt/master/CODE_OF_CONDUCT.md"
    ];

    let failing_urls: Vec<&str> = urls.iter()
        .filter(|u| fetch(u) != *EXPECTED_SATELLITE)
        .map(|u| *u)
        .collect();

    assert!(
        failing_urls.len() == 0,
        format!("Satellite checks failed for {}", failing_urls.join(", "))
    );
}

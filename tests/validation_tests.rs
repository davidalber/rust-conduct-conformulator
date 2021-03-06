extern crate code_of_conduct_conformulator;

use code_of_conduct_conformulator::{
    check_repository_conformance, ConductStatus, BASE, EXPECTED_SATELLITE, RUST_WWW_CODE_OF_CONDUCT,
};

use std::fs::File;
use std::io::Read;

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
    let repos = vec![
        "rust-lang/rust",
        "rust-lang/rust-by-example",
        "rust-lang-nursery/highfive",
        "rust-lang-nursery/rust-clippy",
        "rust-lang-nursery/rustfmt",
    ];
    let conformance = check_repository_conformance(&repos.iter().map(|u| u.to_string()).collect());
    let failing_urls: Vec<&str> = conformance
        .repositories
        .iter()
        .filter(|r| r.code_of_conduct.status != ConductStatus::Correct)
        .map(|r| r.code_of_conduct.url.as_ref().unwrap())
        .map(AsRef::as_ref)
        .collect();

    assert!(
        failing_urls.len() == 0,
        format!("Satellite checks failed for {}", failing_urls.join(", "))
    );
}

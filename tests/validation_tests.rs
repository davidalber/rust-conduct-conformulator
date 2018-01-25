extern crate code_of_conduct_conformulator;

#[macro_use]
extern crate lazy_static;

use code_of_conduct_conformulator::{fetch, make_expected_satellite, read_expected, BASE,
                                    RUST_WWW_CODE_OF_CONDUCT};

lazy_static! {
    static ref EXPECTED_SATELLITE: String = make_expected_satellite();
}

#[test]
fn validate_base_file() {
    let expected_base = read_expected("tests/expected/base.md");

    assert!(
        expected_base == *BASE,
        format!(
            "Rust WWW code of conduct ({}) does not match expected value",
            RUST_WWW_CODE_OF_CONDUCT
        )
    );
}

fn satellite_matches_expected(url: &str) -> bool {
    let satellite = fetch(url);
    if satellite != *EXPECTED_SATELLITE {
        println!(
            "Satellite code of conduct ({}) does not match expected value",
            url
        );
        return false;
    }

    true
}

#[test]
fn validate_satellite_files() {
    let urls = vec![
        "https://raw.githubusercontent.com/rust-lang/rust/master/CODE_OF_CONDUCT.md",
    ];

    let results: Vec<bool> = urls.iter()
        .map(|&u| satellite_matches_expected(u))
        .collect();
    assert!(
        results.iter().fold(true, |acc, &b| acc && b),
        "Satellite checks failed"
    );
}

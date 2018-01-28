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
    assert!(
        resp.status().is_success(),
        format!("Could not load {}", url)
    );
    resp.text().unwrap()
}

// This is a brittle function to convert the Rust WWW code of conduct into the
// code of conduct that should be used in satellite projects.
pub fn make_expected_satellite() -> String {
    let mut expected: Vec<&str> = BASE.split("\n")
        .skip_while(|l| !l.starts_with("#"))
        .map(|l| {
            if l.starts_with("[mod_team]") {
                "[mod_team]: https://www.rust-lang.org/team.html#Moderation-team"
            } else {
                l
            }
        })
        .collect();
    expected.insert(2, "");
    expected.insert(
        2,
        "A version of this document [can be found online](https://www.rust-lang.org/conduct.html).",
    );

    expected.join("\n")
}

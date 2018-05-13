extern crate reqwest;
extern crate serde;

#[macro_use]
extern crate lazy_static;

#[macro_use]
extern crate serde_derive;

use std::time::SystemTime;

pub const RUST_WWW_CODE_OF_CONDUCT: &str =
    "https://raw.githubusercontent.com/rust-lang/rust-www/master/en-US/conduct.md";

lazy_static! {
    pub static ref BASE: String = fetch(RUST_WWW_CODE_OF_CONDUCT).unwrap();
}
lazy_static! {
    pub static ref EXPECTED_SATELLITE: String = make_expected_satellite();
}

pub fn fetch(url: &str) -> Result<String, reqwest::StatusCode> {
    let mut resp = reqwest::get(url).unwrap();
    if !resp.status().is_success() {
        return Err(resp.status());
    }
    assert!(
        resp.status().is_success(),
        format!("Could not fetch {}", url)
    );
    let body = resp.text().unwrap();
    Ok(body)
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

pub fn get_org_repositories(org: &str) -> Vec<String> {
    let urlify = |o| format!("https://api.github.com/orgs/{}/repos", o);
    let json: Vec<Repository> = reqwest::get(&urlify(org)).unwrap().json().unwrap();
    json.into_iter()
        .map(|r| format!("{}/{}", org, r.name))
        .collect()
}

#[derive(Serialize, Deserialize, PartialEq)]
pub enum ConductStatus {
    Correct,
    Incorrect,
    Missing,
    Unknown,
}

#[derive(Serialize, Deserialize)]
pub struct CodeOfConductStatus {
    pub status: ConductStatus,
    pub url: String,
}

impl CodeOfConductStatus {
    fn new(status: ConductStatus, url: String) -> CodeOfConductStatus {
        CodeOfConductStatus { status, url }
    }
}

#[derive(Serialize, Deserialize)]
pub struct ProjectRepository {
    pub code_of_conduct: CodeOfConductStatus,
    pub name: String,
}

impl ProjectRepository {
    fn new(conduct_status: ConductStatus, conduct_url: String, name: String) -> ProjectRepository {
        ProjectRepository {
            code_of_conduct: CodeOfConductStatus::new(conduct_status, conduct_url),
            name,
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct ConformanceReport {
    pub repositories: Vec<ProjectRepository>,
    created_on: u64,
}

impl ConformanceReport {
    fn new(repositories: Vec<ProjectRepository>) -> ConformanceReport {
        ConformanceReport {
            repositories,
            created_on: SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs(),
        }
    }
}

pub fn check_repository_conformance(repos: &Vec<String>) -> ConformanceReport {
    let urlify = |r| {
        format!(
            "https://raw.githubusercontent.com/{}/master/CODE_OF_CONDUCT.md",
            r
        )
    };

    let repositories_conformance = repos
        .iter()
        .map(|r| (r, urlify(r)))
        .map(|(r, u)| match fetch(&u) {
            Err(e) => match e {
                reqwest::StatusCode::NotFound => (r, u, ConductStatus::Missing),
                _ => (r, u, ConductStatus::Unknown),
            },
            Ok(t) => match t == *EXPECTED_SATELLITE {
                true => (r, u, ConductStatus::Correct),
                false => (r, u, ConductStatus::Incorrect),
            },
        })
        .map(|(r, u, s)| ProjectRepository::new(s, u, r.to_string()))
        .collect();
    ConformanceReport::new(repositories_conformance)
}

#[derive(Debug, Deserialize)]
pub struct Repository {
    name: String,
}

extern crate config;
extern crate rayon;
extern crate reqwest;
extern crate serde;

#[macro_use]
extern crate lazy_static;

#[macro_use]
extern crate serde_derive;

use reqwest::header::{qitem, Accept, Authorization};
use rayon::prelude::*;
use std::time::SystemTime;

pub const RUST_WWW_CODE_OF_CONDUCT: &str =
    "https://raw.githubusercontent.com/rust-lang/rust-www/master/en-US/conduct.md";

lazy_static! {
    pub static ref BASE: String = fetch_raw(RUST_WWW_CODE_OF_CONDUCT, None).unwrap();
    pub static ref EXPECTED_SATELLITE: String = make_expected_satellite();
    pub static ref GITHUB_KEY: Option<String> = get_api_key();
}

fn get_api_key() -> Option<String> {
    let mut settings = config::Config::default();
    settings.merge(config::File::with_name("config")).is_ok();
    let api_key = settings.get_str("github-api-key").ok();

    match api_key {
        Some(k) => Some(format!("token {}", k)),
        None => None,
    }
}

pub fn fetch(
    url: &str,
    accept_header: Option<&str>,
) -> Result<reqwest::Response, reqwest::StatusCode> {
    let client = reqwest::Client::new();
    let mut request_builder = client.get(url);
    if let Some(accept_header) = accept_header {
        request_builder.header(Accept(vec![qitem(accept_header.parse().unwrap())]));
    }
    if let Some(ref api_key) = *GITHUB_KEY {
        request_builder.header(Authorization(api_key.to_owned()));
    }

    let resp = request_builder.send().unwrap();
    if !resp.status().is_success() {
        return Err(resp.status());
    }
    assert!(
        resp.status().is_success(),
        format!("Could not fetch {}", url)
    );
    Ok(resp)
}

pub fn fetch_raw(url: &str, accept_header: Option<&str>) -> Result<String, reqwest::StatusCode> {
    let resp = fetch(url, accept_header);
    match resp {
        Ok(mut r) => {
            let body = r.text().unwrap();
            Ok(body)
        }
        Err(e) => Err(e),
    }
}

pub fn fetch_json<T>(url: &str, accept_header: Option<&str>) -> Result<T, reqwest::StatusCode>
where
    for<'de> T: serde::Deserialize<'de>,
{
    let resp = fetch(url, accept_header);
    match resp {
        Ok(mut r) => {
            let ret: T = r.json().unwrap();
            Ok(ret)
        }
        Err(e) => Err(e),
    }
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
    let json: Vec<Repository> = fetch_json(&urlify(org), None).unwrap();
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
    pub url: Option<String>,
}

impl CodeOfConductStatus {
    fn new(status: ConductStatus, url: Option<String>) -> CodeOfConductStatus {
        CodeOfConductStatus { status, url }
    }
}

#[derive(Serialize, Deserialize)]
pub struct ProjectRepository {
    pub code_of_conduct: CodeOfConductStatus,
    pub name: String,
}

impl ProjectRepository {
    fn new(
        conduct_status: ConductStatus,
        conduct_url: Option<String>,
        name: String,
    ) -> ProjectRepository {
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

#[derive(Debug, Deserialize)]
struct CommunityReport {
    health_percentage: u8,
}

fn get_repo_community_report(repo: &str) -> Result<Option<CommunityReport>, reqwest::StatusCode> {
    let url = format!("https://api.github.com/repos/{}/community/profile", repo);
    let cr: Result<CommunityReport, reqwest::StatusCode> = fetch_json(
        &url,
        Some("application/vnd.github.black-panther-preview+json"),
    );
    match cr {
        Err(e) => match e {
            reqwest::StatusCode::NotFound => Ok(None),
            _ => Err(e),
        },
        Ok(s) => Ok(Some(s)),
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
        .par_iter()
        .map(|r| (r, urlify(r)))
        .map(|(r, u)| match fetch_raw(&u, None) {
            Err(e) => match e {
                reqwest::StatusCode::NotFound => (r, None, ConductStatus::Missing),
                _ => (r, None, ConductStatus::Unknown),
            },
            Ok(t) => match t == *EXPECTED_SATELLITE {
                true => (r, Some(u), ConductStatus::Correct),
                false => (r, Some(u), ConductStatus::Incorrect),
            },
        })
        .map(|(r, u, s)| ProjectRepository::new(s, u, r.to_string()))
        .collect();

    let cr: Vec<Option<CommunityReport>> = repos
        .par_iter()
        .map(|r| get_repo_community_report(r).unwrap())
        .collect();
    println!("{:?}", cr);
    ConformanceReport::new(repositories_conformance)
}

#[derive(Debug, Deserialize)]
pub struct Repository {
    name: String,
}

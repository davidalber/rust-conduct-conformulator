#![feature(plugin)]
#![plugin(rocket_codegen)]

extern crate code_of_conduct_conformulator;
extern crate rocket;
extern crate rocket_contrib;
extern crate serde_json;

use code_of_conduct_conformulator::{check_repository_conformance, get_org_repositories,
                                    make_expected_satellite, ConformanceReport};
use rocket_contrib::Json;

pub const ORGANIZATIONS: [&str; 3] = ["rust-lang", "rust-lang-nursery", "rust-lang-deprecated"];

fn get_all_repos() -> Vec<String> {
    let mut repos: Vec<String> = Vec::new();
    for org in ORGANIZATIONS.iter() {
        repos.append(&mut get_org_repositories(org));
    }

    repos
}

#[get("/conduct")]
fn conduct() -> String {
    make_expected_satellite()
}

#[get("/conformance")]
fn conformance() -> Json<ConformanceReport> {
    let repos = get_all_repos();
    Json(check_repository_conformance(&repos))
}

fn main() {
    rocket::ignite()
        .mount("/", routes![conduct, conformance])
        .launch();
}

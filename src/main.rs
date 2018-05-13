extern crate code_of_conduct_conformulator;
extern crate serde_json;

use code_of_conduct_conformulator::{check_repository_conformance, get_org_repositories,
                                    make_expected_satellite};
use serde_json::Error;
use std::env;

pub const ORGANIZATIONS: [&str; 2] = ["rust-lang", "rust-lang-nursery"];

fn get_all_repos() -> Vec<String> {
    let mut repos: Vec<String> = Vec::new();
    for org in ORGANIZATIONS.iter() {
        repos.append(&mut get_org_repositories(org));
    }

    repos
}

fn main() -> Result<(), Error> {
    let args: Vec<String> = env::args().skip(1).collect();
    if args.len() > 0 {
        match args[0].as_ref() {
            "conduct" => print!("{}", make_expected_satellite()),
            "conformance" => {
                let repos = get_all_repos();
                let c = check_repository_conformance(&repos);
                println!("{}", serde_json::to_string(&c)?);
            }
            _ => eprintln!("Task argument (conduct | conformance) required."),
        }
    } else {
        eprintln!("Task argument (conduct | conformance) required.");
    }

    Ok(())
}

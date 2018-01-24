extern crate code_of_conduct_conformulator;

use code_of_conduct_conformulator::CodeOfConduct;

use std::fs::File;
use std::io::Read;

fn read_expected(filename: &str) -> String {
    let mut f = File::open(filename).expect("file not found");

    let mut contents = String::new();
    f.read_to_string(&mut contents)
        .expect("something went wrong reading the file");
    contents
}

#[test]
fn validate_base_file() {
    let base = CodeOfConduct::get_base_code_of_conduct().fetch();
    let expected_base = read_expected("tests/expected/base.md");
    assert_eq!(base, expected_base);
}

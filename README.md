# rust-conduct-conformulator [![Build Status](https://api.travis-ci.org/davidalber/rust-conduct-conformulator.svg)](https://travis-ci.org/davidalber/rust-conduct-conformulator)

The Rust Code of Conduct is checked in to multiple repositories. This is great since it makes the Code of Conduct more ubiquitous. The downside is that copies of the Code of Conduct can become out of sync over time.

This project aims to address the risk of fragmentation by providing cross-repository integration tests that verify the [Rust site Code of Conduct](https://www.rust-lang.org/en-US/conduct.html) and copies of the Code of Conduct in code repositories are synchronized. A passing build badge above indicates that the codes of conduct were in sync as of the last build. Builds occur at least daily.

These tests are intended to cover projects in the [rust-lang](https://github.com/rust-lang), [rust-lang-nursery](https://github.com/rust-lang-nursery/), and [rust-lang-deprecated](https://github.com/rust-lang-deprecated) GitHub organizations. Currently, the following code repositories are covered by the tests.

- [rust-lang/rust](https://github.com/rust-lang/rust) ([code of conduct](https://github.com/rust-lang/rust/blob/master/CODE_OF_CONDUCT.md))

## Onboarding Repositories
To add a new repository to the tests, either contact me or author a PR that adds the GitHub raw file URL to the `urls` vector in the `validate_satellite_files` function in [validation_tests.rs](https://github.com/davidalber/rust-conduct-conformulator/blob/master/tests/validation_tests.rs). Be sure to run `cargo test` before creating the PR.

## Adding the Rust Code of Conduct to a Project
This project also provides the current correct copy of the Code of Conduct for a code repository. Doing `cargo run` generates the file and prints it to stdout.

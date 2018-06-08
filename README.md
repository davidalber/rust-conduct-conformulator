# rust-conduct-conformulator [![Build Status](https://api.travis-ci.org/davidalber/rust-conduct-conformulator.svg)](https://travis-ci.org/davidalber/rust-conduct-conformulator)

The Rust Code of Conduct is checked in to multiple repositories. This is great since it makes the Code of Conduct more ubiquitous. The downside is that copies of the Code of Conduct can become out of sync over time.

This project aims to address the risk of fragmentation two ways:
1. (`cargo test`) The repository contains cross-repository integration tests that verify the [Rust site Code of Conduct](https://www.rust-lang.org/en-US/conduct.html) and copies of the Code of Conduct in specified code repositories are synchronized. A passing build badge above indicates that the codes of conduct were in sync as of the last build. Builds occur at least daily.
1. (`cargo run`) The repository launches a web service that shows the status of the codes of conduct in all respositories in three of Rust's GitHub organziations.

## Test Details
The integration tests are intended to cover projects in the [rust-lang](https://github.com/rust-lang), [rust-lang-nursery](https://github.com/rust-lang-nursery/), and [rust-lang-deprecated](https://github.com/rust-lang-deprecated) GitHub organizations. Currently, the following code repositories are covered by the tests.

- [rust-lang/rust](https://github.com/rust-lang/rust) ([code of conduct](https://github.com/rust-lang/rust/blob/master/CODE_OF_CONDUCT.md))
- [rust-lang/rust-by-example](https://github.com/rust-lang/rust-by-example) ([code of conduct](https://github.com/rust-lang/rust-by-example/blob/master/CODE_OF_CONDUCT.md))
- [rust-lang-nursery/highfive](https://github.com/rust-lang-nursery/highfive) ([code of conduct](https://github.com/rust-lang-nursery/highfive/blob/master/CODE_OF_CONDUCT.md))
- [rust-lang-nursery/rustfmt](https://github.com/rust-lang-nursery/rustfmt) ([code of conduct](https://github.com/rust-lang-nursery/rustfmt/blob/master/CODE_OF_CONDUCT.md))

### Onboarding Repositories
1. To add a new repository to the tests, either contact me or author a PR that adds the repository name to the `repos` vector in the `validate_satellite_files` function in [validation_tests.rs](https://github.com/davidalber/rust-conduct-conformulator/blob/master/tests/validation_tests.rs). Be sure to run `cargo test` before creating the PR.
1. Add a link to the repository and its code of conduct in the section above.

## Service Details
The service uses [Rocket](https://rocket.rs/), so you need to use nightly Rust. Currently, do
```
rustup override set nightly-2018-06-06
```

By default, the service runs on port 8000.
- http://localhost:8000/ returns a page with code of conduct status for all repositories in the [rust-lang](https://github.com/rust-lang), [rust-lang-nursery](https://github.com/rust-lang-nursery/), and [rust-lang-deprecated](https://github.com/rust-lang-deprecated) GitHub organizations.
- http://localhost:8000/conduct returns a page containing the current correct code of conduct for installation in repositories. This is used to produce [satellite_code_of_conduct.md](satellite_code_of_conduct.md).
- http://localhost:8000/conformance returns a JSON payload with code of conduct conformance information. That data is used to produce the index page.

## Adding the Rust Code of Conduct to a Project
The current correct copy of the Code of Conduct for a code repository is in [satellite_code_of_conduct.md](satellite_code_of_conduct.md). If you add that to your repository be sure to [onboard the repository](#onboarding-repositories).

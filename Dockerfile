FROM ubuntu:18.04

RUN apt-get update
RUN apt-get install -y curl \
                       gcc \
		       libssl-dev \
		       pkg-config
RUN curl https://sh.rustup.rs -sSf | sh -s -- -y
ENV PATH="/root/.cargo/bin:${PATH}"
RUN rustup default nightly-2018-06-24

RUN USER=root cargo new --bin /conformulator
WORKDIR /conformulator

COPY Cargo.* config.toml /conformulator/

RUN cargo build --release
RUN rm -rf src target

COPY assets /conformulator/assets
COPY src /conformulator/src
COPY templates /conformulator/templates

RUN cargo build --release

EXPOSE 80

ENV ROCKET_ENV prod
CMD "/conformulator/target/release/code_of_conduct_conformulator"

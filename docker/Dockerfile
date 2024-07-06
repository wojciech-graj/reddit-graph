FROM rust:1.77-slim-bookworm

RUN apt-get update \
 && apt-get upgrade -qq \
 && apt-get install -qq \
 build-essential \
 pgformatter \
 postgresql \
 sudo

RUN rustup component add clippy \
 && rustup component add rustfmt

RUN useradd -ms /bin/bash -G sudo docker \
 && echo 'docker ALL=NOPASSWD: ALL' | EDITOR='tee -a' visudo \
 && mkdir -p /docker \
 && chown -R docker /docker

USER docker
WORKDIR /docker

RUN cargo install cornucopia \
 && cargo install refinery_cli --no-default-features --features "postgresql"

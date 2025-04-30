FROM rust:1.85.1 as build-env
LABEL maintainer="yanorei32"

SHELL ["/bin/bash", "-o", "pipefail", "-c"]

WORKDIR /usr/src
RUN cargo new discardd-rs
COPY LICENSE Cargo.toml Cargo.lock /usr/src/discardd-rs/
WORKDIR /usr/src/discardd-rs
ENV CARGO_REGISTRIES_CRATES_IO_PROTOCOL=sparse
RUN	cargo install cargo-license && cargo license \
	--authors \
	--do-not-bundle \
	--avoid-dev-deps \
	--avoid-build-deps \
	--filter-platform "$(rustc -vV | sed -n 's|host: ||p')" \
	> CREDITS

RUN cargo build --release
COPY src/ /usr/src/discardd-rs/src/

RUN touch src/* && cargo build --release

FROM debian:bookworm-slim@sha256:e9ac68ffde903b241342267a51cd74c5417414af652cb2e380c6ddcf522589bc

WORKDIR /

COPY --chown=root:root --from=build-env \
	/usr/src/discardd-rs/CREDITS \
	/usr/src/discardd-rs/LICENSE \
	/usr/share/licenses/discardd-rs/

COPY --chown=root:root --from=build-env \
	/usr/src/discardd-rs/target/release/discardd-rs \
	/usr/bin/discardd-rs

CMD ["/usr/bin/discardd-rs"]

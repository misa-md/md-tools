# docker build --rm=true -t genshen/md-tools

FROM alpine:latest as linux-musl-builder

LABEL maintainer="genshen genshenchu@gmail.com" \
      description="Build static linked rust application with using musl-libc and libc++."

# setup rust and cargo
RUN wget https://static.rust-lang.org/dist/rust-1.36.0-x86_64-unknown-linux-musl.tar.gz \
    && tar -C /tmp -zxf rust-1.36.0-x86_64-unknown-linux-musl.tar.gz \
    && cd /tmp/rust-1.36.0-x86_64-unknown-linux-musl \
    && sed -i "s|#!/bin/bash|#!/bin/sh|g" ./install.sh \
    && ./install.sh \
    && rm -rf /tmp/rust-1.36.0-x86_64-unknown-linux-musl /tmp/rust-1.36.0-x86_64-unknown-linux-musl.tar.gz 

RUN adduser -D rust \
    && echo "rust ALL=(ALL) NOPASSWD:ALL" >> /etc/sudoers \
    && mkdir -p /home/rust/.cargo/git /home/rust/.cargo/registry /home/rust/project \
    && chown -R rust: /home/rust/.cargo \
    && chown -R rust: /home/rust/project \
    && apk add --no-cache gcc libgcc musl musl-dev sudo

USER rust
WORKDIR /home/rust/project

# start building
FROM linux-musl-builder as md-tools-builder
COPY --chown=rust ./ /home/rust/project/
COPY --chown=rust cargo-config.x86_64-unknown-linux-musl /home/rust/.cargo/config
RUN cargo build --release


FROM alpine:latest
COPY --chown=root:root --from=md-tools-builder /home/rust/project/target/x86_64-unknown-linux-musl/release/md-tools /usr/local/bin/
ENTRYPOINT ["md-tools"]
CMD ["--help"]

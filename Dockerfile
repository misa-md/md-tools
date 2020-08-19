# docker build --rm=true -t genshen/md-tools

FROM alpine:3.12 as linux-musl-builder

LABEL maintainer="genshen genshenchu@gmail.com" \
      description="Build static linked rust application with using musl-libc and libc++."

# setup rust and cargo
RUN wget https://static.rust-lang.org/dist/rust-1.45.2-x86_64-unknown-linux-musl.tar.gz \
    && tar -C /tmp -zxf rust-1.45.2-x86_64-unknown-linux-musl.tar.gz \
    && cd /tmp/rust-1.45.2-x86_64-unknown-linux-musl \
    && sed -i "s|#!/bin/bash|#!/bin/sh|g" ./install.sh \
    && ./install.sh \
    && rm -rf /tmp/rust-1.45.2-x86_64-unknown-linux-musl /tmp/rust-1.45.2-x86_64-unknown-linux-musl.tar.gz

RUN adduser -D rust \
    && echo "rust ALL=(ALL) NOPASSWD:ALL" >> /etc/sudoers \
    && mkdir -p /home/rust/.cargo/git /home/rust/.cargo/registry /home/rust/project \
    && chown -R rust: /home/rust/.cargo \
    && chown -R rust: /home/rust/project \
    && apk add --no-cache clang go gcc libgcc musl musl-dev sudo
# clang is required for using rust-bindgen

USER rust
WORKDIR /home/rust/project

# start building
FROM linux-musl-builder as md-tools-builder
COPY --chown=rust ./ /home/rust/project/
RUN rm -f Cargo.lock \
    && RUSTFLAGS="-C target-feature=-crt-static" cargo build --release
## see https://github.com/clap-rs/clap/issues/1938 when for env RUSTFLAGS.
## todo: use --target=x86_64-unknown-linux-musl

FROM alpine:3.12
COPY --chown=root:root --from=md-tools-builder /home/rust/project/target/release/md-tools /usr/local/bin/
# when using '-crt-static', we must add libgcc
RUN apk add --no-cache libgcc
ENTRYPOINT ["md-tools"]
CMD ["--help"]

# docker build --rm=true -t genshen/md-tools .

FROM centos:centos7.9.2009 as linux-centos-builder

LABEL maintainer="genshen genshenchu@gmail.com" \
      description="Build static linked rust application with using musl-libc and libc++."

ARG RUST_VER=1.50.0-x86_64-unknown-linux-gnu
ARG GO_VER=1.16.linux-amd64

# setup rust and cargo
RUN yum install -y wget \
    && wget https://static.rust-lang.org/dist/rust-${RUST_VER}.tar.gz -O /tmp/rust-${RUST_VER}.tar.gz \
    && tar -C /tmp -zxf /tmp/rust-${RUST_VER}.tar.gz \
    && cd /tmp/rust-${RUST_VER} \
    && sed -i "s|#!/bin/bash|#!/bin/sh|g" ./install.sh \
    && ./install.sh --without=rust-docs \
    && rm -rf /tmp/rust-${RUST_VER} /tmp/rust-${RUST_VER}.tar.gz

RUN adduser rust \
    && echo "rust ALL=(ALL) NOPASSWD:ALL" >> /etc/sudoers \
    && mkdir -p /home/rust/.cargo/git /home/rust/.cargo/registry /home/rust/project \
    && chown -R rust: /home/rust/.cargo \
    && chown -R rust: /home/rust/project

# setup go
RUN wget https://golang.org/dl/go${GO_VER}.tar.gz -O /tmp/go${GO_VER}.tar.gz \
    && tar -C /usr/local/ -zxf /tmp/go${GO_VER}.tar.gz \
    && ln -s /usr/local/go/bin/* /usr/local/bin/ \
    && rm -rf /tmp/go${GO_VER}.tar.gz

# install clang, which is required for using rust-bindgen.
# and set clang as C compiler.
# https://stackoverflow.com/a/48103599, the default clang version is too low
RUN yum install -y centos-release-scl sudo \
    && yum install -y llvm-toolset-7.0 \
    && ln -s /opt/rh/llvm-toolset-7.0/root/usr/bin/clang /usr/local/bin/cc

USER rust
WORKDIR /home/rust/project

# start building
FROM linux-centos-builder as md-tools-builder
COPY --chown=rust ./ /home/rust/project/
RUN source scl_source enable llvm-toolset-7.0 && CC=clang cargo build --release
## see https://github.com/clap-rs/clap/issues/1938 when for env RUSTFLAGS.
## todo: use --target=x86_64-unknown-linux-musl

FROM centos:centos7.9.2009
COPY --chown=root:root --from=md-tools-builder /home/rust/project/target/release/md-tools /usr/local/bin/
# when using '-crt-static', we must add libgcc
ENTRYPOINT ["md-tools"]
CMD ["--help"]

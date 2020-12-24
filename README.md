# md-tools

tools for MISA-MD and Crystal MD

## Build
Make sure [go](https://golang.org) is installed before building.  
### Quick build
```bash
cargo build
```
### Cross compiling
```bash
mv .cargo/config.example.toml .cargo/config.toml 
# windows x86_64 with mingw32-gcc compiler
CC=x86_64-w64-mingw32-gcc CGO_ENABLED=1 GOARCH=amd64 GOOS=windows cargo build --target=x86_64-pc-windows-gnu --release
```
```bash
mv .cargo/config.example.toml .cargo/config.toml 
# linux x86_64 with x86_64-linux-musl-gcc as compiler
CC=x86_64-linux-musl-gcc CGO_ENABLED=1 GOARCH=amd64 GOOS=linux cargo build --target=x86_64-unknown-linux-musl -C target-feature=+crt-static --release
```

## Usage
Following example will convert binary Crystal MD output to xyz and text format.  

```bash
md-tools conv -f xyz -r 64 -i crystal_md.origin.out -o origin.xyz
```

- `-f`(or `--format`) option specific output format;
- `-r`(or `--ranks`) option specific the MPI ranks in simulation;
- `-i`(or `--input`) option specific path of input file;
- `-o`(or `--output`) option specific path of output file;

## Build in docker
```bash
docker build --rm=true -t genshen/md-tools .
docker run --rm genshen/md-tools --help
```

## Crossing build
for windows building:
https://blog.nanpuyue.com/2019/052.html

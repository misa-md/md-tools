# md-tools

tools for Crystal MD

## Usage
Following example will convert binary Crystal MD output to xyz format.  

```bash
md-tools -f xyz -r 64 -i crystal_md.origin.out -o origin.xyz
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

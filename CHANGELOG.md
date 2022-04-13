<a name="unreleased"></a>
## [Unreleased]

### Build
- **cargo:** bump cargo to v3.1.6 and other dependencies
- **cargo:** bump clap to version 3.0.0-rc.8
- **cargo:** migrate clap to version 3.0.0-rc.7
- **cargo:** fix building error about "Colorized Errors" feature in clap 3.0.0-beta.5
- **cargo:** bump clap from 3.0.0-beta.4 to 3.0.0-beta.5
- **cargo:** bump libc from 0.2.98 to 0.2.102
- **cargo:** bump bindgen from 0.56.0 to 0.59.1
- **cargo:** bump clap from 3.0.0-beta.2 to 3.0.0-beta.4
- **cargo:** bump libc from 0.2.91 to 0.2.98
- **cargo:** bump rayon from 1.5.0 to 1.5.1

### Ci
- **github-action:** bump rust version in gh-action to 1.54.0 (clap requires rustc 1.54.0 or greater)
- **github-action:** bump go version in github action config from 1.14 to 1.16

### Docs
- **readme:** update building and usage document in README.md

### Feat
- **analysis:** cargo feature "minio-analysis" to enable/disable "reading from minio for analyzing"
- **conv:** also write force field to output file under `text` output format
- **conv:** feature of user controlled float number precision when outputting as xyz/text/dump
- **conv:** add cli flag `precision` for controlling float number precision in converted file
- **conv:** add support of converting md binary output file to lammps dump format
- **conv:** design new trait (introduced frame) for writing converted result to xyz or text format
- **conv:** add cli flag `standard` to select the format version of binary MD file
- **conv:** add parser implementation for the new version MD binary output file
- **diff:** add force and type fields to Particle, add ability to parse and compare these fields

### Fix
- **cli:** bug fixing of cli argument config in diff and ans sub-command
- **cli:** solve break changes involved in clap v3.0.0-beta.4 in cli.yaml
- **conv:** fix building error of `use of unstable library feature 'seek_convenience'`

### Merge
- **analysis:** Merge pull request [#33](https://github.com/misa-md/md-tools/issues/33) from misa-md/feature-cargo-feature-minio-analysis
- **cargo:** Merge pull request [#40](https://github.com/misa-md/md-tools/issues/40) from misa-md/dependabot/cargo/clap-3.0.0-beta.4
- **cargo:** Merge pull request [#51](https://github.com/misa-md/md-tools/issues/51) from misa-md/dependabot/cargo/clap-3.0.0-beta.5
- **cargo:** Merge pull request [#35](https://github.com/misa-md/md-tools/issues/35) from misa-md/dependabot/cargo/libc-0.2.98
- **cargo:** Merge pull request [#46](https://github.com/misa-md/md-tools/issues/46) from misa-md/dependabot/cargo/libc-0.2.102
- **cargo:** Merge pull request [#38](https://github.com/misa-md/md-tools/issues/38) from misa-md/dependabot/cargo/bindgen-0.59.1
- **cli:** Merge branch 'fix-cli-break-changes-clap-v3.0.0-beta.4' into branch 'master'
- **cli:** Merge pull request [#67](https://github.com/misa-md/md-tools/issues/67) from misa-md/migrate-clap-v3
- **conv:** Merge pull request [#41](https://github.com/misa-md/md-tools/issues/41) from misa-md/feature-convert-to-lammps-dump
- **conv:** Merge pull request [#47](https://github.com/misa-md/md-tools/issues/47) from misa-md/feature-format-with-precision-control
- **conv:** Merge pull request [#32](https://github.com/misa-md/md-tools/issues/32) from misa-md/new-version-binary-conversion-support
- **diff:** Merge pull request [#34](https://github.com/misa-md/md-tools/issues/34) from misa-md/improve-diff-particles

### Perf
- **conv:** use `std::io::BufWriter` when writing converted results file

### Refactor
- **cli:** remove unnecessary code when migrating to clap v3.0.0-rc.7
- **conv:** when converting, use rust to C calling, instead of C side callback, to get each atom
- **conv:** move implementation of binary MD result files conversion to mod `conv`
- **diff:** use closure as a parameter to compare two atoms
- **diff:** move velocity and force filed of `Particle` to vector field `extra_data`
- **diff:** use tuple for particles' position and velocity, refactor particles comparison imp

### Style
- **conv:** code formating and unused variables/imports removing

### Test
- **diff:** fix failed test `test_reader` after adding `type` field to `Particle`


<a name="v0.2.0"></a>
## [v0.2.0] - 2021-04-25
### Build
- **cargo:** bump libc from 0.2.86 to 0.2.91
- **cargo:** bump libc from 0.2.81 to 0.2.86
- **cargo:** bump dependencies version for clap,cc and libc
- **cargo:** add Cargo.lock file to git VCS
- **cargo:** update bindgen requirement from 0.55.1 to 0.56.0 ([#4](https://github.com/misa-md/md-tools/issues/4))
- **cargo:** update bindgen requirement from 0.54.1 to 0.55.1 ([#3](https://github.com/misa-md/md-tools/issues/3))
- **cargo:** add example cargo building config file to .cargo dir for cross compiling
- **cargo:** add ability to build.rs to build C archive from go and gen rust binding from C header
- **docker:** change to use centos as base image for docker image building
- **docker:** add .dockerignore file
- **docker:** update rust version and alpine version, and add ability to build go static lib

### Chore
- add badges to README.md file
- create dependabot.yml for cargo ecosystem
- add github CODEOWNERS file
- **cli:** add word "MISA-MD" to cli help message and README
- **github:** rename commit scope of dependabot from incorrect `npm` to `cargo`
- **version:** bump version to 0.2.0

### Ci
- **github-action:** specific rust version in github-action ci job
- **github-action:** update github action config to setup go env for rust building
- **github-action:** add upload-artifact after finishing building

### Docs
- **readme:** add building document to README.md and correct usage of conv sub-command

### Feat
- **analysis:** passed data read from minio to fn voronoy_ans  for voronoy analysis
- **analysis:** add voronoy analysis algorithm
- **analysis:** add cli flag 'input-from-minio' to ans sub-command then we can read input from minio
- **analysis:** write vacancy data and lattices coordinate of interstitials to analysis result file
- **analysis:** feature of specificing box starting position from cli with --box-start
- **analysis:** add cli option "verbose" for analysis sub-command(ans) to enable/disable verbose log
- **analysis:** add ability to set box size for analysis sub-command in cli
- **analysis:** can specific multiple output files and change default output value to `def.csv`
- **cli:** update crates clap to version 3.0.0-beta.1 so that it can show colored help message
- **cli:** use version and authors from crate in cli
- **cli:** move conversion feature to conv sub-command
- **convert:** add feature of converting multiple binary files together
- **convert:** add option --dry in 'conv' subcommand to dry-run conversion
- **diff:** add `diff` subcommand to compare particles in 2 xyz files
- **diff:** add ability to check atoms under periodic boundary condition
- **minio:** read object data of minio at go side, and make it called at rust side to build snapshot
- **minio:** add an empty go package for reading minio files in analysis
- **voronoy:** write result of voronoy analysis to output file
- **voronoy:** add feature of getting box size of any size-box
- **voronoy:** perform voronoy analysis for multiple input files

### Fix
- **analysis:** fix wrong BCC box size at x dimension when using auto box size determination
- **analysis:** get box length first and calc box size, instead of getting lat index first as before
- **compile:** fix compiling error "initializer element is not constant" in converter.c file

### Merge
- Merge pull request [#21](https://github.com/misa-md/md-tools/issues/21) from misa-md/correct-build-warnings
- Merge pull request [#20](https://github.com/misa-md/md-tools/issues/20) from misa-md/dependabot/cargo/libc-0.2.91
- Merge pull request [#2](https://github.com/misa-md/md-tools/issues/2) from misa-md/ans-file-from-minio
- Merge pull request [#1](https://github.com/misa-md/md-tools/issues/1) from crystal-md/feature-voronoy-analysis
- **analysis:** Merge pull request [#8](https://github.com/misa-md/md-tools/issues/8) from misa-md/feature-analysis-input-box-config
- **cargo:** Merge pull request [#14](https://github.com/misa-md/md-tools/issues/14) from misa-md/dependabot/cargo/libc-0.2.86
- **ci:** Merge branch 'feature-github-action' into 'master'

### Perf
- **analysis:** apply rayon data parallel to voronoy analysis
- **analysis:** add rayon data parallel support for parsing xyz file

### Refactor
- allow to parse line in xyz file into any type only if it has imp of FromStr
- move xyz_reader.rs file from mod 'ans' to mod 'xyz'
- remove some compiling warnings in building
- **analysis:** rename `box_size` in ans::box_config::BoxConfig to `input_box_size`
- **analysis:** move implementation of box size detection to new file box_config.rs
- **voronoy:** refactor voronoy.rs

### Style
- correct building warnings of `#[warn(unused_mut)]` and `#[warn(unused_variables)]`
- ignore "dead_code" building warnings from source file generated by rust-bindgen
- remove "unused imports" building warnings
- **converter:** correct building warning `#[warn(non_snake_case)]` on fields of struct OneAtomType
- **voronoy:** const name format


<a name="v0.1.0"></a>
## v0.1.0 - 2021-04-25
### Build
- **docker:** add Dockerfile to build static linked executable file

### Chore
- add LICENSE file

### Docs
- **readme:** add usage and building step in README.md file

### Feat
- add callback for rust-c interface
- add c api and c implementation, and rust calling
- **cli:** add cli parser
- **input:** add error message if opening the input file fails
- **text:** add text format output
- **version:** release md-tools version 0.1.0
- **xyz:** add feature of converting binary atoms file to xyz format

### Fix
- fix non-null-terminated filename problem in C side
- **compile:** fix compiling issue by move global const to .c file
- **text:** fix wrong InterType type (should be short) and Step type(should be unsigned long)

### Refactor
- **converter:** convert c++ code to c code

### Style
- remove unused packages and rename fn atom.getNameByEleName to atom.get_name_by_ele_name


[Unreleased]: https://github.com/misa-md/md-tools/compare/v0.2.0...HEAD
[v0.2.0]: https://github.com/misa-md/md-tools/compare/v0.1.0...v0.2.0

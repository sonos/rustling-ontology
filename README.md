# rustling-ontology
[![Build Status](https://travis-ci.org/snipsco/rustling-ontology.svg?branch=master)](https://travis-ci.org/snipsco/rustling-ontology)
[![Build Status](https://ci.appveyor.com/api/projects/status/github/snipsco/rustling-ontology?branch=master&svg=true)](https://ci.appveyor.com/project/snipsco/rustling-ontology)

Probabilistic parser for entity detection based on Rustling (https://github.com/snipsco/rustling)

Rustling is a rust port of https://github.com/facebookincubator/duckling



## Supported Output

|   Output  | OutputKind |
| --------- | ------------- |
|  Integer |  Number |
| Float | Number |
| Ordinal | Ordinal |
| Temperature | Temperature |
| Time | Time |
| TimeInterval | Time |
| AmountOfMoney | AmountOfMoney |
| Duration | Duration |


## Benches

If you want to bench the project you will need to an environment variable named `SNIPS_RUSTLING_BENCH_INPUT` with one of these values:

| Language | File |
| -------- | ---- |
| English | en.json |
| French | fr.json |
| Korean | ko.json |
| German | de.json |

## Get started

### Install

- Open a terminal

- Install rust

```
curl https://sh.rustup.rs -sSf | sh
```

Select the default installation and add cargo to your source path with `source $HOME/.cargo/env`. You can also add this line 
`export PATH=$PATH:$HOME/.cargo/bin` to your shell configuration `.bashrc` or `zshrc` (depending on your terminal)

- Clone this repository:

```
git clone git@github.com:snipsco/rustling-ontology.git
```

### Build the library

```
cd rustling-ontology
cargo build
```

It can take a while because the training for all languages takes time.

### Use the command line to run Rustling

First, go to the cli folder
```
cd cli
```

Second, run this command

```
cargo run -- --lang en parse "tomorrow morning"
```

If you want to reduce the scope of rustling, you can run:

```
cargo run -- --lang fr parse "reserve un restaurant demain matin pour cinq personnes" -k Time,Number
```

If you want to see how the sentence has been parsed by rustling, you can run:

```
cargo run -- --lang en play "monday september the twenty sixth"
```

In this mode, the reference date used is the current date

### Use the command line to debug Rustling

go to the cli-debug folder
```
cd cli-debug
```

run this command

```
cargo run -- --lang en parse "tomorrow morning"
```

It will display how the sentence has been parsed by rustling without any ML model. (Faster to compile because the training is not done)

In debug mode, the reference date used is 2013/02/12

# License

## Apache 2.0/MIT

All original work licensed under either of
 * Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)
at your option.

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall
be dual licensed as above, without any additional terms or conditions.

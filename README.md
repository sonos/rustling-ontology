# rustling-ontology
[![Build Status](https://travis-ci.org/snipsco/rustling-ontology.svg?branch=develop)](https://travis-ci.org/snipsco/rustling-ontology)

Probabilistic parser for entity detection based on Rustling (https://github.com/snipsco/rustling)

Rutling is a rust port of https://github.com/facebookincubator/duckling



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
| Korean | ko.json |
| German | de.json |

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

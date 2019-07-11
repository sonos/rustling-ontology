# Changelog
All notable changes to this project will be documented in this file.

## [0.19.0]
### Added
- [All] Added new datetime subtypes [#167](https://github.com/snipsco/rustling-ontology/pull/167)

### Fixed
- [All] Include prefix + in numbers [#186](https://github.com/snipsco/rustling-ontology/pull/186)
- [All] Set boundaries for quarters in datetimes [#185](https://github.com/snipsco/rustling-ontology/pull/185)
- [En] En moneys: add "centime" (request from PM team) [#183](https://github.com/snipsco/rustling-ontology/pull/183)
- [Fr] Fix/fr add duration vocab 2 [#182](https://github.com/snipsco/rustling-ontology/pull/182)
- [Fr] Fr "sept" abbreviation (for "september") removed if no following dot [#178](https://github.com/snipsco/rustling-ontology/pull/178)
- [Es] Misc. fixes for Spanish. [#177](https://github.com/snipsco/rustling-ontology/pull/177)
- [Ja] Delete rule that accepts numbers followed by quantifiers for cardinal [#176](https://github.com/snipsco/rustling-ontology/pull/176)
- [Fr] Fix some interval rules in Fr and switched Duration/Datetime priority [#173](https://github.com/snipsco/rustling-ontology/pull/173)
- [En] Typo in English training [#168](https://github.com/snipsco/rustling-ontology/pull/168)

## [0.18.1]
### Fixed
- [Es] Various fixes
- [Ja] Remove quantifiers in Japanese cardinals
- [Fr] Fixed some interval rules and switched Duration/Datetime priority
- [En] Fixed typos in training examples

### Added
- [Pt] Improved all entities

## [0.18.0]
### Changed
- [Pt] Add Portuguese V0

### Fixed
- Crash when attempting to parse wrong month and day.
- Fix and adjust date written abbreviations in all languages.
- [De] Change end of time span setting to get the right intervals.
- [De] Fix relative minute for value=1.
- [It] Fix financial rule with Rubles.
- [Es] Fix percentage pattern and other typos.

## [0.17.7] - 2019-01-17
### Changed
- Fix resolution of decimal numbers in textual form.

## [0.17.6] - 2018-12-13
### Changed
- Fuller coverage of Spanish and Italian

[0.19.0]: https://github.com/snipsco/rustling-ontology/compare/0.18.1...0.19.0
[0.18.1]: https://github.com/snipsco/rustling-ontology/compare/0.18.0...0.18.1
[0.18.0]: https://github.com/snipsco/rustling-ontology/compare/0.17.7...0.18.0
[0.17.7]: https://github.com/snipsco/rustling-ontology/compare/0.17.6...0.17.7
[0.17.6]: https://github.com/snipsco/rustling-ontology/compare/0.17.5...0.17.6

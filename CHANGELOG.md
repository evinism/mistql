# Change Log

Currently, the language version, Python implementation version, and JavaScript implementation version share the version name. This changelog reflects changes on both the language, spec, and implementations, as there's no real distinction between them right now. 

In the future, this is expected to change. It's likely that at 0.5.0, we will define the language spec separately from the implementations.

Format [Keep A ChangeLog](https://keepachangelog.com/en/1.0.0/)

## [0.5.0-beta.1]

### Changed
- [185](https://github.com/evinism/mistql/pull/185)
 JS: Updated `filterkeys`, `filtervalues`, `matchkeys`,`matchvalues` to reject non-objects.
- [203](https://github.com/evinism/mistql/pull/203)Python: Made typeguard opt-in via env var.
- [201](https://github.com/evinism/mistql/pull/201)Python: Enabled an LRU cache for the parser.
- [202](https://github.com/evinism/mistql/pull/202)Python: Enabled lazy conversion and evaluation of input objects via an opt-in instance flag.

### Fixed
- [185](https://github.com/evinism/mistql/pull/185) Made `string` throw errors for deeply-nested non-stringable values.
- [185](https://github.com/evinism/mistql/pull/185) JS: Modulo operator now throws an error for division by zero.

## [0.4.12]

### Changed
- https://github.com/evinism/mistql/pull/182, https://github.com/evinism/mistql/pull/181 Bumped many dependencies to latest versions.
- https://github.com/evinism/mistql/pull/183 Removed EOL'd Python3.7 support.

## [0.4.11]

### Changed
- https://github.com/evinism/mistql/pull/161 Established `null` as unsortable.

### Fixed
- https://github.com/evinism/mistql/pull/161 Made uncomparable types uncomparable when using `<`, `>`, `<=`, and `>=`, as before behavior was platform-specific. 

## [0.4.10]

### Added
- https://github.com/evinism/mistql/pull/149 Tests establishing expected behavior for unicode characters and indexing.

### Changed
- https://github.com/evinism/mistql/pull/150 Modified JS implementation `#index` and `#split` functions to match the expected unicode behavior as established above.

### Fixed
- https://github.com/evinism/mistql/pull/152 Allowed numbers to end in bare decimal points
- https://github.com/evinism/mistql/pull/158 Dramatically improved JS parser and lexer performance.
- https://github.com/evinism/mistql/pull/157 Dramatically improved indexing performance on unicode strings.

## [0.4.9] 2022-05-05

### Added
- https://github.com/evinism/mistql/pull/135 Tests establishing expected behavior for sorting a single-element array with an unsortable function.

### Changed
- https://github.com/evinism/mistql/pull/135 Made it so that sorting single-element arrays with unsortable contents in Python and Javascript implementations.

### Fixed
- https://github.com/evinism/mistql/pull/136 Resolves issue wherein we were able to return non-external values in JS if the values were nested within an array or object.
- https://github.com/evinism/mistql/pull/142 Resolves broken behavior for piping to functions that use the $ variable, e.g. `[] | $.count`

## [0.4.8] 2022-04-19

Changelog Start Release.


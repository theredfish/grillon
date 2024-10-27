# `grillon` changelog

## [v0.5.0] - 2024-10-27

- [Diff](/../../compare/v0.4.0...v0.5.0)
- [Milestone](/../../milestone/3)

[v0.5.0]: /../../tree/v0.5.0

### Breaking Changes

- Replace Hyper by Reqwest for the default blanket implementation ([#63], [#73])

### Changed

- Expansion of the book documentation ([#42])

### Added

- `Equality` json path assertions ([#34])
- Built-in json schema assertion ([#9], [#48], [#50], [#51], [#55])
- Extend json path assertions with `contains` and `does_not_contain` ([#40], [#75])
- Support string literals for the `headers` built-in functions ([#33], [#78])
- Built-in single header assertion ([#82])
- Add optional cookie store to the http client ([#54], [#77])
- Add static links checker ([#43])

[#9]: /../../issues/9
[#33]: /../../issues/33
[#34]: /../../pull/34
[#40]: /../../issues/40
[#42]: /../../pull/42
[#43]: /../../pull/43
[#48]: /../../pull/48
[#50]: /../../pull/50
[#51]: /../../issues/51
[#54]: /../../issues/54
[#55]: /../../pull/55
[#63]: /../../issues/63
[#73]: /../../issues/73
[#75]: /../../pull/75
[#77]: /../../pull/77
[#78]: /../../pull/78
[#82]: /../../pull/82

## [0.4.0] - 2023-01-26

[0.4.0]: /../../tree/v0.4.0

- [Diff](/../../compare/v0.3.0...v0.4.0)

### Changed

- Complete rewrite of the assertion logic ([#22])
- Enhancement of the http matchers as part of the DSL ([#17], [#18], [#23])
- Simplified CI file and updated github actions ([#27])

### Added

- Domain specific language for matching operators ([#20])
- Http response time matcher ([#19])
- Dependabot to manage dependency updates ([#30])
- Grillon book with mdbook and github actions ([#28])

[#17]: /../../issues/17
[#18]: /../../issues/18
[#19]: /../../issues/19
[#20]: /../../pull/20
[#22]: /../../pull/22
[#23]: /../../pull/23
[#27]: /../../issues/27
[#28]: /../../issues/28
[#30]: /../../issues/30

## [0.3.0] - 2022-01-25

[0.3.0]: /../../tree/v0.3.0

- [Diff](/../../compare/v0.2.0...v0.3.0)

### Changed

- `Response::json` has now a return type following the standard for `async` functions bounded by the
  lifetime of their arguments. Now the function should also be compatible with `async_trait` without `Send` requirement.
  ([#16])

[#16]: /../../pull/16

## [0.2.0] - 2022-01-22

[0.2.0]: /../../tree/v0.2.0

- [Diff](/../../compare/v0.1.0...v0.2.0)
- [Milestone](/../../milestone/1)

### Added

- Built-in HTTP functions : `HEAD`, `OPTIONS` and `CONNECT`. ([#8], [#12])
- `Assert::assert_fn` to extend built-in assertions with a custom assertion. ([#7], [#13], [#14] [#15])

### Changed

- `Assert` fields (`json`, `headers`, `status`) are now public to allow external access. ([#15])

[#7]: /../../issues/7
[#8]: /../../issues/8
[#13]: /../../issues/13
[#12]: /../../pull/12
[#14]: /../../pull/14
[#15]: /../../pull/15

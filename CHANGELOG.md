# `grillon` changelog

## [v0.5.0-alpha.1] - 2023

- [Diff](/../../compare/v0.4.0...v0.5.0-alpha.1)
- [Milestone](https://github.com/theredfish/grillon/milestone/3)

[v0.5.0-alpha.1]: /../../tree/v0.5.0-alpha.1

### Changed

- Expansion of the book documentation ([#42])

### Added

- TLS support by default with Hyper ([#45])
- Built-in json path assertions ([#34])
- Built-in json schema assertion ([#48], [#50])

[#34]: /../../issues/34
[#42]: /../../issues/42
[#45]: /../../issues/45
[#48]: /../../issues/48
[#50]: /../../issues/50

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

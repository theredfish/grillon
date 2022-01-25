# `grillon` changelog

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

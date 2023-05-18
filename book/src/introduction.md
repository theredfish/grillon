# Introduction

Grillon is a Rust library offering an elegant and natural way to approach API testing in Rust.

- Elegant, intuitive and expressive API
- Built-in testing functions
- Extensible

Check out our [Quickstart](./quickstart.md).

## Usage

As the library is flexible, you can easily integrate it into your testing strategy in a Rust project.
You can use it for synthetic monitoring, endpoint monitoring, functional testing, integration
testing, BDD testing (e.g [cucumber-rs](https://github.com/cucumber-rs/cucumber)), ... it's up to
you. Grillon does not impose any test strategy or organization.

Depending on how you configure your [logs](logs.md), the execution will fail-fast or not and can be
formatted in a human-readable or json output.

## Next big steps

Here is an unordered and non-exhaustive list of what is planned for Grillon next:

- Improve HTTP testing: HTTP/1.1 + HTTP/2, json path, xpath, form-data
- Extend testing capabilities per-protocol/framework
  - WebSocket
  - gRPC
  - SSL
  - TCP, UDP, DNS, ICMP
- Logs and metrics
- Support for YAML-formatted (or other formats) tests to extend the library outside of Rust projects

## Sponsors

[![Owl Duty logo](./img/owlduty_logo.jpg)](https://owlduty.com)

[Owl Duty](https://owlduty.com) is the platform for developers and testers, providing the tools and the infrastructure to monitor and test APIs.

## You?

You can sponsor me ([@theredfish](https://github.com/sponsors/theredfish)) with the `Coffee sponsor` tier (or higher) and I will add your handle / company name here.

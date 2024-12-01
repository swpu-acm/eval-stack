# Changelog

## \[0.2.0]

### New Features

- [`5867664`](https://github.com/swpu-acm/eval-stack/commit/5867664299189524d24c781d0826eaf7e932debd) ([#9](https://github.com/swpu-acm/eval-stack/pull/9) by [@fu050409](https://github.com/swpu-acm/eval-stack/../../fu050409)) Add support for Deno.js, and other changes:

  - Use deno instead of node for executing JavaScript files, deny all permissions by default.
  - Allow stderr to be piped since we can capture it for error messages.
  - Disable core dumps by default.
  - Set CPU time limits using `libc`.
  - Use seccomp to restrict sys calls and fs operations.
  - Prevent process to create subprocesses.

## \[0.1.2]

### New Features

- [`839c728`](https://github.com/swpu-acm/eval-stack/commit/839c728fada2e05fd47f0f879565f7d5a5f0a67e) Add serde derive for `Language` struct.

## \[0.1.1]

### New Features

- [`20bb292`](https://github.com/swpu-acm/eval-stack/commit/20bb292693c179588b6f46792f6b299917210ea0) ([#4](https://github.com/swpu-acm/eval-stack/pull/4) by [@fu050409](https://github.com/swpu-acm/eval-stack/../../fu050409)) Judge outputs for each line of expected outputs.
- [`a500968`](https://github.com/swpu-acm/eval-stack/commit/a50096867409251a4aac30822f19dc54281c6b47) ([#6](https://github.com/swpu-acm/eval-stack/pull/6) by [@fu050409](https://github.com/swpu-acm/eval-stack/../../fu050409)) Add `serde` feature to enable serialization and deserialization.

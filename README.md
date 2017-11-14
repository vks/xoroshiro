# xoroshiro

[![Status][status-img]][status-url]

Rust implementation of the [xoroshiro128+, xorshift1024*φ and
splitmix64](http://xoroshiro.di.unimi.it) random number generators.

## License

`xoroshiro` is primarily distributed under the terms of both the MIT license and
the Apache License (Version 2.0).

See LICENSE-APACHE, and LICENSE-MIT for details.

## Other projects

* Parts of the code is were taken from [this pull
  request](https://github.com/rust-lang-nursery/rand/pull/102).
* Some of the test vectors were taken and adapted from the [xorshift crate]
  (https://github.com/astocko/xorshift).
* The [xoroshiro128 crate](https://github.com/mscharley/rust-xoroshiro128) is
  similar to this one.


[status-img]: https://travis-ci.org/vks/xoroshiro.svg?branch=master
[status-url]: https://travis-ci.org/vks/xoroshiro

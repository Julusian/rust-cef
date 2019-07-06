# Rust CEF SDK

Bindings to the [Chromium Embedded Framework](https://bitbucket.org/chromiumembedded/cef/wiki/Home) for Rust.
This aims to be an easy to use and safe wrapper around the CEF capi.
It aims to follow the C++ api wherever possible, to make porting code or learning from other documentation easy.

Note: This is a work in progress. Many areas are not yet implemented, and it is not very well tested yet.

## Installation

You will need to download the minimal distribution of CEF from http://opensource.spotify.com/cefbuilds/index.html.
Currently this has only been tested with `CEF 75.0.13+g2c92fcd+chromium-75.0.3770.100` other versions may work, but may also unexpectedly crash.

Unzip the downloaded distribution, and inform cargo where it is located through the CEF_ROOT environment variable

eg for linux: `export CEF_ROOT=/home/julus/Projects/cef_binary_75.0.13+g2c92fcd+chromium-75.0.3770.100_linux64_minimal` 

You can then run cargo commands like usual and it should work as expected.

## Usage

See the examples, or the official C++ docs for more information.

## License

Licensed under either of

 * Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any
additional terms or conditions.
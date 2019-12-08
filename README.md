# aoc_2019

Solutions to the 2019 [Advent of Code](https://adventofcode.com/2019) problems in Rust.

## Structure
Shared code in `src/lib.rs` and etc. Each day is a separate file `src/bin/dayX.rs` to create a separate executable.

## Using
1. Put input in `input/dayX.txt`
2. Run `cargo run --bin dayX`

## Testing
Mostly just unit tests. Each day's file _should_ have at least a few unit tests, derived from the sample inputs. Use `cargo test` to run tests.

## License
This project is licensed under the GNU AGPL v3.0 or later. See [LICENSE](./LICENSE) for more details.
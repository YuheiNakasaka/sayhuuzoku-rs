# sayhuuzoku-rs

Implementation of [YuheiNakasaka/sayhuuzoku](https://github.com/YuheiNakasaka/sayhuuzoku) with Rust.

## Example

```
$ cargo run -- -c 2
ザ・エゴイストセレブコレクション

$ cargo run -- -c 3
アロマテラスぇちちバナナ

$ cargo run -- -c 4
幽玄空港白書アナル
```

## Installation

You can use this command anywhere in local machine by running below command.

```
cargo install --path .
```

After that, the command can be run.

```
sayhuuzoku-rs -c 3
```

## Usage

```
Huuzoku name generator 1.0.0
Yuhei Nakasaka
Generator of shop names like huuzoku

USAGE:
    sayhuuzoku-rs [FLAGS] [OPTIONS]

FLAGS:
    -h, --help       Prints help information
    -v, --verbose
    -V, --version    Prints version information

OPTIONS:
    -c, --count <count>    [default: 4]
```

## License

The library is available as open source under the terms of the MIT License.

# 🐸 Slippi DB

Slippi DB ingests [Slippi replays][slippi] and puts the data into a SQLite database for easier parsing.

The primary goal of this project is to make it easier to analyze large amounts of Slippi data. Its end goal is to create something similar to [Ballchasing.com][ballchasing] but for Melee.

Written in [Rust][rust] using [Peppi][peppi], Slippi DB can parse gigabytes worth of Slippi files in a couple seconds.

## Usage

```
USAGE:
    slippi-db [OPTIONS] <directories>...

ARGS:
    <directories>...    Directories to search for .slp files in

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -o, --output-db <output-db>    Set output database file [default: slippi.db]
```

## Building

```shell
$ git clone https://github.com/mtimkovich/slippi-db
$ cd slippi-db
$ cargo build --release
```

## 🗒️ Examples

There's some examples of what is possible in the `examples/` directory. This is good starting point, especially if you're not familiar with SQL. These can be run via:

```shell
$ sqlite3 slippi.db < examples/hours_played.sql
```

*If you come up with a cool query, make a PR and I'll add it to the examples!*

## 🚧 Roadmap

- [x] Write data to SQLite DB.
- [x] Check the filelist for new files.
- [x] Save player data.
- [x] Create more SQL examples.
- [x] Detect the winner(s).
  - [x] Discard short matches (<30s).
  - [ ] Handle tiebreaks.
  - [ ] Detect rage-quits and assign them losses.
- [ ] Create releases.

## Bug/Feature Requests

* [File an issue!](https://github.com/mtimkovich/slippi-db/issues)
* Twitter: [@DJSwerveGG][twitter]

PRs welcome!

[slippi]: https://github.com/project-slippi/slippi-wiki/blob/master/SPEC.md
[peppi]: https://github.com/hohav/peppi
[rust]: https://www.rust-lang.org/
[ballchasing]: https://ballchasing.com
[twitter]: https://twitter.com/DJSwerveGG

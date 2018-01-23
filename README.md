# wof-tools: Process data from the whosonfirst-data repository

Parse out geojson from the whosonfirst project and extract geometries.

## Getting Started

Take a look at the [Whos on First dataset](https://github.com/whosonfirst-data/whosonfirst-data). It's big! 

### Prerequisites

Make sure you have [rust
installed](https://www.rust-lang.org/en-US/install.html).

```
$ rustc --version
rustc 1.23.0 (766bd11c8 2018-01-01)
```

### Building

In the root of the project, run:

```
$ cargo build --release
```

Take a look at the options:

```
$ ./target/release/wof-tools --help
usage: wof-tools [options]

Options:
    --wof-data-dir <dir>    wof directory to use
    -h, --help              show this message
```

Then run on a repository:

```
$ ./target/release/wof-tools --wof-data-dir /path/to/whosonfirst-data
```


## Running the tests

Explain how to run the automated tests for this system

### Break down into end to end tests

Explain what these tests test and why

```
Give an example
```

### And coding style tests

Explain what these tests test and why

```
Give an example
```

<!--
based on:
https://gist.github.com/PurpleBooth/109311bb0361f32d87a2
-->

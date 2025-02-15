# gzip-size-cli
Rust version of [@sindresorhus](https://github.com/sindresorhus)'s [gzip-size-cli](https://github.com/sindresorhus/gzip-size-cli).

I use that tool a lot but the startup time of Node.js is a bit high for me.
This has the exact same API and features but it runs fast enough that you won't notice the latency.

```
gzip-size
Show the gzipped size of a file or stdin.

USAGE:
    gzip-size [OPTIONS] [FILE]

ARGS:
    <FILE>

OPTIONS:
    -h, --help                Print help information
        --include-original    Include original size
        --level <LEVEL>       Compression level [0-9] [default: 9]
        --raw                 Display value in bytes
```

## Install

Clone the repository, and use Cargo:
```bash
cargo build --release
```

You can put the resulting binary anywhere in your `$PATH`:
```bash
cp target/release/gzip-size /usr/bin/gzip-size
```

## License
Licensed under either of Apache License, Version 2.0 or MIT license at your option.

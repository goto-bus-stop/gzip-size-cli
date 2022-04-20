# gzip-size-cli
Rust version of [@sindresorhus](https://github.com/sindresorhus)'s [gzip-size-cli](https://github.com/sindresorhus/gzip-size-cli).

I use that tool a lot but the startup time of Node.js is a bit high for something so simple.

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

## License
Licensed under either of Apache License, Version 2.0 or MIT license at your option.

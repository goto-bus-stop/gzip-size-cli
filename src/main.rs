use clap::Parser;
use flate2::write::GzEncoder;
use flate2::Compression;
use humansize::{format_size, BINARY};
use std::fs::File;
use std::io::{BufReader, Read, Write};
use std::path::PathBuf;

/// Show the gzipped size of a file or stdin. A straight port of sindresorhus' Node.js gzip-size-cli.
#[derive(Parser, Debug)]
#[clap(name = "gzip-size")]
struct Cli {
    /// The file to compress. If none is given, input is read from stdin.
    file: Option<PathBuf>,
    /// Compression level [0-9]
    #[clap(long, default_value_t = 9)]
    level: u32,
    /// Display value in bytes.
    #[clap(long)]
    raw: bool,
    /// Include original size.
    #[clap(long = "include-original")]
    include_orginal: bool,
}

/// `Read` wrapper that counts how many bytes were read.
struct ReadSize<R: Read> {
    inner: R,
    size: usize,
}
impl<R: Read> ReadSize<R> {
    /// Wrap a `Read` instance.
    fn new(inner: R) -> Self {
        Self { inner, size: 0 }
    }

    /// Return the number of bytes that were read.
    fn size(&self) -> usize {
        self.size
    }
}

impl<R: Read> Read for ReadSize<R> {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        let chunk_size = self.inner.read(buf)?;
        self.size += chunk_size;
        Ok(chunk_size)
    }
}

/// `Write` wrapper that counts how many bytes were written.
struct WriteSize<W: Write> {
    inner: W,
    size: usize,
}
impl<W: Write> WriteSize<W> {
    /// Wrap a `Write` instance.
    fn new(inner: W) -> Self {
        Self { inner, size: 0 }
    }

    /// Return the number of bytes that were written.
    fn size(&self) -> usize {
        self.size
    }
}

impl<W: Write> Write for WriteSize<W> {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        let chunk_size = self.inner.write(buf)?;
        self.size += chunk_size;
        Ok(chunk_size)
    }
    fn flush(&mut self) -> std::io::Result<()> {
        self.inner.flush()
    }
}

fn main() -> anyhow::Result<()> {
    let args = Cli::parse();

    let compression = Compression::new(args.level);
    let mut encoder = GzEncoder::new(WriteSize::new(std::io::sink()), compression);

    let read_size = if let Some(path) = args.file {
        let mut f = ReadSize::new(BufReader::with_capacity(64 * 1024, File::open(path)?));
        std::io::copy(&mut f, &mut encoder)?;
        f.size()
    } else if atty::isnt(atty::Stream::Stdin) {
        let stdin = std::io::stdin();
        let mut rs = ReadSize::new(stdin.lock());
        std::io::copy(&mut rs, &mut encoder)?;
        rs.size()
    } else {
        anyhow::bail!("No input: specify a file name or pipe something");
    };

    let write_size = encoder.finish()?.size();
    match (args.include_orginal, args.raw) {
        (false, false) => println!("{}", format_size(write_size, BINARY)),
        (false, true) => println!("{}", write_size),
        (true, false) => println!(
            "{} → {}",
            format_size(read_size, BINARY),
            format_size(write_size, BINARY),
        ),
        (true, true) => println!("{} → {}", read_size, write_size),
    }

    Ok(())
}

#[cfg(test)]
mod tests {}

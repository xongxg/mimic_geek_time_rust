use std::fs::File;
use std::io::{BufReader, Read};
use std::io::Result;

fn main() {
    let f = File::open("/etc/hosts").unwrap();
    let mut reader = MyReader::new(BufReader::new(f));

    let size = reader.process().unwrap();
    println!("total read size: {}", size);
}

struct MyReader<R> {
    reader: R,
    buf: String,
}

impl<R> MyReader<R> {
    fn new(reader: R) -> MyReader<R> {
        Self {
            reader,
            buf: String::with_capacity(1024),
        }
    }
}

impl<R: Read> MyReader<R>
// where R:Read
{
    pub fn process(&mut self) -> Result<usize> {
        self.reader.read_to_string(&mut self.buf)
    }
}

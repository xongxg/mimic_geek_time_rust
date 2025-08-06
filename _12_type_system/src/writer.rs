use std::io::Write;
use std::io::{BufWriter, Result};
use std::net::TcpStream;

#[derive(Debug)]
pub struct MyWriter<W> {
    writer: W,
}

impl<W: Write> MyWriter<W> {
    pub fn new(writer: W) -> MyWriter<W> {
        Self { writer }
    }


    pub fn write(&mut self, buf: &str) -> Result<()> {
        self.writer.write_all(buf.as_bytes())
    }
}

fn main() {
    let stream = TcpStream::connect("127.0.0.1:8080").unwrap();
    let mut writer = MyWriter::new(BufWriter::new(stream));

    writer.write("Hello, World!").unwrap();
}

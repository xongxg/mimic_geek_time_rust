use std::fmt::{Debug, Formatter};
use std::io::Write;

fn main() {
    let mut buf = BufBuilder::new();
    buf.write_all(b"hello world").expect("TODO: panic message");
    println!("buf: {:?}", buf);
}

pub struct BufBuilder {
    buf: Vec<u8>,
}

impl Debug for BufBuilder {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", String::from_utf8_lossy(&self.buf))
    }
}

impl BufBuilder {
    pub fn new() -> BufBuilder {
        Self {
            buf: Vec::with_capacity(1024),
        }
    }
}

impl Write for BufBuilder {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        self.buf.extend_from_slice(buf);
        Ok(buf.len())
    }

    fn flush(&mut self) -> std::io::Result<()> {
        Ok(())
    }
}

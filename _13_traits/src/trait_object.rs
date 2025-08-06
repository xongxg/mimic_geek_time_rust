use std::fs::File;
use std::io::{Read, Write};

fn main() {
    let mut f = File::create("/tmp/test_write_trait").unwrap();
    let w: &mut dyn Write = &mut f;
    w.write_all(b"Hello, world!").unwrap();

    // let w1 = w.by_ref();
    // let b = f.by_ref(&mut f);
    // b.write_all(b"Hello, world!").unwrap();

}

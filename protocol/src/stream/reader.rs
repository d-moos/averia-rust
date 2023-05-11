use std::io::Bytes;
use std::mem::{size_of, size_of_val};

struct Simple {
    data: [u8; 8096],
}

struct Massive {
    data: [[u8; 4096]; 2]
}

trait Reader {
    fn foo() {
    }
}
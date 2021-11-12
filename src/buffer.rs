use std::io::prelude::*;
use std::str;

const CAPACITY: usize = 512;

pub struct Buffer {
    pub inner: Vec<u8>,
    pub cursor: usize,
}

impl Buffer {
    pub fn new() -> Self {
        Self {
            inner: vec![0u8; CAPACITY],
            cursor: 0,
        }
    }

    pub fn insert(&mut self, b: u8) {
        self.inner[self.cursor] = b;
        self.cursor += 1;

        if self.cursor == self.inner.len() {
            self.inner.append(&mut vec![0u8; CAPACITY]);
        }
    }
}

impl ToString for Buffer {
    fn to_string(&self) -> String {
        let mut ret = String::new();
        let s = str::from_utf8(&self.inner[0..self.cursor]).unwrap();
        ret.push_str(s);
        ret
    }
}

impl Seek for Buffer {
    fn seek(&mut self, _pos: std::io::SeekFrom) -> std::io::Result<u64> {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new() {
        let _buf = Buffer::new();
    }

    #[test]
    fn insert() {
        let mut buf = Buffer::new();

        buf.insert(b'a');
        assert_eq!(buf.inner[0], b'a');

        buf.insert(b'b');
        assert_eq!(buf.inner[1], b'b');

        buf.insert(b'c');
        assert_eq!(buf.inner[2], b'c');
    }

    #[test]
    fn to_string() {
        let mut buf = Buffer::new();
        buf.insert(b'f');
        buf.insert(b'o');
        buf.insert(b'o');
        let s = buf.to_string();
        assert_eq!(s, "foo");
    }
}

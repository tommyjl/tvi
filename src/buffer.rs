use std::io::prelude::*;
use std::io::SeekFrom;
use std::str;

const CAPACITY: usize = 16;

pub struct Buffer {
    pub inner: Vec<u8>,
    pub cursor: usize,
    gap_end: usize,
}

impl Buffer {
    pub fn new() -> Self {
        Self {
            inner: vec![0u8; CAPACITY],
            cursor: 0,
            gap_end: CAPACITY,
        }
    }

    pub fn insert(&mut self, b: u8) {
        self.inner[self.cursor] = b;
        self.cursor += 1;

        if self.cursor == self.inner.len() {
            self.inner.append(&mut vec![0u8; CAPACITY]);
            self.inner[self.gap_end..].rotate_right(CAPACITY);
            self.gap_end += CAPACITY;
        }
    }

    fn draw_buffer(&self) -> String {
        let mut ret = String::new();

        let left = str::from_utf8(&self.inner[0..self.cursor]).unwrap();
        ret.push_str(left);
        ret.push('\n');

        let right = str::from_utf8(&self.inner[self.gap_end..]).unwrap();
        ret.push_str(right);
        ret.push('\n');

        ret
    }

    fn draw_buffer_info(&self) -> String {
        format!("{} | {} | {}", self.cursor, self.gap_end, self.inner.len())
    }
}

impl ToString for Buffer {
    fn to_string(&self) -> String {
        let mut ret = String::new();
        ret.push_str(&self.draw_buffer());
        ret.push('\r');
        ret.push_str(&self.draw_buffer_info());
        ret.push('\r');
        ret
    }
}

impl Seek for Buffer {
    fn seek(&mut self, pos: SeekFrom) -> std::io::Result<u64> {
        if let SeekFrom::Current(offset) = pos {
            if offset < 0 && self.cursor > 0 {
                self.cursor -= offset.abs() as usize;
                self.inner[self.cursor..self.gap_end].rotate_left(offset.abs() as usize);
                self.gap_end -= 1;
            } else if offset > 0 && self.gap_end < self.inner.len() {
                self.gap_end += 1;
                self.inner[self.cursor..self.gap_end].rotate_right(offset.abs() as usize);
                self.cursor += offset as usize;
            }
        } else {
            todo!()
        }
        Ok(self.cursor as u64)
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

use crate::buffer::Buffer;
use std::io;

pub struct Tui {
    buf: Buffer,
}

impl Tui {
    pub fn new() -> Self {
        // TODO: Enable raw mode
        Self { buf: Buffer::new() }
    }

    fn handle_input(&mut self, input: u8) {
        self.buf.insert(input);
    }

    pub fn run(&mut self) {
        loop {
            let mut buf = String::new();
            let length = io::stdin().read_line(&mut buf).unwrap();

            let bytes = buf.into_bytes();
            for byte in bytes.iter().take(length) {
                self.handle_input(*byte);
            }

            println!("-------start");
            print!("{}", self.buf.to_string());
            println!("-------end");
        }
    }
}

impl Default for Tui {
    fn default() -> Self {
        Self::new()
    }
}

use crate::buffer::Buffer;
use crate::term::Term;
use std::io;
use std::io::prelude::*;

pub struct Tui {
    buf: Buffer,
    term: Term,
}

impl Tui {
    pub fn new() -> Self {
        Self {
            buf: Buffer::new(),
            term: Term::new().expect("Failed to set up terminal"),
        }
    }

    fn handle_input(&mut self, input: u8) {
        self.buf.insert(input);
    }

    pub fn run(&mut self) {
        loop {
            let mut buf = [0u8; 512]; // Arbitrary choice of 512
            let len = io::stdin().read(&mut buf).unwrap();
            for byte in buf.iter().take(len) {
                // TODO: Proper signal handling
                if *byte == 3 {
                    return;
                } else {
                    // print!("{:x} - {}\r\n", *byte, *byte as char);
                    self.handle_input(*byte);
                }
            }
            self.draw();
        }
    }

    fn draw(&self) {
        let mut out = io::stdout();
        print!("{}", self.buf.to_string());
        out.flush().unwrap();
    }
}

impl Default for Tui {
    fn default() -> Self {
        Self::new()
    }
}

enum TerminalInput {
    Up,
    Right,
    Down,
    Left,
    Utf8(char),
}

fn read_thingy(reader: BufRead) -> TerminalInput {
    reader.&mut
    // reader.iter().next();
    // let next = reader
    TerminalInput::Utf8('b' as char)
}

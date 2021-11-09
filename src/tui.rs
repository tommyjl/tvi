use crate::buffer::Buffer;
use crate::term::Term;
use crate::term_seq::ClearScreen;
use crate::term_seq::Goto;
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
            let input = match len {
                1 => match buf[0] {
                    0x03 => Input::Etx,
                    0x1A => Input::Sub,
                    0x1B => Input::Esc,
                    0x7F => Input::Backspace,
                    0x0D => Input::Enter,
                    // 0x0A => Input::LF,
                    c if c < 0x20 => panic!("Unhandled input single character {}", c),
                    _ => Input::Utf8(buf[0] as char),
                },
                3 => match &buf[0..3] {
                    b"\x1B[A" => Input::Up,
                    b"\x1B[B" => Input::Down,
                    b"\x1B[C" => Input::Right,
                    b"\x1B[D" => Input::Left,
                    b"\x1B[F" => Input::End,
                    b"\x1B[H" => Input::Home,
                    _ => panic!("Unhandled input {:?}", &buf[0..3]),
                },
                4 => match &buf[0..4] {
                    b"\x1B[1~" => Input::Home,
                    b"\x1B[2~" => Input::Insert,
                    b"\x1B[3~" => Input::Delete,
                    b"\x1B[4~" => Input::End,
                    b"\x1B[5~" => Input::PgUp,
                    b"\x1B[6~" => Input::PgDown,
                    b"\x1B[7~" => Input::Home,
                    b"\x1B[8~" => Input::End,
                    _ => panic!("Unhandled input {:?}", &buf[0..4]),
                },
                _ => panic!("Unhandled input {:?}", &buf[0..len]),
            };

            if cfg!(feature = "debug_inputs") {
                print!("{} - {:?}\r\n", len, input);
            }

            match input {
                Input::Etx => return,
                Input::Utf8(c) => self.handle_input(c as u8),
                _ => {}
            };

            if !cfg!(feature = "debug_inputs") {
                self.draw();
            }
        }
    }

    fn draw(&self) {
        let mut out = io::stdout();
        print!("{}{}{}\r", Goto(1, 1), ClearScreen, self.buf.to_string());
        out.flush().unwrap();
    }
}

impl Default for Tui {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug)]
enum Input {
    Etx, // End of text (Ctrl-C interrupt)
    Sub, // Substitute (Ctrl-Z suspend)

    Esc,
    Enter,
    Up,
    Down,
    Right,
    Left,
    Backspace,
    Home,
    Insert,
    Delete,
    End,
    PgUp,
    PgDown,

    Utf8(char),
}

use crate::buffer::Buffer;
use crate::term::Term;
use crate::term_seq::ClearScreen;
use crate::term_seq::Goto;
use std::io;
use std::io::prelude::*;

pub struct Tui {
    buf: Buffer,
    _term: Term,
}

impl Tui {
    pub fn new() -> Self {
        Self {
            buf: Buffer::new(),
            _term: Term::new().expect("Failed to set up terminal"),
        }
    }

    fn handle_input(&mut self, input: u8) {
        self.buf.insert(input);
    }

    pub fn run(&mut self) {
        loop {
            let input = read_input(&mut io::stdin()).unwrap();

            if cfg!(feature = "debug_inputs") {
                print!("Input = {:?}\r\n", input);
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
    Tab,
    BackTab,

    Up,
    Down,
    Right,
    Left,

    Backspace,
    Insert,
    Delete,
    Home,
    End,
    PgUp,
    PgDown,

    Utf8(char),
}

fn read_input<R>(stdin: &mut R) -> Result<Input, String>
where
    R: Read,
{
    let mut buf = [0u8; 512]; // Arbitrary choice of 512
    let len = stdin.read(&mut buf).unwrap();
    match &buf[0..len] {
        [0x03] => Ok(Input::Etx),
        [0x09] => Ok(Input::Tab),
        [0x1A] => Ok(Input::Sub),
        [0x1B] => Ok(Input::Esc),
        [0x7F] => Ok(Input::Backspace),
        [0x0A] => Ok(Input::Enter),
        [0x0D] => Ok(Input::Enter),
        b"\x1B[A" => Ok(Input::Up),
        b"\x1B[B" => Ok(Input::Down),
        b"\x1B[C" => Ok(Input::Right),
        b"\x1B[D" => Ok(Input::Left),
        b"\x1B[F" => Ok(Input::End),
        b"\x1B[H" => Ok(Input::Home),
        b"\x1B[Z" => Ok(Input::BackTab),
        b"\x1B[1~" => Ok(Input::Home),
        b"\x1B[2~" => Ok(Input::Insert),
        b"\x1B[3~" => Ok(Input::Delete),
        b"\x1B[4~" => Ok(Input::End),
        b"\x1B[5~" => Ok(Input::PgUp),
        b"\x1B[6~" => Ok(Input::PgDown),
        b"\x1B[7~" => Ok(Input::Home),
        b"\x1B[8~" => Ok(Input::End),
        buf if len == 1 && buf[0] >= 0x20 && buf[0] < 0x7F => Ok(Input::Utf8(buf[0] as char)),
        _ => Err(format!(
            "Unhandled input. Length = {}, Buf = {:?}",
            len,
            &buf[0..len]
        )),
    }
}

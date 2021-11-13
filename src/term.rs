use crate::term_seq::{AlternativeScreen, Goto, NormalScreen};
use libc::termios as Termios;
use std::io;
use std::mem;
use std::ops::Deref;
use std::ops::DerefMut;

pub struct Term<W>
where
    W: io::Write,
{
    output: W,
    _raw: Termios,
    original: Termios,
}

impl<W: io::Write> Term<W> {
    pub fn new(mut output: W) -> io::Result<Self> {
        let original = get_attr()?;

        // TODO: Use set_attr on the output variable instead of STDOUT_FILENO
        let mut raw = original;
        make_raw(&mut raw);
        set_attr(&raw)?;

        write!(output, "{}{}", AlternativeScreen, Goto(1, 1))?;
        output.flush()?;

        Ok(Self {
            output,
            _raw: raw,
            original,
        })
    }
}

impl<W: io::Write> Deref for Term<W> {
    type Target = W;

    fn deref(&self) -> &Self::Target {
        &self.output
    }
}

impl<W: io::Write> DerefMut for Term<W> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.output
    }
}

impl<W: io::Write> Drop for Term<W> {
    fn drop(&mut self) {
        let _ = write!(self.output, "{}", NormalScreen);
        let _ = set_attr(&self.original);
    }
}

fn get_attr() -> io::Result<Termios> {
    unsafe {
        let mut termios: Termios = mem::zeroed();
        if libc::tcgetattr(libc::STDOUT_FILENO, &mut termios) == 0 {
            Ok(termios)
        } else {
            Err(std::io::Error::last_os_error())
        }
    }
}

fn set_attr(termios: &Termios) -> io::Result<()> {
    unsafe {
        if libc::tcsetattr(libc::STDOUT_FILENO, libc::TCSANOW, termios) == 0 {
            Ok(())
        } else {
            Err(std::io::Error::last_os_error())
        }
    }
}

// NOTE: cfmakeraw disables control characters
// TOOD: Handle control characters? :|
fn make_raw(termios: &mut Termios) {
    unsafe {
        libc::cfmakeraw(termios);
    }
}

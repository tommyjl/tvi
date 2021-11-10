use libc::termios as Termios;
use std::io;
use std::mem;

pub struct Term {
    _raw: Termios,
    original: Termios,
}

impl Term {
    pub fn new() -> io::Result<Self> {
        let original = get_attr()?;

        let mut raw = original;
        make_raw(&mut raw);
        set_attr(&raw)?;

        Ok(Self {
            _raw: raw,
            original,
        })
    }
}

impl Drop for Term {
    fn drop(&mut self) {
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

use std::os::unix::io::AsRawFd;
use std::{env, fs, io};

/// Is this stream a TTY?
pub fn is_tty<T: AsRawFd>(stream: &T) -> bool {
    if let Ok(fd) = libredox::call::dup(stream.as_raw_fd() as _, b"termios") {
        let _ = libredox::call::close(fd);
        true
    } else {
        false
    }
}

/// Get the TTY device.
///
/// This allows for getting stdio representing _only_ the TTY, and not other streams.
pub fn get_tty() -> io::Result<fs::File> {
    let tty = env::var("TTY").map_err(|x| io::Error::new(io::ErrorKind::NotFound, x))?;
    fs::OpenOptions::new().read(true).write(true).open(tty)
}

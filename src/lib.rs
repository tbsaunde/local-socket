extern crate windows_named_pipe;

#[cfg(windows)]
use windows_named_pipe::*;
#[cfg(unix)] use std::os::unix::net::*;
use std::path::Path;
use std::io::{Result, Read, Write};

#[cfg(windows)]
pub struct LocalStream {
    stream: PipeStream
}

#[cfg(unix)]
pub struct LocalStream {
    stream: UnixStream
}

impl LocalStream {
    #[cfg(unix)]
    pub fn connect<P: AsRef<Path>>(path: P) -> Result<LocalStream>
    {
        match UnixStream::connect(path) {
            Ok(socket) => Ok(LocalStream{ stream: socket }),
            Err(x) => Err(x)
        }
    }

#[cfg(windows)]
    pub fn connect<P: AsRef<Path>>(path: P) -> Result<LocalStream>
    {
        match PipeStream::connect(path) {
            Ok(pipe) => Ok(LocalStream{ stream: pipe }),
            Err(x) => Err(x)
        }
    }
}

impl Read for LocalStream {
    fn read(&mut self, buf: &mut [u8]) -> Result<usize>
    {
        self.stream.read(buf)
    }
}

impl Write for LocalStream
{
    fn write(&mut self, buf: &[u8]) -> Result<usize>
    {
        self.stream.write(buf)
    }

    fn flush(&mut self) -> Result<()>
    {
        self.stream.flush()
    }
}

#[cfg(unix)]
#[derive(Debug)]
pub struct LocalListener {
    listener: UnixListener
}

#[cfg(windows)]
pub struct LocalListener {
    listener: PipeListener
}

impl LocalListener {
    #[cfg(unix)]
    pub fn bind<P: AsRef<Path>>(path: P) -> Result<Self>
    {
        match UnixListener::bind(path) {
            Ok(listener) => Ok(LocalListener{ listener: listener }),
            Err(x) =>Err(x)
        }
    }

    #[cfg(windows)]
    pub fn bind<P: AsRef<Path>>(path: P) -> Result<Self>
    {
        match PipeListener::bind(path) {
            Ok(l) => Ok(LocalListener{ listener: l }),
                Err(x) => Err(x)
        }
    }

    #[cfg(unix)]
    pub fn accept(&mut self) -> Result<LocalStream>
    {
        match self.listener.accept() {
            Ok((stream, _)) => Ok(LocalStream{ stream: stream }),
            Err(x) => Err(x)
        }
    }

    #[cfg(windows)]
    pub fn accept(&mut self) -> Result<LocalStream>
    {
        match self.listener.accept() {
            Ok(stream) => Ok(LocalStream{ stream: stream }),
            Err(x) => Err(x)
        }
    }

    pub fn incoming<'a>(&'a mut self) -> Incoming<'a>
    {
        Incoming{ listener: self }
    }
}

pub struct Incoming<'a>
{
    listener: &'a mut LocalListener
}

impl<'a> IntoIterator for &'a mut LocalListener
{
    type Item = Result<LocalStream>;
    type IntoIter = Incoming<'a>;

    fn into_iter(self) -> Incoming<'a>
    {
        self.incoming()
    }
}

impl<'a> Iterator for Incoming<'a>
{
    type Item = Result<LocalStream>;

    fn next(&mut self) -> Option<Result<LocalStream>>
    {
        Some(self.listener.accept())
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
    }
}

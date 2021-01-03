#![allow(dead_code)]
#![allow(unused_must_use)]


use std::path::{PathBuf, Path};
use nix::sys::stat::Mode;
use std::io::prelude::*;

/// Attempt to create a new Unix named pipe/FIFO on disk.
fn create_pipe<P: ?Sized + nix::NixPath>(path: &P, mode: Option<Mode>) -> nix::Result<()> {
    nix::unistd::mkfifo(path, mode.unwrap_or_else(|| Mode::from_bits_truncate(0o660)))
}

/// Attempt to delete a Unix named pipe/FIFO from disk.
async fn remove_pipe<P: AsRef<Path>>(path: P) -> tokio::io::Result<()> {
    tokio::fs::remove_file(&path).await
}

/// Represents a path to a Unix named pipe (FIFO).
///
/// Provides convenience methods to create readers and writers, as well as an
/// easy way to ensure the pipe actually exists.
#[derive(Clone)]
pub struct Pipe {
    inner: PathBuf,
}

impl Pipe {
    /// Wraps a given path in a `Pipe`.
    pub fn new<T: Into<PathBuf>>(path: T) -> Self {
        Self { inner: path.into() }
    }
    /// Checks if the path exists.
    pub fn exists(&self) -> bool {
        self.inner.exists()
    }
    /// Ensures the path exists, creating a named pipe in its place if it doesn't.
    pub fn ensure_exists(&self) -> nix::Result<()> {
        if !self.exists() {
            create_pipe(&self.inner, None)
        } else {
            Ok(())
        }
    }
    /// Tries to delete the pipe from disk and consumes the `NamedPipe`.
    pub async fn delete(self) -> tokio::io::Result<()> {
        if self.inner.exists() {
            remove_pipe(&self.inner).await
        } else {
            Ok(())
        }
    }

    /// Creates a reader for this named pipe.
    pub fn open_read(&self) -> Reader {
        Reader::from_path(self)
    }
    /// Creates a writer for this named pipe.
    pub fn open_write(&self) -> Writer {
        Writer::from_path(self)
    }
}

/// A convenience wrapper for reading from Unix named pipes.
pub struct Reader {
    path: Pipe,
}

impl Reader {
    /// Creates a new reader, cloning the given Pipe.
    pub fn from_path(source: &Pipe) -> Self {
        Self {
            path: source.clone(),
        }
    }
    /// Checks if the named pipe actually exists and tries to create it if it doesn't.
    pub fn ensure_pipe_exists(&self) -> nix::Result<&Self> {
        self.path.ensure_exists()?;
        Ok(self)
    }
    /// Reads all bytes from the pipe no asyc.
    pub fn read(&self) -> std::io::Result<Vec<u8>> {
        std::fs::read(&self.path.inner)
    }
    /// Reads all bytes from the pipe.
    /// The returned Future will resolve when something is written to the pipe.
    pub async fn read_async(&self) -> tokio::io::Result<Vec<u8>> {
        tokio::fs::read(&self.path.inner).await
    }
    /// Reads a String from the pipe no async.
    /// The returned Future will resolve when something is written to the pipe.
    pub fn string(&self) -> std::io::Result<String> {
        std::fs::read_to_string(&self.path.inner)
    }
    /// Reads a String from the pipe.
    /// The returned Future will resolve when something is written to the pipe.
    pub async fn string_async(&self) -> tokio::io::Result<String> {
        tokio::fs::read_to_string(&self.path.inner).await
    }
}

/// A convenience wrapper for writing to Unix named pipes.
pub struct Writer {
    path: Pipe,
}

impl Writer {
    async fn _write(&self, data: &[u8]) -> tokio::io::Result<()> {
        use tokio::io::AsyncWriteExt;
        let mut file = tokio::fs::OpenOptions::new()
            .write(true)
            .create(false)
            .open(&self.path.inner)
            .await?;
        file.write_all(data).await
    }
    pub fn from_path(source: &Pipe) -> Self {
        Self {
            path: source.clone(),
        }
    }
    /// Checks if the named pipe actually exists and tries to create it if it doesn't.
    pub fn ensure_pipe_exists(&self) -> nix::Result<&Self> {
        self.path.ensure_exists()?;
        Ok(self)
    }
    /// Writes byte data to the pipe.
    pub fn write(&self, data: &[u8]) -> std::io::Result<()> {
        let mut buffer = std::fs::File::create(&self.path.inner.to_str().unwrap())?;
        buffer.write(data)?;
        Ok(())
    }
    /// Writes byte data to the pipe.
    /// The returned Future will resolve when the bytes are read from the pipe.
    pub async fn write_async(&self, data: &[u8]) -> tokio::io::Result<()> {
        self._write(data).await
    }
    /// Writes &str data to the pipe.
    pub fn string(&self, data: String) -> std::io::Result<()> {
        let mut buffer = std::fs::File::create(&self.path.inner.to_str().unwrap())?;
        buffer.write(data.as_bytes())?;
        Ok(())
    }
    /// Writes &str data to the pipe.
    /// The returned Future will resolve when the string is read from the pipe.
    pub async fn string_async(&self, data: &str) -> tokio::io::Result<()> {
        self._write(data.as_bytes()).await
    }
}

// #[cfg(test)]
// mod tests {
//     use tokio::{task::{self, block_on}, io};
//     #[test]
//     fn write_and_read_threaded() -> io::Result<()> {
//         use std::thread;
//         let pipe = super::Pipe::new("./test_pipe_3");
//         pipe.ensure_exists().unwrap();
//         let writer = pipe.open_write();
//         let reader = pipe.open_read();
//         let data_to_send = "Hello pipe";
//         let t_write = thread::spawn(move || block_on(writer.write_str(data_to_send)));
//         let t_read = thread::spawn(move || block_on(reader.read_string()));
//         t_write.join().unwrap()?;
//         let read_result = t_read.join().unwrap()?;
//         assert_eq!(read_result, data_to_send);
//         block_on(pipe.delete())
//     }
//     #[test]
//     fn write_and_read_async() -> io::Result<()> {
//         block_on(async {
//             let pipe = super::Pipe::new("./test_pipe_4");
//             pipe.ensure_exists().unwrap();
//             let writer = pipe.open_write();
//             let reader = pipe.open_read();
//             let data_to_send = "Hello pipe";
//             let t1 = task::spawn(async move { writer.write_str(data_to_send).await });
//             let t2 = task::spawn(async move { reader.read_string().await });
//             t1.await?;
//             let read_result = t2.await?;
//             assert_eq!(read_result, data_to_send);
//             pipe.delete().await
//         })
//     }
//     #[test]
//     fn ensure_on_write() -> io::Result<()> {
//         block_on(async {
//             let pipe = super::Pipe::new("./test_pipe_5");
//             pipe.ensure_exists().unwrap();
//             let writer = pipe.open_write();
//             let reader = pipe.open_read();
//             let data_to_send = "Hello pipe";
//             let t1 = task::spawn(async move {
//                 writer
//                     .ensure_pipe_exists()
//                     .unwrap()
//                     .write_str(data_to_send)
//                     .await
//             });
//             let t2 = task::spawn(async move { reader.read_string().await });
//             t1.await?;
//             let read_result = t2.await?;
//             assert_eq!(read_result, data_to_send);
//             pipe.delete().await
//         })
//     }
//     #[test]
//     fn ensure_on_read() -> io::Result<()> {
//         block_on(async {
//             let pipe = super::Pipe::new("./test_pipe_6");
//             pipe.ensure_exists().unwrap();
//             let writer = pipe.open_write();
//             let reader = pipe.open_read();
//             let data_to_send = "Hello pipe";
//             let t1 = task::spawn(async move { writer.write_str(data_to_send).await });
//             let t2 =
//                 task::spawn(
//                     async move { reader.ensure_pipe_exists().unwrap().read_string().await },
//                 );
//             t1.await?;
//             let read_result = t2.await?;
//             assert_eq!(read_result, data_to_send);
//             pipe.delete().await
//         })
//     }
// }

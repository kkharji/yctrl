use anyhow::{bail, Context, Result};
use serde::de::DeserializeOwned;
use std::{
    env,
    fmt::Debug,
    io::{Read, Write},
    os::unix::net::UnixStream,
};

pub struct Socket {
    socket_path: String,
}

impl Socket {
    pub fn new() -> Result<Self> {
        let user = env::var("USER")?;
        let socket_path = format!("/tmp/yabai_{user}.socket");
        Ok(Self { socket_path })
    }

    /// Send given arguments to yabai and return a stream for further processing
    fn send<A: AsRef<[u8]>>(self: &Self, args: &[A]) -> Result<UnixStream> {
        let mut stream = UnixStream::connect(&self.socket_path)?;
        stream.set_nonblocking(false)?;
        for arg in args.iter().map(AsRef::as_ref) {
            if arg.contains(&b'\0') {
                bail!("Internal: Unexpected NUL byte in arg: {arg:?}");
            }
            stream.write_all(arg)?;
            stream.write_all(b"\0")?;
        }
        stream.write_all(b"\0")?;
        stream.flush()?;
        Ok(stream)
    }

    /// Send request to yabai socket and return string.
    pub fn request<A: AsRef<[u8]> + Debug>(self: &Self, args: &[A]) -> Result<String> {
        let mut stream = self.send(args)?;
        let mut buf = Vec::new();
        stream.read_to_end(&mut buf)?;

        if buf.get(0) == Some(&7) {
            anyhow::bail!(
                "Yabai: {} {:?}",
                String::from_utf8_lossy(&buf[1..]).trim(),
                args
            );
        }

        String::from_utf8(buf).map_err(anyhow::Error::new)
    }

    /// Send request to yabai socket and ignore response unless it is an error response.
    pub fn execute<A: AsRef<[u8]> + Debug>(self: &Self, args: &[A]) -> Result<()> {
        let mut buf = [0; 1];
        let mut stream = self.send(args)?;

        // Ingore overflow error
        stream.read_exact(&mut buf).ok();

        if buf.get(0) != Some(&7) {
            Ok(())
        } else {
            bail!("Yabai: fail to execute {:?}", args)
        }
    }

    pub fn query<T, A>(self: &Self, args: &[A]) -> Result<T>
    where
        T: DeserializeOwned,
        A: AsRef<[u8]> + Debug,
    {
        loop {
            // NOTE: According to @slam, sometime queries return empty string.
            let raw = self.request(args)?;
            if raw.is_empty() {
                eprintln!("{:?} returned an empty string, retrying", args);
                continue;
            }
            return serde_json::from_str(&raw)
                .with_context(|| format!("Failed to desrialize JSON: {raw}"));
        }
    }
}

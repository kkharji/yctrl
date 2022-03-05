use anyhow::{bail, Context, Result};
use serde::de::DeserializeOwned;
use std::env;
use std::fmt::Debug;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::UnixStream;

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
    async fn send<A: AsRef<[u8]>>(self: &Self, args: &[A]) -> Result<UnixStream> {
        let mut stream = UnixStream::connect(&self.socket_path).await?;
        stream.writable().await?;

        for arg in args.iter().map(AsRef::as_ref) {
            if arg.contains(&b'\0') {
                bail!("Internal: Unexpected NUL byte in arg: {arg:?}");
            }
            stream.write_all(arg).await?;
            stream.write_all(b"\0").await?;
        }
        stream.write_all(b"\0").await?;
        stream.flush().await?;
        Ok(stream)
    }

    /// Send request to yabai socket and return string.
    pub async fn request<A: AsRef<[u8]> + Debug>(self: &Self, args: &[A]) -> Result<String> {
        let mut stream = self.send(args).await?;
        let mut buf = Vec::new();

        // Wait till the stream become readable.
        stream.readable().await?;
        // Read till EOF
        stream.read_to_end(&mut buf).await?;
        // Check if yabai errored
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
    pub async fn execute<A: AsRef<[u8]> + Debug>(self: &Self, args: &[A]) -> Result<()> {
        let mut buf = [0; 1];
        let mut stream = self.send(args).await?;
        // Wait till the stream become readable
        stream.readable().await?;
        // Ignore overflow error
        stream.read_exact(&mut buf).await.ok();
        // Check for error code
        if buf.get(0) != Some(&7) {
            Ok(())
        } else {
            bail!("Yabai: fail to execute {:?}", args)
        }
    }

    pub async fn query<T, A>(self: &Self, args: &[A]) -> Result<T>
    where
        T: DeserializeOwned,
        A: AsRef<[u8]> + Debug,
    {
        loop {
            // NOTE: According to @slam, sometime queries return empty string.
            let raw = self.request(args).await?;
            if raw.is_empty() {
                eprintln!("{:?} returned an empty string, retrying", args);
                continue;
            }
            return serde_json::from_str(&raw)
                .with_context(|| format!("Failed to desrialize JSON: {raw}"));
        }
    }
}

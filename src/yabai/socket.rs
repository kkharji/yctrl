use super::models::{Space, Window};
use anyhow::{bail, Context, Result};
use serde::de::DeserializeOwned;
use std::env;
use std::fmt::Debug;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::UnixStream;

const QUERY_GET_SPACE_WINDOWS: &[&str; 3] = &["query", "--windows", "--space"];
const QUERY_GET_FOCUSED_WINDOW: &[&str; 3] = &["query", "--windows", "--window"];
const QUERY_GET_FOCUSED_SPACE: &[&str; 3] = &["query", "--spaces", "--space"];
const QUERY_GET_ALL_WINDOWS: &[&str; 2] = &["query", "--windows"];
const QUERY_GET_ALL_SPACES: &[&str; 2] = &["query", "--spaces"];

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
        let mut command = Vec::from([0x0, 0x0, 0x0, 0x0]);
        for arg in args.iter().map(AsRef::as_ref) {
            if arg.contains(&0x0) {
                bail!("Internal: Unexpected NUL byte in arg: {arg:?}");
            }
            command.extend_from_slice(arg);
            command.push(0x0)
        }

        command.push(0x0);
        command[0] = (command.len() - 4) as u8;

        stream.write_all(&command).await?;
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

    pub async fn focused_space(self: &Self) -> Result<Space> {
        self.query::<Space, _>(QUERY_GET_FOCUSED_SPACE).await
    }

    pub async fn focused_window(self: &Self) -> Result<Window> {
        self.query::<Window, _>(QUERY_GET_FOCUSED_WINDOW).await
    }
    pub async fn last_window(&self) -> Result<Window> {
        self.query::<Window, _>(&["query", "--windows", "--window", "last"])
            .await
    }

    pub async fn spaces(self: &Self, _display: &str) -> Result<Vec<Space>> {
        // reserved for current display/all displays
        self.query::<Vec<Space>, _>(QUERY_GET_ALL_SPACES).await
    }

    pub async fn windows(self: &Self, space: &str) -> Result<Vec<Window>> {
        let windows = if space == "current" {
            self.query::<Vec<Window>, _>(QUERY_GET_SPACE_WINDOWS)
                .await?
        } else if space == "all" {
            self.query::<Vec<Window>, _>(QUERY_GET_ALL_WINDOWS).await?
        } else {
            self.query::<Vec<Window>, _>(&["query", "--windows", "--space", space])
                .await?
        };
        Ok(windows
            .into_iter()
            .filter(|w| w.subrole != "AXUnknown.Hammerspoon" && !w.is_minimized && !w.is_hidden)
            .collect())
    }

    pub async fn window_by_id(self: &Self, space: &str, id: &u32) -> Result<Option<Window>> {
        let windows = if space == "current" {
            self.query::<Vec<Window>, _>(QUERY_GET_SPACE_WINDOWS)
                .await?
        } else if space == "all" {
            self.query::<Vec<Window>, _>(QUERY_GET_ALL_WINDOWS).await?
        } else {
            self.query::<Vec<Window>, _>(&["query", "--windows", "--space", space])
                .await?
        };
        Ok(windows.into_iter().find(|w| &w.id == id))
    }
}

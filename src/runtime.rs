use anyhow::{bail, Context, Result};
use std::fs;
use tokio::net::{UnixListener, UnixStream};

use crate::yabai;
use tokio::io::AsyncReadExt;
use tokio::time::Instant;
use tracing_subscriber::fmt::format::FmtSpan;
use tracing_subscriber::EnvFilter;

pub struct Runtime {}

impl Runtime {
    pub async fn start() -> Result<()> {
        configure_tracing_subscriber()?;
        let socket_path = "/tmp/yctrl.socket";

        if fs::metadata(socket_path).is_ok() {
            fs::remove_file(socket_path).with_context(|| {
                format!("could not delete previous socket at {:?}", socket_path)
            })?;
        }

        let listener = UnixListener::bind(socket_path)?;
        tracing::info!("Listening on {socket_path}");
        loop {
            let (stream, _) = listener.accept().await?;
            tokio::spawn(async move {
                if let Err(e) = Self::handle(stream).await {
                    tracing::error!("{:?}", e);
                }
            });
        }
    }

    async fn handle(mut s: UnixStream) -> Result<()> {
        tracing::trace!("Handling new request .. ");
        let now = Instant::now();
        let mut response = String::default();

        s.read_to_string(&mut response).await?;

        let mut arguments: Vec<&str> = response.trim().split_whitespace().collect();

        // Get Request type
        let rtype: &str = arguments.remove(0);

        if rtype == "event" {
            // Parse event
            let event = yabai::Event::try_from(arguments)?;
            tracing::debug!("{:?}", event);
        } else {
            bail!("Request type: '{rtype}' is not supported.")
        }

        let elapsed_time = now.elapsed();
        tracing::trace!(
            "Request handled in {} microseconds ..",
            elapsed_time.subsec_micros()
        );

        Ok(())
    }
}

fn configure_tracing_subscriber() -> Result<()> {
    // Configure tracing_subscriber
    tracing_subscriber::fmt()
        // Filter what traces are displayed based on RUST_LOG var: `RUST_LOG=chat=trace`
        .with_env_filter(EnvFilter::from_default_env())
        // Log events when `tracing` spans are created, entered, existed, or closed.
        .with_span_events(FmtSpan::FULL)
        // Set this subscriber as the default, to collect all traces emitted by the programmer.
        .init();

    Ok(())
}

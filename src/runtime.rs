use crate::state::{SharedState, State};
use crate::yabai::Event;
use anyhow::{bail, Context, Ok, Result};
use async_trait::async_trait;
use rand::rngs::StdRng;
use rand::{Rng, SeedableRng};
use std::fs;
use std::sync::Arc;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::{UnixListener, UnixStream};
use tokio::sync::Mutex;
use tracing::Level;
use tracing_subscriber::fmt::format::FmtSpan;
use tracing_subscriber::EnvFilter;
mod space_event;
mod window_event;

const SOCKET_PATH: &str = "/tmp/yctrl.socket";

#[async_trait]
pub trait EventHandler {
    async fn handle(&self, state: SharedState) -> Result<()>;
}

#[async_trait]
impl EventHandler for Event {
    async fn handle(&self, state: SharedState) -> Result<()> {
        match self {
            Event::Window(e) => e.handle(state).await,
            Event::Space(s) => s.handle(state).await,
            _ => {
                bail!("{:?} is not supported.", self)
            }
        }
    }
}

pub async fn start() -> Result<()> {
    let state = Arc::new(Mutex::new(State::default()));

    configure_tracing_subscriber()?;

    if fs::metadata(SOCKET_PATH).is_ok() {
        fs::remove_file(SOCKET_PATH)
            .with_context(|| format!("could not delete previous socket at {:?}", SOCKET_PATH))?;
    }

    let listener = UnixListener::bind(SOCKET_PATH)?;
    tracing::info!("Listening on {SOCKET_PATH}");
    loop {
        let (stream, _) = listener.accept().await?;
        let state = state.clone();
        tokio::spawn(async move {
            if let Err(e) = handle(stream, state).await {
                tracing::error!("{:?}", e);
            }
        });
    }
}

async fn handle(mut s: UnixStream, state: SharedState) -> Result<()> {
    let mut rng = StdRng::from_entropy();
    let id: u32 = rng.gen_range(222..999);

    let mut response = String::default();

    s.read_to_string(&mut response).await?;

    let mut arguments: Vec<&str> = response.trim().split_whitespace().collect();

    // Get Request type
    let rtype: &str = arguments.remove(0);

    let span = tracing::trace_span!("Request", "[{}]", id);

    match rtype {
        "event" => {
            let event = Event::try_from(arguments)?;
            tracing::event!(parent: &span, Level::DEBUG, "{}", event);
            event.handle(state).await
        }
        "config" => {
            tracing::event!(parent: &span, Level::DEBUG, "Updating configuration");
            state.lock().await.handle(arguments)?;

            tracing::event!(
                parent: &span,
                Level::DEBUG,
                "New Configuration: {:#?}",
                state.lock().await.config
            );
            return Ok(());
        }
        _ => {
            bail!("Request type: '{rtype}' is not supported.")
        }
    }
}

fn configure_tracing_subscriber() -> Result<()> {
    // Configure tracing_subscriber
    tracing_subscriber::fmt()
        // Filter what traces are displayed based on RUST_LOG var: `RUST_LOG=chat=trace`
        .with_env_filter(EnvFilter::from_default_env())
        // Log events when `tracing` spans are created, entered, existed, or closed.
        .with_span_events(FmtSpan::CLOSE)
        .with_target(false)
        // Set this subscriber as the default, to collect all traces emitted by the programmer.
        .init();

    Ok(())
}

/// Execute argument in the runtime
pub async fn execute(args: &Vec<String>) -> Result<()> {
    let mut stream = UnixStream::connect(SOCKET_PATH).await?;
    stream.writable().await?;
    stream.write_all(args.join(" ").as_str().as_ref()).await?;
    stream.flush().await?;
    Ok(())
}

use anyhow::{anyhow, bail, Context, Result};
use std::{
    fs,
    io::Read,
    os::unix::net::{UnixListener, UnixStream},
    time::{SystemTime, UNIX_EPOCH},
};

use crate::yabai;

pub struct EventLoop {
    // TODO: add state
}

impl EventLoop {
    pub fn start() -> Result<()> {
        let socket_path = "/tmp/yctrl.socket";
        if fs::metadata(socket_path).is_ok() {
            fs::remove_file(socket_path).with_context(|| {
                format!("could not delete previous socket at {:?}", socket_path)
            })?;
        }

        let listener = UnixListener::bind(socket_path)?;
        let mut buffer = Vec::new();

        for stream in listener.incoming() {
            match stream {
                Ok(stream) => {
                    if let Err(e) = Self::handle(&mut buffer, stream) {
                        println!("{}", e)
                    }
                    buffer.clear();
                }
                Err(e) => {
                    bail!("Error: {e}");
                }
            };
        }

        Ok(())
    }

    fn handle(mut buffer: &mut Vec<u8>, mut s: UnixStream) -> Result<()> {
        // Read stream to buf
        s.read_to_end(&mut buffer)?;
        // Remove of we got newline.
        if buffer.last() == Some(&10) {
            buffer.pop();
        }
        // Get current timestamp
        let timestamp = SystemTime::now().duration_since(UNIX_EPOCH)?.as_secs();
        // Parse event
        let event =
            yabai::Event::try_from(buffer).map_err(|e| anyhow!("{timestamp}: ERROR({:?})", e,))?;
        // log received event
        println!("{timestamp}: Event Received: {:?}", event);

        Ok(())

        // let now = Instant::now();
        // event.handle()?
        // let elapsed_time = now.elapsed();
        // println!(
        //     "Handling incomming event took {} seconds.",
        //     elapsed_time.subsec_millis()
        // );
    }
}

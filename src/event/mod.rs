pub mod types;

use self::types::Event;
use anyhow::{Context, Result};
use std::{
    fs,
    io::Read,
    os::unix::net::UnixListener,
    str,
    time::{SystemTime, UNIX_EPOCH},
};

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
        loop {
            let (mut s, _) = listener.accept()?;
            let mut buf = Vec::new();
            s.read_to_end(&mut buf)?;

            // Remove of we got newline.
            if buf.last() == Some(&10) {
                buf.pop();
            }

            // Parse Event
            let timestamp = SystemTime::now().duration_since(UNIX_EPOCH)?.as_secs();
            let result = Event::try_from(&buf);
            if let Err(result) = result {
                let message = str::from_utf8(&buf).unwrap_or("(ERROR: Unable to read message).");
                println!("{timestamp}: Error: Message(\"{message}\"). ERROR({result})");
                continue;
            }

            let event = result.unwrap();
            println!("{timestamp}: Event Received: {:?}", event);

            // let now = Instant::now();
            // event.handle()?
            // let elapsed_time = now.elapsed();
            // println!(
            //     "Handling incomming event took {} seconds.",
            //     elapsed_time.subsec_millis()
            // );
        }
    }
}

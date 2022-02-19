use self::models::{YabaiSpace, YabaiWindow};
use anyhow::{bail, Context, Result};
use serde::de::DeserializeOwned;
use std::{
    env,
    fmt::Debug,
    io::{Read, Write},
    os::unix::net::UnixStream,
    path::Path,
};
mod models;

/// Send request to yabai socket.
pub fn request<A, P>(socket_path: P, args: &[A]) -> Result<String>
where
    A: AsRef<[u8]> + Debug,
    P: AsRef<Path>,
{
    let mut stream = UnixStream::connect(socket_path)?;
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

    let mut buf = Vec::new();
    stream.read_to_end(&mut buf)?;

    if buf.get(0) == Some(&7) {
        anyhow::bail!(
            "Yabai: {} {:?}",
            String::from_utf8_lossy(&buf[1..]).trim(),
            args.to_owned()
        );
    }

    Ok(String::from_utf8(buf)?)
}

pub fn query<T, A, P>(socket_path: P, args: &[A]) -> Result<T>
where
    T: DeserializeOwned,
    A: AsRef<[u8]> + Debug,
    P: AsRef<Path>,
{
    loop {
        let raw = request(&socket_path, args)?;
        // NOTE: According to @slam, sometime queries return empty string.
        if raw.is_empty() {
            eprintln!("{:?} returned an empty string, retrying", args);
            continue;
        }
        return serde_json::from_str(&raw)
            .with_context(|| format!("Failed to desrialize JSON: {raw}"));
    }
}

enum CommandScope {
    Window,
    Space,
    Display,
}

impl CommandScope {
    pub fn new(str: &str) -> Result<Self> {
        Ok(match str {
            "window" => CommandScope::Window,
            "space" => CommandScope::Space,
            "display" => CommandScope::Display,
            _ => bail!("Unexpected command scope"),
        })
    }
}

fn main() -> Result<()> {
    let mut args: Vec<String> = env::args().collect();
    let user = env::var("USER")?;
    let socket_path = format!("/tmp/yabai_{user}.socket");

    // Remove caller from argument.
    args.remove(0);
    if args.len() < 3 {
        bail!("Not enough arguments provided.")
    }

    // Fix when the user provided id for sub command.
    let mut command_pos = 1;
    if let Ok(_) = args[1].as_str().parse::<u32>() {
        command_pos = 2;
    }

    // Mutate command to correct format.
    if let Some(cmd) = args.get_mut(command_pos) {
        *cmd = format!("--{cmd}");
    }

    match CommandScope::new(args[0].as_str())? {
        CommandScope::Window => Window::handle_request(socket_path, args),
        CommandScope::Space => Space::handle_request(socket_path, args),
        _ => request(&socket_path, &args).map(|_| ()),
    }
}

struct Window();
impl Window {
    fn handle_request(socket_path: String, args: Vec<String>) -> Result<()> {
        let select = args.last().unwrap().as_str();
        let command = args[1].clone();

        // Only further process these commands.
        if &command != "--focus" && &command != "--swap" && &command != "--warp" {
            println!("can't further process '{command}' redirecting to yabai socket");
            return request(&socket_path, &args).map(|_| ());
        }

        // Only further process next/prev, if not run the command as it.
        if select != "next" && select != "prev" {
            println!("got {select} redirecting to yabai socket");
            return request(socket_path, &args).map(|_| ());
        }

        // See if next/prev just works before doing anything else.
        if request(&socket_path, &args).is_ok() {
            println!("successfully ran {select} through yabai socket");
            return Ok(());
        }

        // Get current space information.
        let space = query::<YabaiSpace, _, _>(&socket_path, &["query", "--spaces", "--space"])?;

        // Should just change focus to next space window
        // TODO: move to next space too and delete current space???
        if space.first_window == space.last_window && &command == "--focus" {
            let windows =
                query::<Vec<YabaiWindow>, _, _>(&socket_path, &["query", "--windows", "--space"])?
                    .into_iter()
                    .filter(|w| {
                        w.subrole != "AXUnknown.Hammerspoon" && w.is_visible && !w.has_focus
                    })
                    .collect::<Vec<YabaiWindow>>();

            if windows.len() == 0 {
                println!(
                    "No windows left in current space, trying {select} space instead of window"
                );
                return Space::handle_request(
                    socket_path,
                    vec!["space".to_string(), command, select.to_string()],
                );
            } else {
                return request(
                    socket_path,
                    &["window", &command, &windows.first().unwrap().id.to_string()],
                )
                .map(|_| ());
            }
        }

        // Get Id based on whether the select value.
        let id = if select == "next" {
            space.first_window.to_string()
        } else {
            space.last_window.to_string()
        };

        println!("{select} window isn't found, trying to foucs {id}");

        // Finally, Try to focus by id or else focus to first window
        request(&socket_path, &["window", &command, &id])
            .or_else(|_| request(socket_path, &["window", &command, "first"]))
            .map(|_| ())
    }
}

struct Space();
impl Space {
    fn handle_request(socket_path: String, args: Vec<String>) -> Result<()> {
        request(&socket_path, &args).map(|_| ())
    }
}

use self::models::{YabaiSpace, YabaiWindow};
use crate::constants::QUERY_SPACE_WINDOWS;
use anyhow::{bail, Context, Result};
use serde::de::DeserializeOwned;
use std::{
    env,
    fmt::Debug,
    io::{Read, Write},
    os::unix::net::UnixStream,
};
mod constants;
mod models;

/// Send request to yabai socket.
pub fn request<A>(socket_path: &str, args: &[A]) -> Result<String>
where
    A: AsRef<[u8]> + Debug,
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

pub fn query<T, A>(socket_path: &str, args: &[A]) -> Result<T>
where
    T: DeserializeOwned,
    A: AsRef<[u8]> + Debug,
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

fn should_just_redirect<A>(cmd: &str, _args: &[A]) -> bool
where
    A: AsRef<[u8]> + Debug,
{
    return cmd != "focus" && cmd != "swap" && cmd != "move" && cmd != "warp";
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
    let command = args.get_mut(command_pos).unwrap();
    let cmd = command.clone();
    *command = format!("--{command}");

    // Redirect if these isn't the command given
    if should_just_redirect(&cmd, &args) {
        println!("redircting '{:?}' to yabai socket.", args);
        return request(&socket_path, &args).map(|_| ());
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

        // Only further process next/prev, if not run the command as it.
        if select != "next" && select != "prev" {
            println!("got {select} redirecting to yabai socket");
            return request(&socket_path, &args).map(|_| ());
        }

        // See if next/prev just works before doing anything else.
        if request(&socket_path, &args).is_ok() {
            println!("successfully ran {select} through yabai socket");
            return Ok(());
        }

        // Get current space information.
        let space = query::<YabaiSpace, _>(&socket_path, &["query", "--spaces", "--space"])?;

        // Should just change focus to next space window
        // TODO: support moving window to next/prev space and delete current space empty??
        if space.first_window == space.last_window && &command == "--focus" {
            let windows = query::<Vec<YabaiWindow>, _>(&socket_path, QUERY_SPACE_WINDOWS)?
                .into_iter()
                // not sure why Hammerspoon create these windows
                .filter(|w| w.subrole != "AXUnknown.Hammerspoon" && w.is_visible && !w.has_focus)
                .collect::<Vec<YabaiWindow>>();

            if windows.len() == 0 {
                println!("No windows left in space, trying {select} space instead of window");
                let args = vec!["space".to_string(), command, select.to_string()];
                return Space::handle_request(socket_path, args);
            } else {
                let args = &["window", &command, &windows.first().unwrap().id.to_string()];
                return request(&socket_path, args).map(|_| ());
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
            .or_else(|_| request(&socket_path, &["window", &command, "first"]))
            .map(|_| ())
    }
}

struct Space();
impl Space {
    fn handle_request(socket_path: String, args: Vec<String>) -> Result<()> {
        let select = args.last().unwrap();

        // Only further process next/prev, if not run the command as it.
        if select != "next" && select != "prev" {
            println!("got {:?} ... redirecting to yabai socket", select);
            request(&socket_path, &args).map(|_| ())
        } else {
            // See if next/prev just works before doing anything else.
            request(&socket_path, &args)
                .or_else(|_| {
                    let pos = if select == "next" { "first" } else { "last" };
                    request(&socket_path, &["space", &args[1], pos])
                })
                .map(|_| ())
        }
    }
}

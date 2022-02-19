use self::models::{YabaiSpace, YabaiWindow};
use crate::constants::{QUERY_CURRENT_SPACE, QUERY_SPACE_WINDOWS};
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

/// Send request to yabai socket and return string.
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

/// Send request to yabai socket.
pub fn execute<A>(socket_path: &str, args: &[A]) -> Result<()>
where
    A: AsRef<[u8]> + Debug,
{
    let mut buf = [0; 1];
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

    // Ignore if yabai return nothing.
    stream.read_exact(&mut buf).ok();

    if buf.get(0) == Some(&7) {
        bail!("Yabai: fail to execute {:?}", args)
    }

    Ok(())
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

fn should_just_redirect<A: AsRef<[u8]> + Debug>(cmd: &str, _args: &[A]) -> bool {
    cmd != "focus"
        && cmd != "swap"
        && cmd != "move"
        && cmd != "warp"
        && cmd != "space"
        && cmd != "inc"
}

fn main() -> Result<()> {
    let mut args: Vec<String> = env::args().collect();
    let user = env::var("USER")?;
    let socket_path = format!("/tmp/yabai_{user}.socket");

    // Remove caller from argument.
    args.remove(0);
    if args.len() < 2 {
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
        return execute(&socket_path, &args);
    }

    match CommandScope::new(args[0].as_str())? {
        CommandScope::Window => Window::handle_request(socket_path, args),
        CommandScope::Space => Space::handle_request(socket_path, args),
        _ => execute(&socket_path, &args).map(|_| ()),
    }
}

struct Window();
impl Window {
    fn handle_space_subcommand(socket_path: String, args: Vec<String>) -> Result<()> {
        let select = args.last().unwrap();
        let command = args[1].clone();
        let space_args = vec!["space".to_string(), "--focus".to_string(), select.clone()];

        // Only further process next/prev, if not run the command as it.
        if select != "next" && select != "prev" {
            if execute(&socket_path, &args).is_ok() {
                return Space::handle_request(socket_path, space_args);
            };
        }

        // Try to execute as is
        if execute(&socket_path, &args).is_ok() {
            return Space::handle_request(socket_path, space_args);
        }

        // Try position rather than order
        let pos = if select == "next" { "first" } else { "last" };
        if execute(&socket_path, &["window", &command, pos]).is_ok() {
            return Space::handle_request(socket_path, space_args);
        }

        bail!("Fail handle space command!!! {:?}", args)
    }

    fn handle_inc_request(socket_path: String, args: Vec<String>) -> Result<()> {
        let dir = if args.last().unwrap() == "left" {
            "-150:0"
        } else {
            "+150:0"
        };

        execute(&socket_path, &["window", "--resize", "left:", dir])
            .or_else(|_| execute(&socket_path, &["window", "--resize", "right:", dir]))
    }

    fn handle_request(socket_path: String, args: Vec<String>) -> Result<()> {
        // Handle special cases
        match args[1].as_str() {
            "--space" => return Self::handle_space_subcommand(socket_path, args),
            "--inc" => return Self::handle_inc_request(socket_path, args),
            _ => (),
        };

        let select = args.last().unwrap().as_str();
        let command = args[1].clone();

        // Only further process next/prev, if not run the command as it.
        if select != "next" && select != "prev" {
            println!("got {select} redirecting to yabai socket");
            return execute(&socket_path, &args);
        }

        // See if next/prev just works before doing anything else.
        if execute(&socket_path, &args).is_ok() {
            println!("successfully ran {select} through yabai socket");
            return Ok(());
        }

        // Get current space information.
        let space = query::<YabaiSpace, _>(&socket_path, QUERY_CURRENT_SPACE)?;

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
                return execute(&socket_path, args);
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
        execute(&socket_path, &["window", &command, &id])
            .or_else(|_| execute(&socket_path, &["window", &command, "first"]))
    }
}

struct Space();
impl Space {
    fn handle_request(socket_path: String, args: Vec<String>) -> Result<()> {
        let select = args.last().unwrap();

        // Only further process when select != next/prev and succeeded
        if select != "next" && select != "prev" && execute(&socket_path, &args).is_ok() {
            return Ok(());
        }

        // See if next/prev just works before doing anything else.
        execute(&socket_path, &args).or_else(|_| {
            let pos = if select == "next" { "first" } else { "last" };
            execute(&socket_path, &["space", &args[1], pos])
        })
    }
}

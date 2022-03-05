mod constants;
mod runtime;
mod yabai;

use self::constants::*;
use self::runtime::Runtime;
use anyhow::{anyhow, bail, Result};
use std::env;
use std::fmt::Debug;

fn should_just_redirect<A>(cmd: &str, _args: &[A]) -> bool
where
    A: AsRef<[u8]> + Debug,
{
    cmd != "focus"
        && cmd != "swap"
        && cmd != "move"
        && cmd != "warp"
        && cmd != "space"
        && cmd != "inc"
        && cmd != "make"
}

fn main() -> Result<()> {
    let mut args: Vec<String> = env::args().skip(1).collect();
    let argc = args.len();

    if argc == 0 {
        Runtime::start().map_err(|e| anyhow!("Unable to start listener: {e}"))?
    } else if argc < 2 {
        bail!("yctrl: Not enough arguments provided.")
    }

    // Get yabai scoket path
    let yabai = yabai::Socket::new()?;

    // Fix when the user provided id for sub command.
    let mut command_pos = 1;
    if args[1].as_str().parse::<u32>().is_ok() {
        command_pos = 2;
    }

    // Correct format: Note should maybe check if it's already correct
    let command = args.get_mut(command_pos).unwrap();
    let cmd = command.clone();
    *command = format!("--{command}");

    // Check if we should just redirect to yabai scoket.
    if should_just_redirect(&cmd, &args) {
        println!("redircting '{:?}' to yabai socket.", args);
        return yabai.execute(&args);
    }

    // Handle User request
    match args[0].as_str() {
        "window" => Window::handle(&yabai, args),
        "space" => Space::handle(&yabai, args),
        _ => yabai.execute(&args),
    }
}

struct Window();
impl Window {
    fn space(yabai: &yabai::Socket, args: Vec<String>) -> Result<()> {
        let select = args.last().unwrap();
        let command = args[1].clone();
        let space_args = vec!["space".to_string(), "--focus".to_string(), select.clone()];

        // Only further process next/prev, if not run the command as it.
        if select != "next" && select != "prev" && yabai.execute(&args).is_ok() {
            return Space::handle(yabai, space_args);
        }

        // Try to execute as is
        if yabai.execute(&args).is_ok() {
            return Space::handle(yabai, space_args);
        }

        // Try position rather than order
        let pos = if select == "next" { "first" } else { "last" };
        if yabai.execute(&["window", &command, pos]).is_ok() {
            return Space::handle(yabai, space_args);
        }

        bail!("Fail handle space command!!! {:?}", args)
    }

    /// Toggle between largest and smallest window.
    /// TODO: Switch between left space and child windows
    fn master(yabai: &yabai::Socket) -> Result<()> {
        yabai
            .execute(&["window", "--warp", "first"])
            .or_else(|_| yabai.execute(&["window", "--warp", "last"]))
        // let windows: Vec<YabaiWindow> = query(&socket_path, QUERY_SPACE_WINDOWS)?;
        // let current = windows.iter().find(|w| w.has_focus).unwrap();
        // let largest = windows.iter().max_by_key(|&w| w.frame.sum()).unwrap();
        // eprintln!("largest = {:#?}", largest);
        // eprintln!("total = {:#?}", largest.frame.sum());
        // let mut partial_args = vec!["window".to_string(), "--warp".to_string()];
        // if largest.id == current.id {
        //     partial_args.push("next".to_string());
        //     Self::handle(socket_path, partial_args)
        // } else {
        //     partial_args.push(largest.id.to_string());
        //     Self::handle(socket_path, partial_args)
        // }
    }

    fn inc(yabai: &yabai::Socket, args: Vec<String>) -> Result<()> {
        let left = args.last().unwrap() == "left";
        let dir = if left { "-150:0" } else { "+150:0" };
        let args = &["window", "--resize", &format!("left:{dir}")];

        yabai.execute(args).or_else(|_| {
            let mut args = args.to_vec();
            let dir = format!("right:{dir}");
            args.insert(2, &dir);
            yabai.execute(&args)
        })
    }

    fn handle(yabai: &yabai::Socket, args: Vec<String>) -> Result<()> {
        // Handle special cases
        match (args[1].as_str(), args[2].as_str()) {
            ("--space", _) => return Self::space(yabai, args),
            ("--inc", _) => return Self::inc(yabai, args),
            ("--make", "master") => return Self::master(yabai),
            _ => (),
        };

        let select = args.last().unwrap().as_str();
        let command = args[1].clone();

        // Only further process next/prev, if not run the command as it.
        if select != "next" && select != "prev" {
            println!("got {select} redirecting to yabai socket");
            return yabai.execute(&args);
        }

        // See if next/prev just works before doing anything else.
        if yabai.execute(&args).is_ok() {
            println!("successfully ran {select} through yabai socket");
            return Ok(());
        }

        // Get current space information.
        let space: yabai::Space = yabai.query(QUERY_CURRENT_SPACE)?;

        // Should just change focus to next space window
        // TODO: support moving window to next/prev space and delete current space empty??
        if space.first_window == space.last_window && &command == "--focus" {
            let windows = yabai
                .query::<Vec<yabai::Window>, _>(QUERY_SPACE_WINDOWS)?
                .into_iter()
                // not sure why Hammerspoon create these windows
                .filter(|w| w.subrole != "AXUnknown.Hammerspoon" && w.is_visible && !w.has_focus)
                .collect::<Vec<yabai::Window>>();

            if windows.is_empty() {
                println!("No windows left in space, trying {select} space instead of window");
                let args = vec!["space".to_string(), command, select.to_string()];
                return Space::handle(yabai, args);
            } else {
                let args = &["window", &command, &windows.first().unwrap().id.to_string()];
                return yabai.execute(args);
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
        yabai
            .execute(&["window", &command, &id])
            .or_else(|_| yabai.execute(&["window", &command, "first"]))
    }
}

struct Space();
impl Space {
    fn handle(yabai: &yabai::Socket, args: Vec<String>) -> Result<()> {
        let select = args.last().unwrap();

        // Only further process when select != next/prev and succeeded
        if select != "next" && select != "prev" && yabai.execute(&args).is_ok() {
            return Ok(());
        }

        // See if next/prev just works before doing anything else.
        yabai.execute(&args).or_else(|_| {
            let pos = if select == "next" { "first" } else { "last" };
            yabai.execute(&["space", &args[1], pos])
        })
    }
}

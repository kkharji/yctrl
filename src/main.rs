mod config;
mod constants;
mod runtime;
mod scratchpad;
mod state;
mod yabai;

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

#[tokio::main]
async fn main() -> Result<()> {
    let mut args: Vec<String> = env::args().skip(1).collect();
    let argc = args.len();

    if argc == 0 {
        return runtime::start()
            .await
            .map_err(|e| anyhow!("Unable to start listener: {e}"));
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

    if args[0].as_str() != "config" && args[0].as_str() != "scratchpad" {
        // Correct format: Note should maybe check if it's already correct
        let command = args.get_mut(command_pos).unwrap();
        let cmd = command.clone();
        *command = format!("--{command}");
        // Check if we should just redirect to yabai scoket.
        if should_just_redirect(&cmd, &args) {
            println!("redircting '{:?}' to yabai socket.", args);
            return yabai.execute(&args).await;
        }
    }

    // Handle User request
    match args[0].as_str() {
        "window" => WindowService::handle(&yabai, args).await,
        "space" => SpaceService::handle(&yabai, args).await,
        "scratchpad" => runtime::execute(&args).await,
        "config" => {
            if args[1].as_str().contains("yctrl") {
                runtime::execute(&args).await
            } else {
                yabai.execute(&args).await
            }
        }
        _ => yabai.execute(&args).await,
    }
}

struct WindowService();
impl WindowService {
    async fn space(yabai: &yabai::Socket, args: Vec<String>) -> Result<()> {
        let select = args.last().unwrap();
        let command = args[1].clone();
        let space_args = vec!["space".to_string(), "--focus".to_string(), select.clone()];

        // Only further process next/prev, if not run the command as it.
        if select != "next" && select != "prev" && yabai.execute(&args).await.is_ok() {
            return SpaceService::handle(yabai, space_args).await;
        }

        // Try to execute as is
        if yabai.execute(&args).await.is_ok() {
            return SpaceService::handle(yabai, space_args).await;
        }

        // Try position rather than order
        let pos = if select == "next" { "first" } else { "last" };
        if yabai.execute(&["window", &command, pos]).await.is_ok() {
            return SpaceService::handle(yabai, space_args).await;
        }

        bail!("Fail handle space command!!! {:?}", args)
    }

    /// Toggle between largest and smallest window.
    /// TODO: Switch between left space and child windows
    async fn master(yabai: &yabai::Socket) -> Result<()> {
        let succ = yabai.execute(&["window", "--warp", "first"]).await.is_ok();
        if !succ {
            yabai.execute(&["window", "--warp", "last"]).await?
        }
        Ok(())

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

    async fn inc(yabai: &yabai::Socket, args: Vec<String>) -> Result<()> {
        let left = args.last().unwrap() == "left";
        let dir = if left { "-150:0" } else { "+150:0" };
        let args = &["window", "--resize", &format!("left:{dir}")];

        if !yabai.execute(args).await.is_ok() {
            let mut args = args.to_vec();
            let direction = format!("right:{dir}");
            args.insert(2, &direction);
            yabai.execute(&args).await?
        }
        Ok(())
    }

    async fn handle(yabai: &yabai::Socket, args: Vec<String>) -> Result<()> {
        // Handle special cases
        match (args[1].as_str(), args[2].as_str()) {
            ("--space", _) => return Self::space(yabai, args).await,
            ("--inc", _) => return Self::inc(yabai, args).await,
            ("--make", "master") => return Self::master(yabai).await,
            _ => (),
        };

        let select = args.last().unwrap().as_str();
        let command = args[1].clone();

        // Only further process next/prev, if not run the command as it.
        if select != "next" && select != "prev" {
            println!("got {select} redirecting to yabai socket");
            return yabai.execute(&args).await;
        }

        // See if next/prev just works before doing anything else.
        if yabai.execute(&args).await.is_ok() {
            println!("successfully ran {select} through yabai socket");
            return Ok(());
        }

        println!("Fail to run {select}, ... trying to determine next window");

        // Get current space information.
        let space = yabai.focused_space().await?;

        println!("Got yabai spaces");

        if space.first_window == space.last_window && &command == "--focus" {
            let windows = yabai.windows("current").await?;
            println!("{windows:#?}");
            if windows.is_empty() {
                println!("No windows left in space, trying {select} space instead of window");
                let args = vec!["space".to_string(), command, select.to_string()];
                return SpaceService::handle(yabai, args).await;
            } else if let Some(current_focused) = windows.iter().find(|w| w.has_focus) {
                if let Some(current_index) =
                    space.windows.iter().position(|&x| x == current_focused.id)
                {
                    println!("current_index: {current_index:?}");
                    let idx = if select == "next" {
                        let idx = current_index + 1;
                        if idx > space.windows.len() - 1 {
                            0
                        } else {
                            idx
                        }
                    } else {
                        if current_index == 0 {
                            space.windows.len() - 1
                        } else {
                            let idx = current_index - 1;
                            if idx > 0 {
                                idx
                            } else {
                                space.windows.len() - 1
                            }
                        }
                    };
                    if let Some(id) = space.windows.get(idx) {
                        let args = &["window", &command, &id.to_string()];
                        println!("{args:?}");
                        return yabai.execute(args).await;
                    } else {
                        eprintln!("{idx} is invalid index: {:?}", space.windows);
                    }
                };
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
        let succ = yabai.execute(&["window", &command, &id]).await.is_ok();
        if !succ {
            yabai.execute(&["window", &command, "first"]).await?
        }
        Ok(())
    }
}

struct SpaceService();
impl SpaceService {
    async fn handle(yabai: &yabai::Socket, args: Vec<String>) -> Result<()> {
        let select = args.last().unwrap();

        // Only further process when select != next/prev and succeeded
        if select != "next" && select != "prev" && yabai.execute(&args).await.is_ok() {
            return Ok(());
        }

        // See if next/prev just works before doing anything else.
        let succ = yabai.execute(&args).await.is_ok();
        if !succ {
            let pos = if select == "next" { "first" } else { "last" };
            yabai.execute(&["space", &args[1], pos]).await?
        }
        Ok(())
    }
}

use anyhow::Result;
use tokio::process::Command;

pub async fn window_hide_current() -> Result<()> {
    let script = r#"tell application "System Events" to set visible of (item 1 of (processes whose frontmost is true)) to false"#;
    Command::new("osascript")
        .arg("-e")
        .arg(script)
        .output()
        .await?;

    Ok(())
}

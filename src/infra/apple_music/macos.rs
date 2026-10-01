//! The only production Apple boundary. No Music calls at import or startup.
use super::{process, Command};
use anyhow::Result;
use std::time::Duration;

const SCRIPT: &str = include_str!("music.js");
const TIMEOUT: Duration = Duration::from_secs(8);
/// How long a launched Music gets to answer its first command.
const LAUNCH_WAIT: Duration = Duration::from_secs(5);

pub(super) struct MacClient;

impl super::dispatch::Client for MacClient {
  async fn execute(&self, operation: Command) -> Result<String> {
    let args = operation.arguments()?;
    let invoke = || {
      let mut child = tokio::process::Command::new("/usr/bin/osascript");
      child
        .args(["-l", "JavaScript", "-e", SCRIPT, "--"])
        .args(&args);
      child
    };
    let label = args[0].as_str();
    let output = process::run(invoke(), TIMEOUT, label).await?;
    if serde_json::from_str::<serde_json::Value>(&output)?["not_running"] == true
      && operation.may_launch()
    {
      let mut open = tokio::process::Command::new("/usr/bin/open");
      // Launch hidden, without bringing Music or an existing window forward.
      open.args(["-g", "-j", "-b", "com.apple.Music"]);
      process::run(open, TIMEOUT, "launch").await?;
      // `open` can return before Music answers: give it up to 5s in all, or
      // a cold start fails, and the status reads would take it for a quit.
      let deadline = tokio::time::Instant::now() + LAUNCH_WAIT;
      loop {
        let left = deadline.saturating_duration_since(tokio::time::Instant::now());
        if left.is_zero() {
          anyhow::bail!("Music did not start");
        }
        let output = process::run(invoke(), left.min(TIMEOUT), label).await?;
        if serde_json::from_str::<serde_json::Value>(&output)?["not_running"] != true {
          return Ok(output);
        }
        tokio::time::sleep(Duration::from_millis(500).min(left)).await;
      }
    }
    Ok(output)
  }
}

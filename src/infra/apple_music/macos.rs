//! The only production Apple boundary. No Music calls at import or startup.
use super::{process, Command};
use anyhow::Result;
use std::time::Duration;

const SCRIPT: &str = include_str!("music.js");
const TIMEOUT: Duration = Duration::from_secs(8);

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
    let output = process::run(invoke(), TIMEOUT).await?;
    if serde_json::from_str::<serde_json::Value>(&output)?["not_running"] == true
      && operation.may_launch()
    {
      let mut open = tokio::process::Command::new("/usr/bin/open");
      // Launch hidden, without bringing Music or an existing window forward.
      open.args(["-g", "-j", "-b", "com.apple.Music"]);
      process::run(open, TIMEOUT).await?;
      return process::run(invoke(), TIMEOUT).await;
    }
    Ok(output)
  }
}

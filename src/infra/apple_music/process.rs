//! A bounded child runner. No shell, process-name lookup, or process-group kill.
use anyhow::{bail, Context, Result};
use std::{process::Stdio, time::Duration};
use tokio::{io::AsyncReadExt, process::Command};

const OUTPUT_LIMIT: u64 = 2 * 1024 * 1024;

pub(super) async fn run(mut command: Command, deadline: Duration) -> Result<String> {
  command
    .stdin(Stdio::null())
    .stdout(Stdio::piped())
    .stderr(Stdio::piped())
    .kill_on_drop(true);
  let mut child = command.spawn().context("Cannot start Music helper")?;
  let mut stdout = child
    .stdout
    .take()
    .context("Missing helper stdout")?
    .take(OUTPUT_LIMIT + 1);
  let mut stderr = child
    .stderr
    .take()
    .context("Missing helper stderr")?
    .take(16_385);
  let mut out = Vec::new();
  let mut err = Vec::new();
  let result = tokio::time::timeout(deadline, async {
    tokio::try_join!(
      child.wait(),
      stdout.read_to_end(&mut out),
      stderr.read_to_end(&mut err)
    )
  })
  .await;
  let status = match result {
    Ok(Ok((status, _, _))) => status,
    other => {
      // This handle is the exact process we spawned; wait reaps that child.
      let _ = child.kill().await;
      let _ = child.wait().await;
      return match other {
        Err(_) => Err(anyhow::anyhow!(
          "Music helper timed out; check macOS Automation permission and retry"
        )),
        Ok(Err(error)) => Err(error.into()),
        Ok(Ok(_)) => unreachable!(),
      };
    }
  };
  if !status.success() {
    // Do not log library strings, script arguments, or arbitrary stderr.
    if String::from_utf8_lossy(&err).contains("-1743") {
      bail!("Music automation was denied. Allow your terminal/spotatui to control Music in System Settings > Privacy & Security > Automation");
    }
    bail!("Music helper failed ({status}); check that the item is available in Music and Automation permission is enabled");
  }
  if out.len() as u64 > OUTPUT_LIMIT {
    bail!("Music response exceeded the size limit");
  }
  String::from_utf8(out).context("Music returned invalid UTF-8")
}

#[cfg(all(test, unix))]
mod tests {
  use super::*;
  #[tokio::test]
  async fn apple_music_helper_timeout_kills_and_reaps_only_its_child() {
    let mut unrelated = Command::new("/bin/sleep")
      .arg("20")
      .kill_on_drop(true)
      .spawn()
      .unwrap();
    let mut command = Command::new("/bin/sleep");
    command.arg("20");
    let start = std::time::Instant::now();
    let error = run(command, Duration::from_millis(30)).await.unwrap_err();
    let unrelated_alive = unrelated.try_wait().unwrap().is_none();
    unrelated.kill().await.unwrap();
    unrelated.wait().await.unwrap();
    assert!(unrelated_alive);
    assert!(error.to_string().contains("timed out"));
    assert!(start.elapsed() < Duration::from_secs(3));
    let mut echo = Command::new("/bin/echo");
    echo.arg("still alive");
    assert_eq!(
      run(echo, Duration::from_secs(1)).await.unwrap(),
      "still alive\n"
    );
  }
}

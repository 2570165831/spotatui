//! A bounded, serial Music worker, separate from the serial IoEvent pump.
//! A foreign start is returned to the pump only after Music acknowledged pause.
use super::{parse_snapshot, parse_uri, Command, MusicUri};
use crate::{core::app::App, infra::network::IoEvent};
use anyhow::{bail, ensure, Result};
use std::{
  future::Future,
  sync::{Arc, Weak},
  time::Duration,
};
use tokio::sync::{mpsc, Mutex};

pub(crate) trait Client: Send + 'static {
  fn execute(&self, command: Command) -> impl Future<Output = Result<String>> + Send;
}

enum Work {
  Transport { generation: u64, command: Command },
  Handoff { generation: u64, event: IoEvent },
}

pub(crate) struct Router {
  app: Arc<Mutex<App>>,
  tx: mpsc::Sender<Work>,
}

/// Validate every URI before claiming playback. Mixed-source lists never reach
/// Music or fall through to Spotify; no cross-source queue in this version.
fn start_command(
  context: &Option<String>,
  uris: &Option<Vec<String>>,
  offset: Option<usize>,
) -> Result<Option<Command>> {
  let is_apple = |u: &str| u.starts_with("applemusic:");
  if !context.as_deref().is_some_and(is_apple)
    && !uris
      .as_ref()
      .is_some_and(|list| list.iter().any(|u| is_apple(u)))
  {
    return Ok(None);
  }
  if let Some(context) = context {
    let container = parse_uri(context)?;
    let track = uris
      .as_ref()
      .map(|list| -> Result<String> {
        ensure!(
          list.len() == 1,
          "Apple Music accepts one selected track in a playlist"
        );
        match parse_uri(&list[0])? {
          MusicUri::Track(id) => Ok(id),
          _ => bail!("Expected an Apple Music track"),
        }
      })
      .transpose()?;
    return Ok(Some(Command::Play {
      container,
      track,
      offset: offset.unwrap_or(0),
    }));
  }
  let list = uris.as_ref().expect("Apple URI list was found");
  for uri in list {
    ensure!(
      matches!(parse_uri(uri)?, MusicUri::Track(_)),
      "Apple Music URI lists must contain library tracks only"
    );
  }
  let uri = list
    .get(offset.unwrap_or(0))
    .ok_or_else(|| anyhow::anyhow!("Apple Music track offset is out of range"))?;
  Ok(Some(Command::Play {
    container: parse_uri(uri)?,
    track: None,
    offset: 0,
  }))
}

impl Router {
  #[cfg(all(feature = "apple-music", target_os = "macos"))]
  pub(crate) fn new(app: &Arc<Mutex<App>>) -> Self {
    Self::with_client(app, super::macos::MacClient)
  }

  pub(crate) fn with_client<C: Client>(app: &Arc<Mutex<App>>, client: C) -> Self {
    let (tx, rx) = mpsc::channel(32);
    tokio::spawn(worker(Arc::downgrade(app), rx, client));
    Self {
      app: Arc::clone(app),
      tx,
    }
  }

  /// Returning None consumes the event immediately; no Apple call is awaited.
  pub(crate) async fn route_apple_music_event(&self, event: IoEvent) -> Option<IoEvent> {
    if let IoEvent::AppleMusicHandoff { generation, event } = event {
      return self
        .app
        .lock()
        .await
        .finish_apple_music_handoff(generation)
        .then_some(*event);
    }

    // Reserve before changing ownership. A busy helper must not lose a start
    // or invalidate the live claim without accepting the corresponding work.
    let mut app = self.app.lock().await;
    let work = match event {
      IoEvent::StartPlayback(ref context, ref uris, offset)
        if context.is_some() || uris.is_some() =>
      {
        let command = match start_command(context, uris, offset) {
          Ok(command) => command,
          Err(error) => {
            app.handle_error(error);
            return None;
          }
        };
        if command.is_none() && !app.apple_music_owns_playback() {
          return Some(event);
        }
        let permit = match self.tx.try_reserve() {
          Ok(permit) => permit,
          Err(_) => {
            app.set_error_status_message("Music is busy; retry the playback request", 4);
            return None;
          }
        };
        match command {
          Some(command) => {
            if let Err(error) = command.arguments() {
              app.handle_error(error);
              return None;
            }
            let generation = app.claim_apple_music();
            permit.send(Work::Transport {
              generation,
              command,
            });
          }
          None => {
            let generation = app.begin_apple_music_handoff();
            permit.send(Work::Handoff { generation, event });
          }
        }
        app.is_loading = false;
        return None;
      }
      other if app.apple_music_owns_playback() => {
        if app.apple_music_state().switching
          && crate::infra::network::Network::event_is_transport(&other)
        {
          app.set_status_message("Waiting for Music to pause before switching source", 4);
          return None;
        }
        let command = match other {
          IoEvent::StartPlayback(None, None, None) => Command::Resume,
          IoEvent::PausePlayback => Command::Pause,
          IoEvent::NextTrack => Command::Next,
          IoEvent::PreviousTrack | IoEvent::ForcePreviousTrack => Command::Previous,
          IoEvent::Seek(position) => Command::Seek(position),
          IoEvent::ChangeVolume(volume) => Command::Volume(volume),
          IoEvent::Shuffle(_) | IoEvent::Repeat(_) => {
            app.set_status_message("Use Music to change shuffle or repeat", 4);
            return None;
          }
          IoEvent::AdvanceNativeQueue | IoEvent::FinishNativeQueue | IoEvent::AddItemToQueue(_) => {
            app.set_status_message(crate::core::queue::APPLE_MUSIC_QUEUE_UNSUPPORTED, 4);
            return None;
          }
          IoEvent::TransferPlaybackToDevice(..) => {
            app.set_status_message("Start a Spotify track to switch playback from Music", 4);
            return None;
          }
          _ => return Some(other),
        };
        Work::Transport {
          generation: app.apple_music_state().generation,
          command,
        }
      }
      other => return Some(other),
    };
    if self.tx.try_send(work).is_err() {
      app.set_error_status_message("Music is busy; retry the request", 4);
    }
    app.is_loading = false;
    None
  }
}

async fn worker<C: Client>(weak: Weak<Mutex<App>>, mut rx: mpsc::Receiver<Work>, client: C) {
  let mut poll = tokio::time::interval(Duration::from_secs(1));
  poll.set_missed_tick_behavior(tokio::time::MissedTickBehavior::Skip);
  // Poll errors are reported once, not once per second. Explicit commands
  // always report their own failures and can be retried after permission changes.
  let mut poll_failed = false;
  loop {
    let work = tokio::select! {
      biased;
      next = rx.recv() => match next { Some(work) => Some(work), None => break },
      _ = poll.tick() => None,
    };
    let Some(app) = weak.upgrade() else {
      break;
    };
    match work {
      Some(Work::Transport {
        generation,
        command,
      }) => {
        if !owns_generation(&app, generation).await {
          continue;
        }
        let result = client
          .execute(command)
          .await
          .and_then(|json| parse_snapshot(&json));
        let mut app = app.lock().await;
        match result {
          Ok(snapshot) => {
            app.accept_apple_music_snapshot(generation, snapshot);
            poll_failed = false;
          }
          Err(error) => {
            app.apple_music_failed(generation, error);
            poll_failed = true;
          }
        }
      }
      Some(Work::Handoff { generation, event }) => {
        if !owns_generation(&app, generation).await {
          continue;
        }
        let result = client
          .execute(Command::Pause)
          .await
          .and_then(|json| parse_snapshot(&json))
          .and_then(|s| {
            ensure!(
              !s.playing,
              "Music did not acknowledge pause; the other source was not started"
            );
            Ok(s)
          });
        let mut app = app.lock().await;
        match result {
          Ok(snapshot) if app.apple_music_state().generation == generation => {
            app.acknowledge_apple_music_pause(generation, snapshot);
            app.dispatch_without_spinner(IoEvent::AppleMusicHandoff {
              generation,
              event: Box::new(event),
            });
          }
          Ok(_) => {}
          Err(error) => app.apple_music_failed(generation, error),
        }
      }
      None => {
        let generation = {
          let app = app.lock().await;
          if !app.apple_music_owns_playback() || app.apple_music_state().switching || poll_failed {
            continue;
          }
          app.apple_music_state().generation
        };
        let result = client
          .execute(Command::Snapshot)
          .await
          .and_then(|json| parse_snapshot(&json));
        let mut app = app.lock().await;
        match result {
          Ok(snapshot) => app.accept_apple_music_snapshot(generation, snapshot),
          Err(error) => {
            if app.apple_music_state().generation == generation {
              app.set_error_status_message(
                format!("Music status unavailable: {error}. Retry a playback command."),
                8,
              );
              poll_failed = true;
            }
          }
        }
      }
    }
  }
}

async fn owns_generation(app: &Arc<Mutex<App>>, generation: u64) -> bool {
  let app = app.lock().await;
  app.apple_music_owns_playback() && app.apple_music_state().generation == generation
}

#[cfg(test)]
mod tests {
  use super::*;
  use crate::core::user_config::UserConfig;
  use std::sync::mpsc::channel;

  struct FakeClient {
    calls: Arc<Mutex<Vec<Command>>>,
    fail_pause: bool,
  }
  impl Client for FakeClient {
    async fn execute(&self, command: Command) -> Result<String> {
      self.calls.lock().await.push(command.clone());
      tokio::time::sleep(Duration::from_millis(40)).await;
      if self.fail_pause && command == Command::Pause {
        bail!("permission denied");
      }
      Ok(r#"{"running":true,"playing":false,"track":null,"position":0,"volume":50}"#.into())
    }
  }

  fn apple_start() -> IoEvent {
    IoEvent::StartPlayback(None, Some(vec!["applemusic:0123456789ABCDEF".into()]), None)
  }

  #[test]
  fn apple_music_start_rejects_mixed_sources_before_claiming() {
    assert!(start_command(
      &None,
      &Some(vec![
        "applemusic:0123456789ABCDEF".into(),
        "spotify:track:x".into()
      ]),
      None
    )
    .is_err());
    assert!(start_command(
      &None,
      &Some(vec!["applemusic:0123456789ABCDEF".into()]),
      Some(2)
    )
    .is_err());
    assert!(start_command(
      &Some("spotify:playlist:x".into()),
      &Some(vec!["applemusic:0123456789ABCDEF".into()]),
      None
    )
    .is_err());
  }

  #[tokio::test]
  async fn apple_music_slow_helper_does_not_hold_app_or_pump_and_handoff_waits_for_pause() {
    struct GatedClient {
      started: mpsc::UnboundedSender<Command>,
      finish: Arc<tokio::sync::Semaphore>,
    }
    impl Client for GatedClient {
      async fn execute(&self, command: Command) -> Result<String> {
        self.started.send(command).unwrap();
        self.finish.acquire().await.unwrap().forget();
        Ok(r#"{"running":true,"playing":false,"track":null,"position":0,"volume":50}"#.into())
      }
    }
    let (tx, rx) = channel();
    let app = Arc::new(Mutex::new(App::new(tx, UserConfig::new(), None)));
    let (started, mut calls) = mpsc::unbounded_channel();
    let finish = Arc::new(tokio::sync::Semaphore::new(0));
    let router = Router::with_client(
      &app,
      GatedClient {
        started,
        finish: Arc::clone(&finish),
      },
    );
    assert!(tokio::time::timeout(
      Duration::from_secs(1),
      router.route_apple_music_event(apple_start())
    )
    .await
    .unwrap()
    .is_none());
    assert!(matches!(
      tokio::time::timeout(Duration::from_secs(1), calls.recv())
        .await
        .unwrap(),
      Some(Command::Play { .. })
    ));
    // The helper has actually started and cannot finish without our permit.
    drop(
      tokio::time::timeout(Duration::from_secs(1), app.lock())
        .await
        .unwrap(),
    );
    assert!(tokio::time::timeout(
      Duration::from_secs(1),
      router.route_apple_music_event(IoEvent::StartPlayback(
        None,
        Some(vec!["spotify:track:x".into()]),
        None
      ))
    )
    .await
    .unwrap()
    .is_none());
    assert!(rx.try_recv().is_err());
    finish.add_permits(1);
    assert_eq!(
      tokio::time::timeout(Duration::from_secs(1), calls.recv())
        .await
        .unwrap(),
      Some(Command::Pause)
    );
    assert!(rx.try_recv().is_err());
    assert!(app.lock().await.apple_music_owns_playback());
    finish.add_permits(1);
    tokio::time::timeout(Duration::from_secs(2), async {
      loop {
        if let Ok(event) = rx.try_recv() {
          assert!(matches!(
            router.route_apple_music_event(event).await,
            Some(IoEvent::StartPlayback(..))
          ));
          break;
        }
        tokio::task::yield_now().await;
      }
    })
    .await
    .unwrap();
    assert!(!app.lock().await.apple_music_owns_playback());
  }

  #[tokio::test]
  async fn apple_music_failed_pause_never_releases_another_start() {
    let (tx, rx) = channel();
    let app = Arc::new(Mutex::new(App::new(tx, UserConfig::new(), None)));
    app.lock().await.claim_apple_music();
    let router = Router::with_client(
      &app,
      FakeClient {
        calls: Arc::new(Mutex::new(vec![])),
        fail_pause: true,
      },
    );
    router
      .route_apple_music_event(IoEvent::StartPlayback(
        None,
        Some(vec!["file:///track.flac".into()]),
        None,
      ))
      .await;
    tokio::time::timeout(Duration::from_secs(2), async {
      while !app.lock().await.api_error().contains("permission denied") {
        tokio::task::yield_now().await;
      }
    })
    .await
    .unwrap();
    assert!(rx.try_recv().is_err());
    assert!(app.lock().await.apple_music_owns_playback());
  }
}

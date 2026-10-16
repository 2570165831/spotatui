//! An error's presentation text stays separate from its English diagnostic.
use super::Language;
#[cfg(any(test, feature = "spotify-session", feature = "dj-core"))]
use super::{Strings, EN, ZH_HANS, ZH_HANT};

#[derive(Debug, Clone)]
pub(crate) struct LocalizedMessage {
  english: String,
  simplified: String,
  traditional: String,
}

impl LocalizedMessage {
  #[cfg(any(test, feature = "spotify-session", feature = "dj-core"))]
  pub(crate) fn new(render: impl Fn(&Strings) -> String) -> Self {
    Self {
      english: render(&EN),
      simplified: render(&ZH_HANS),
      traditional: render(&ZH_HANT),
    }
  }

  /// Add literal context, such as a speaker name, without parsing any rendered text.
  #[cfg(feature = "spotify-session")]
  pub(crate) fn map(self, render: impl Fn(&str) -> String) -> Self {
    Self {
      english: render(&self.english),
      simplified: render(&self.simplified),
      traditional: render(&self.traditional),
    }
  }

  pub(crate) fn text(&self, language: Language) -> &str {
    match language {
      Language::ZhHans => &self.simplified,
      Language::ZhHant => &self.traditional,
      Language::Auto | Language::En => &self.english,
    }
  }
}

impl std::fmt::Display for LocalizedMessage {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    f.write_str(&self.english)
  }
}

impl std::error::Error for LocalizedMessage {}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn discovery_and_search_errors_keep_exact_diagnostics_and_separate_presentation() {
    for (message, english) in [
      (
        LocalizedMessage::new(|s| s.status.infra.made_for_you_missing.to_string()),
        "Spotify did not send the Made For You page",
      ),
      (
        LocalizedMessage::new(|s| s.status.infra.made_for_you_unavailable.to_string()),
        "Made For You is unavailable: every known browsePage hash is retired",
      ),
      (
        LocalizedMessage::new(|s| {
          s.status
            .infra
            .search_query_unavailable("searchTracks {query}")
        }),
        "Search is unavailable: every known searchTracks {query} hash is retired",
      ),
    ] {
      assert_eq!(message.to_string(), english);
      assert_eq!(message.text(Language::En), english);
      assert!(!message.text(Language::ZhHans).is_empty());
      assert!(!message.text(Language::ZhHant).is_empty());
    }
  }

  #[test]
  fn speaker_and_playlist_diagnostics_keep_the_original_english_bytes() {
    let s = &EN.status.infra;
    let cases = [
      (
        s.cast_no_answer("192.0.2.1:8009"),
        "no answer from 192.0.2.1:8009",
      ),
      (
        s.cast_no_tls("192.0.2.1:8009"),
        "no TLS handshake with 192.0.2.1:8009",
      ),
      (s.cast_message_size(65537), "a 65537-byte Cast message"),
      (s.cast_malformed.to_string(), "a malformed Cast message"),
      (s.cast_no_reply("LAUNCH"), "no reply to LAUNCH"),
      (
        s.cast_receiver_failed("null"),
        "the Spotify receiver did not start: null",
      ),
      (
        s.cast_client_id_missing.to_string(),
        "no clientID in getInfoResponse",
      ),
      (
        s.cast_login_refused("null"),
        "the speaker refused the login: null",
      ),
      (
        s.cast_access_token_missing.to_string(),
        "no accessToken in the device-auth reply",
      ),
      (
        s.speaker_token_failed("403 Forbidden", "null"),
        "the token exchange failed (403 Forbidden): null",
      ),
      (
        s.speaker_token_unsupported("\"default\""),
        "the speaker asks for token type \"default\", which is not supported yet",
      ),
      (
        s.speaker_client_id_missing.to_string(),
        "no clientID in getInfo",
      ),
      (
        s.speaker_login_refused("null", "2"),
        "the speaker refused the login: null (2)",
      ),
      (
        s.speaker_not_joined("Room"),
        "Room did not join after its login",
      ),
      (
        s.playlist_changed.to_string(),
        "The playlist changed since it loaded; open it again and retry",
      ),
    ];
    for (actual, expected) in cases {
      assert_eq!(actual, expected);
    }
  }
}

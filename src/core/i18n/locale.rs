//! Locale selection is separate from reading the process/OS environment.
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Serialize)]
#[cfg_attr(all(test, feature = "gui"), derive(ts_rs::TS))]
pub enum Language {
  #[default]
  #[serde(rename = "auto")]
  Auto,
  #[serde(rename = "en")]
  En,
  #[serde(rename = "zh-Hans")]
  ZhHans,
  #[serde(rename = "zh-Hant")]
  ZhHant,
}

impl Language {
  pub const OPTIONS: &'static [&'static str] = &["auto", "en", "zh-Hans", "zh-Hant"];

  pub const fn as_str(self) -> &'static str {
    match self {
      Self::Auto => "auto",
      Self::En => "en",
      Self::ZhHans => "zh-Hans",
      Self::ZhHant => "zh-Hant",
    }
  }

  pub fn parse(value: &str) -> Option<Self> {
    let normalized = normalized_locale(value);
    match normalized.split('-').next().unwrap_or("") {
      "auto" if normalized == "auto" => Some(Self::Auto),
      "en" => Some(Self::En),
      "zh" => Some(language_with(value)),
      _ => None,
    }
  }

  pub const fn resolve(self, system: Self) -> Self {
    match self {
      Self::Auto => match system {
        Self::ZhHans => Self::ZhHans,
        Self::ZhHant => Self::ZhHant,
        Self::Auto | Self::En => Self::En,
      },
      explicit => explicit,
    }
  }
}

impl<'de> Deserialize<'de> for Language {
  fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
    #[derive(Deserialize)]
    #[serde(untagged)]
    enum Value {
      Text(String),
      Other(serde::de::IgnoredAny),
    }
    if let Value::Text(value) = Value::deserialize(deserializer)? {
      if let Some(language) = Self::parse(&value) {
        return Ok(language);
      }
    }
    log::warn!("Invalid behavior.language; using auto (expected auto, en, zh-Hans or zh-Hant)");
    Ok(Self::Auto)
  }
}

fn normalized_locale(value: &str) -> String {
  value
    .trim()
    .split(['.', '@', ':'])
    .next()
    .unwrap_or("")
    .replace('_', "-")
    .to_ascii_lowercase()
}

/// Interpret one locale, including POSIX encoding/modifier suffixes and BCP 47
/// script/region tags. An explicit script takes precedence over the region.
/// Bare `zh` defaults to Simplified; all non-Chinese languages use English.
pub fn language_with(value: &str) -> Language {
  let normalized = normalized_locale(value);
  let mut parts = normalized.split('-');
  if parts.next() != Some("zh") {
    return Language::En;
  }
  let parts: Vec<&str> = parts.collect();
  if parts.contains(&"hant") || parts.contains(&"cht") {
    Language::ZhHant
  } else if parts.contains(&"hans") || parts.contains(&"chs") {
    Language::ZhHans
  } else if parts.iter().any(|part| matches!(*part, "tw" | "hk" | "mo")) {
    Language::ZhHant
  } else {
    Language::ZhHans
  }
}

/// POSIX message-locale overrides precede the native UI-language fallback.
/// Empty variables are ignored. C/POSIX are deliberate English selections.
fn system_language_with(values: [Option<&str>; 3], native: Option<&str>) -> Language {
  values
    .into_iter()
    .flatten()
    .find(|value| !value.trim().is_empty())
    .or(native)
    .map(language_with)
    .unwrap_or(Language::En)
}

pub fn system_language() -> Language {
  static LANGUAGE: std::sync::OnceLock<Language> = std::sync::OnceLock::new();
  *LANGUAGE.get_or_init(read_system_language)
}

fn read_system_language() -> Language {
  let values = ["LC_ALL", "LC_MESSAGES", "LANG"].map(|key| std::env::var(key).ok());
  if values
    .iter()
    .flatten()
    .any(|value| !value.trim().is_empty())
  {
    return system_language_with(values.each_ref().map(|value| value.as_deref()), None);
  }
  let native = native_language();
  system_language_with([None, None, None], native.as_deref())
}

#[cfg(target_os = "macos")]
fn native_language() -> Option<String> {
  // defaults reads the user's preferred UI languages without changing locale
  // or spawning a shell. The first language alone decides the UI language.
  let output = std::process::Command::new("/usr/bin/defaults")
    .args(["read", "-g", "AppleLanguages"])
    .output()
    .ok()?;
  if !output.status.success() {
    return None;
  }
  let text = String::from_utf8(output.stdout).ok()?;
  apple_language_with(&text).map(str::to_owned)
}

#[cfg(any(target_os = "macos", test))]
fn apple_language_with(value: &str) -> Option<&str> {
  value
    .trim()
    .strip_prefix('(')?
    .split([',', ')'])
    .next()
    .map(|language| language.trim().trim_matches('"'))
    .filter(|language| !language.is_empty())
}

#[cfg(target_os = "windows")]
fn native_language() -> Option<String> {
  use windows_sys::Win32::Globalization::{GetUserPreferredUILanguages, MUI_LANGUAGE_NAME};
  let mut count = 0;
  let mut length = 0;
  // Both calls use owned buffers. The first returns the required UTF-16 size;
  // the second writes at most that many code units, including the double NUL.
  unsafe {
    if GetUserPreferredUILanguages(
      MUI_LANGUAGE_NAME,
      &mut count,
      std::ptr::null_mut(),
      &mut length,
    ) == 0
      || length == 0
    {
      return None;
    }
    let mut buffer = vec![0u16; length as usize];
    if GetUserPreferredUILanguages(
      MUI_LANGUAGE_NAME,
      &mut count,
      buffer.as_mut_ptr(),
      &mut length,
    ) == 0
    {
      return None;
    }
    let end = buffer.iter().position(|&unit| unit == 0)?;
    (end > 0).then(|| String::from_utf16_lossy(&buffer[..end]))
  }
}

#[cfg(not(any(target_os = "macos", target_os = "windows")))]
fn native_language() -> Option<String> {
  None
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn chinese_scripts_and_regions_select_the_matching_table() {
    for locale in [
      "zh",
      "zh_CN",
      "zh_CN.UTF-8",
      "zh-SG",
      "zh_Hans",
      "zh-CHS",
      "ZH-hans-CN",
      "zh-Hans-HK",
      "zh_CN@foo",
    ] {
      assert_eq!(language_with(locale), Language::ZhHans, "{locale}");
    }
    for locale in [
      "zh_TW",
      "zh-HK",
      "zh_MO.UTF-8",
      "zh-Hant",
      "zh-CHT",
      "zh-Hant-CN",
    ] {
      assert_eq!(language_with(locale), Language::ZhHant, "{locale}");
    }
  }

  #[test]
  fn every_other_system_language_uses_english() {
    for locale in [
      "",
      "en-US",
      "ja-JP",
      "ko-KR",
      "fr_FR.UTF-8",
      "de",
      "C",
      "C.UTF-8",
      "POSIX",
      "zho",
      "not-zh-HK",
    ] {
      assert_eq!(language_with(locale), Language::En, "{locale}");
    }
  }

  #[test]
  fn locale_precedence_and_explicit_settings_do_not_mutate_the_environment() {
    assert_eq!(
      system_language_with([Some("C"), Some("zh_TW"), Some("zh_CN")], Some("zh-Hant")),
      Language::En
    );
    assert_eq!(
      system_language_with([None, Some("zh_TW"), Some("en_US")], None),
      Language::ZhHant
    );
    assert_eq!(
      system_language_with([Some(""), None, None], Some("zh-SG")),
      Language::ZhHans
    );
    assert_eq!(system_language_with([None, None, None], None), Language::En);
    assert_eq!(Language::En.resolve(Language::ZhHant), Language::En);
    assert_eq!(Language::ZhHans.resolve(Language::En), Language::ZhHans);
    assert_eq!(Language::Auto.resolve(Language::ZhHant), Language::ZhHant);
  }

  #[test]
  fn macos_uses_the_first_preferred_language_even_when_it_is_not_chinese() {
    assert_eq!(
      apple_language_with("(\n    \"zh-Hant-TW\",\n    en\n)"),
      Some("zh-Hant-TW")
    );
    assert_eq!(
      apple_language_with("(fr, \"zh-Hans\")").map(language_with),
      Some(Language::En)
    );
    assert_eq!(apple_language_with("()"), None);
    assert_eq!(apple_language_with("unavailable"), None);
  }
}

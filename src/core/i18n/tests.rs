use super::*;

#[test]
fn template_values_are_literal_and_placeholders_can_be_reordered() {
  assert_eq!(
    format_template(
      "{name}: {count}",
      &[("count", "3"), ("name", "{count} 音乐")]
    ),
    "{count} 音乐: 3"
  );
  assert_eq!(
    format_template("{a}/{a}/{missing}/{", &[("a", "中")]),
    "中/中/{missing}/{"
  );
}

fn entries(strings: &Strings) -> std::collections::BTreeMap<String, String> {
  fn visit(
    path: String,
    value: serde_json::Value,
    out: &mut std::collections::BTreeMap<String, String>,
  ) {
    match value {
      serde_json::Value::String(text) => {
        out.insert(path, text);
      }
      serde_json::Value::Object(fields) => {
        for (name, value) in fields {
          visit(
            if path.is_empty() {
              name
            } else {
              format!("{path}.{name}")
            },
            value,
            out,
          );
        }
      }
      other => panic!("the message table must contain only strings and groups: {path}: {other}"),
    }
  }
  let mut out = std::collections::BTreeMap::new();
  visit(
    String::new(),
    serde_json::to_value(strings).unwrap(),
    &mut out,
  );
  out
}

fn placeholders(text: &str) -> std::collections::BTreeSet<&str> {
  let mut names = std::collections::BTreeSet::new();
  let mut tail = text;
  while let Some(start) = tail.find('{') {
    let Some(end) = tail[start + 1..].find('}').map(|end| start + 1 + end) else {
      break;
    };
    let name = &tail[start + 1..end];
    if !name.is_empty()
      && name
        .chars()
        .all(|ch| ch.is_ascii_alphanumeric() || ch == '_')
    {
      names.insert(name);
    }
    tail = &tail[end + 1..];
  }
  names
}

#[test]
fn every_language_has_nonempty_messages_and_the_same_named_placeholders() {
  let english = entries(&EN);
  assert!(!english.is_empty());
  for (language, table) in [("en", &EN), ("zh-Hans", &ZH_HANS), ("zh-Hant", &ZH_HANT)] {
    let translated = entries(table);
    assert_eq!(
      english.keys().collect::<Vec<_>>(),
      translated.keys().collect::<Vec<_>>()
    );
    for (key, message) in &translated {
      assert!(!message.trim().is_empty(), "empty {language}: {key}");
      assert_eq!(
        placeholders(message),
        placeholders(&english[key]),
        "placeholder mismatch {language}: {key}"
      );
    }
  }
}

/// Informational by design: contributors can ship English first.
#[test]
fn reports_entries_still_in_english() {
  let english = entries(&EN);
  let simplified = entries(&ZH_HANS);
  for (language, table) in [("zh-Hans", &ZH_HANS), ("zh-Hant", &ZH_HANT)] {
    let translated = entries(table);
    let mut counts: std::collections::BTreeMap<String, (usize, usize)> =
      std::collections::BTreeMap::new();
    let mut missing = Vec::new();
    let mut simplified_only = Vec::new();
    for (key, message) in &translated {
      let group = key.rsplit_once('.').unwrap().0.to_string();
      let count = counts.entry(group).or_default();
      count.0 += 1;
      if *message == english[key] {
        missing.push(key.as_str());
        if language == "zh-Hant" && simplified[key] != english[key] {
          simplified_only.push(key.as_str());
        }
      } else {
        count.1 += 1;
      }
    }
    println!(
      "{language}: {} entries, {} differ from English",
      translated.len(),
      translated.len() - missing.len()
    );
    for (group, (total, translated)) in counts {
      println!("  {group}: {translated}/{total} translated");
    }
    println!(
      "Still English (including intentional product names/keys):\n{}",
      missing.join("\n")
    );
    if language == "zh-Hant" {
      println!(
        "Translated in Simplified but missing in Traditional:\n{}",
        simplified_only.join("\n")
      );
    }
  }
}

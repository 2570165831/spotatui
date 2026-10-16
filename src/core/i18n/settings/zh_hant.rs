use super::{SettingsStrings, EN};

// Keep ..EN so new English fields compile before their translations are added.
#[allow(clippy::needless_update)]
pub const ZH_HANT: SettingsStrings = SettingsStrings { ..EN };

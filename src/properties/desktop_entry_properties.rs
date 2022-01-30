use crate::reactive::NamedProperties;
use indradb::{Identifier, NamedProperty};
use serde_json::{json, Value};
use strum_macros::{AsRefStr, Display, IntoStaticStr};

#[allow(non_camel_case_types)]
#[derive(AsRefStr, IntoStaticStr, Display)]
pub enum DesktopEntryProperties {
    #[strum(serialize = "name")]
    NAME,
    #[strum(serialize = "appid")]
    APPID,
    #[strum(serialize = "source")]
    SOURCE,
    #[strum(serialize = "path")]
    PATH,
    #[strum(serialize = "categories")]
    CATEGORIES,
    #[strum(serialize = "groups")]
    GROUPS,
    #[strum(serialize = "exec")]
    EXEC,
    #[strum(serialize = "icon")]
    ICON,
    #[strum(serialize = "type")]
    TYPE,
    #[strum(serialize = "comment")]
    COMMENT,
    #[strum(serialize = "keywords")]
    KEYWORDS,
    #[strum(serialize = "mime_type")]
    MIME_TYPE,
    #[strum(serialize = "terminal")]
    TERMINAL,
    #[strum(serialize = "no_display")]
    NO_DISPLAY,
    #[strum(serialize = "startup_notify")]
    STARTUP_NOTIFY,
}

impl DesktopEntryProperties {
    pub fn default_value(&self) -> Value {
        match self {
            DesktopEntryProperties::NAME => json!(""),
            DesktopEntryProperties::APPID => json!(""),
            DesktopEntryProperties::SOURCE => json!(""),
            DesktopEntryProperties::PATH => json!(""),
            DesktopEntryProperties::CATEGORIES => json!([]),
            DesktopEntryProperties::GROUPS => json!([]),
            DesktopEntryProperties::EXEC => json!(""),
            DesktopEntryProperties::ICON => json!(""),
            DesktopEntryProperties::TYPE => json!(""),
            DesktopEntryProperties::COMMENT => json!(""),
            DesktopEntryProperties::KEYWORDS => json!([]),
            DesktopEntryProperties::MIME_TYPE => json!([]),
            DesktopEntryProperties::TERMINAL => json!(false),
            DesktopEntryProperties::NO_DISPLAY => json!(false),
            DesktopEntryProperties::STARTUP_NOTIFY => json!(false),
        }
    }
    pub fn properties() -> NamedProperties {
        vec![
            NamedProperty::from(DesktopEntryProperties::NAME),
            NamedProperty::from(DesktopEntryProperties::APPID),
            NamedProperty::from(DesktopEntryProperties::SOURCE),
            NamedProperty::from(DesktopEntryProperties::PATH),
            NamedProperty::from(DesktopEntryProperties::CATEGORIES),
            NamedProperty::from(DesktopEntryProperties::GROUPS),
            NamedProperty::from(DesktopEntryProperties::EXEC),
            NamedProperty::from(DesktopEntryProperties::ICON),
            NamedProperty::from(DesktopEntryProperties::TYPE),
            NamedProperty::from(DesktopEntryProperties::COMMENT),
            NamedProperty::from(DesktopEntryProperties::KEYWORDS),
            NamedProperty::from(DesktopEntryProperties::MIME_TYPE),
            NamedProperty::from(DesktopEntryProperties::TERMINAL),
            NamedProperty::from(DesktopEntryProperties::NO_DISPLAY),
            NamedProperty::from(DesktopEntryProperties::STARTUP_NOTIFY),
        ]
    }
}

impl From<DesktopEntryProperties> for NamedProperty {
    fn from(p: DesktopEntryProperties) -> Self {
        NamedProperty {
            name: Identifier::new(p.to_string()).unwrap(),
            value: p.default_value(),
        }
    }
}

impl From<DesktopEntryProperties> for String {
    fn from(p: DesktopEntryProperties) -> Self {
        p.to_string()
    }
}

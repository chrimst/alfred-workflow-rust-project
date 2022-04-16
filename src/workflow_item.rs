use std::collections::HashMap;
use serde::{Serialize, Serializer, Deserialize};
use serde::ser::SerializeSeq;

#[derive(Serialize, Deserialize)]
pub struct WorkflowItem {
    #[serde(skip_serializing_if = "Option::is_none")]
    uid: Option<String>,
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    item_type: Option<ItemType>,
    title: String,
    subtitle: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    icon: Option<Icon>,
    valid: bool,
    #[serde(rename = "match")]
    #[serde(skip_serializing_if = "Option::is_none")]
    match_key: Option<String>,
    mods: Option<HashMap<String, ModeItem>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    arg: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    action: Option<Action>,
    #[serde(skip_serializing_if = "Option::is_none")]
    text: Option<ItemText>,
    #[serde(skip_serializing_if = "Option::is_none")]
    quicklookurl: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    autocomplete: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    variables: Option<HashMap<String, String>>,
}

#[derive(Deserialize)]
pub enum ItemType {
    DEFAULT,
    FILE,
    FILE2,
}

impl ItemType {
    pub fn name(&self) -> &'static str {
        match self {
            ItemType::DEFAULT => "default",
            ItemType::FILE => "file",
            ItemType::FILE2 => "file:skipcheck",
        }
    }
}

impl Serialize for ItemType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
    {
        serializer.serialize_str(self.name())
    }
}

#[derive(Deserialize, Debug)]
pub enum IconType {
    FileIcon,
    FileType,
}

impl IconType {
    pub fn name(&self) -> &'static str {
        match self {
            IconType::FileIcon => "fileicon",
            IconType::FileType => "filetype",
        }
    }
}

impl Serialize for IconType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
    {
        serializer.serialize_str(self.name())
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Icon {
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    icon_type: Option<IconType>,
    path: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ModeItem {
    valid: bool,
    arg: String,
    subtitle: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    icon: Option<Icon>,
    #[serde(skip_serializing_if = "Option::is_none")]
    variables: Option<HashMap<String, String>>,
}

impl ModeItem {
    pub fn new(arg: &str, subtitle: &str) -> ModeItem {
        ModeItem {
            valid: true,
            arg: arg.to_string(),
            subtitle: subtitle.to_string(),
            icon: None,
            variables: None,
        }
    }
}

pub enum Modifier {
    CMD,
    CTRL,
    ALT,
    FN,
}

impl Modifier {
    fn name(self) -> &'static str {
        match self {
            Modifier::CMD => "cmd",
            Modifier::FN => "fn",
            Modifier::CTRL => "ctrl",
            Modifier::ALT => "alt",
        }
    }
}

// in later
#[derive(Deserialize, Debug)]
pub enum Action {
    S1 { var: String },
    S2 { var: Vec<String> },
    S3 { var: ActionItem },
}

impl Serialize for Action {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
    {
        match self {
            Action::S1 { var } => serializer.serialize_str(var),
            Action::S2 { var } => {
                let mut seq = serializer.serialize_seq(Some(var.len()))?;
                for e in var {
                    seq.serialize_element(e)?;
                }
                seq.end()
            }
            Action::S3 { var } => ActionItem::serialize(var, serializer),
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ActionItem {
    #[serde(skip_serializing_if = "Option::is_none")]
    text: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    file: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    auto: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct ItemText {
    #[serde(skip_serializing_if = "Option::is_none")]
    copy: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "largetype")]
    large_type: Option<String>,
}
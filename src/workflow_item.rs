use std::collections::HashMap;

use serde::ser::SerializeSeq;
use serde::{Deserialize, Serialize, Serializer};

use crate::common::EnumIdent;
use crate::icon::Icon;

// the key used for modifier
pub enum ModKey {
    CMD,
    CTRL,
    ALT,
    FN,
    SHIFT,
}

#[derive(Serialize, Deserialize)]
pub struct Modifier {
    #[serde(skip_serializing_if = "Option::is_none")]
    arg: Option<Vec<String>>,
    subtitle: String,
    valid: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    icon: Option<Icon>,
    #[serde(skip_serializing_if = "Option::is_none")]
    variables: Option<HashMap<String, String>>,
}

impl Modifier {
    pub fn new() -> Modifier {
        return Modifier {
            arg: None,
            subtitle: "".to_string(),
            valid: true,
            icon: None,
            variables: None,
        };
    }

    pub fn valid(mut self, v: bool) -> Modifier {
        self.valid = v;
        self
    }

    pub fn subtitle(mut self, title: &str) -> Modifier {
        self.subtitle = title.to_string();
        self
    }

    pub fn icon(mut self, icon: Icon) -> Modifier {
        self.icon = Some(icon);
        self
    }

    pub fn args(mut self, arg: &str) -> Modifier {
        let mut args = self.arg.unwrap_or_default();
        args.push(arg.to_string());
        self.arg = Some(args);
        self
    }

    pub fn vars(mut self, name: &str, value: &str) -> Modifier {
        let mut map = self.variables.unwrap_or_default();
        map.insert(name.to_string(), value.to_string());
        self.variables = Some(map);
        self
    }
}

pub enum ItemType {
    DEFAULT,
    FILE,
    FILE2,
}

impl EnumIdent for ModKey {
    fn name(&self) -> &'static str {
        match self {
            ModKey::CMD => "cmd",
            ModKey::FN => "fn",
            ModKey::CTRL => "ctrl",
            ModKey::ALT => "alt",
            ModKey::SHIFT => "shift",
        }
    }
}

impl EnumIdent for ItemType {
    fn name(&self) -> &'static str {
        match self {
            ItemType::DEFAULT => "default",
            ItemType::FILE => "file",
            ItemType::FILE2 => "file:skipcheck",
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct WorkflowItem {
    #[serde(skip_serializing_if = "Option::is_none")]
    uid: Option<String>,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    item_type: Option<String>,
    title: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    subtitle: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    icon: Option<Icon>,
    valid: bool,
    #[serde(rename = "match", skip_serializing_if = "Option::is_none")]
    matches: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    mods: Option<HashMap<String, Modifier>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    arg: Option<String>,
    #[serde(rename = "action", skip_serializing_if = "Option::is_none")]
    actions: Option<Action>,
    #[serde(skip_serializing_if = "Option::is_none")]
    text: Option<ItemText>,
    #[serde(rename = "quicklookurl", skip_serializing_if = "Option::is_none")]
    quick_look: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    autocomplete: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    variables: Option<HashMap<String, String>>,
}

impl WorkflowItem {
    pub fn new(title: &str) -> WorkflowItem {
        WorkflowItem {
            title: title.to_string(),
            uid: None,
            item_type: None,
            subtitle: None,
            icon: None,
            valid: true,
            matches: None,
            mods: None,
            arg: None,
            actions: None,
            text: None,
            quick_look: None,
            autocomplete: None,
            variables: None,
        }
    }

    pub fn uid(mut self, uid: &str) -> WorkflowItem {
        self.uid = Some(uid.to_string());
        self
    }

    pub fn item_type(mut self, item_type: ItemType) -> WorkflowItem {
        self.item_type = Some(item_type.name().to_string());
        self
    }

    pub fn subtitle(mut self, subtitle: &str) -> WorkflowItem {
        self.subtitle = Some(subtitle.to_string());
        self
    }

    pub fn icon(mut self, icon: Icon) -> WorkflowItem {
        self.icon = Some(icon);
        self
    }

    pub fn valid(mut self, v: bool) -> WorkflowItem {
        self.valid = v;
        self
    }

    pub fn matches(mut self, matches: &str) -> WorkflowItem {
        self.matches = Some(matches.to_string());
        self
    }

    pub fn mods(mut self, keys: Vec<ModKey>, modifier: Modifier) -> WorkflowItem {
        // do nothing if no keys
        if keys.len() <= 0 {
            return self;
        }

        let x1: Vec<&str> = keys.iter().map(|x| x.name()).collect();
        let key_str = x1.join("+");

        let mut map = self.mods.unwrap_or_default();
        map.insert(key_str, modifier);
        self.mods = Some(map);
        self
    }

    pub fn actions(mut self, action: Action) -> WorkflowItem {
        self.actions = Some(action);
        self
    }
    pub fn text(mut self, text: ItemText) -> WorkflowItem {
        self.text = Some(text);
        self
    }
    pub fn quick_look(mut self, look_uri: &str) -> WorkflowItem {
        self.quick_look = Some(look_uri.to_string());
        self
    }
    pub fn auto_complete(mut self, auto_complete: &str) -> WorkflowItem {
        self.autocomplete = Some(auto_complete.to_string());
        self
    }
    pub fn vars(mut self, name: &str, value: &str) -> WorkflowItem {
        let mut vars = self.variables.unwrap_or_default();
        vars.insert(name.to_string(), value.to_string());
        self.variables = Some(vars);
        self
    }
}

// in later
#[derive(Deserialize)]
pub enum Action {
    SingleItem(String),
    MultiItem(Vec<String>),
    ObjectItem(ActionItem),
}

impl Serialize for Action {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Action::SingleItem(var) => serializer.serialize_str(var),
            Action::MultiItem(var) => {
                let mut seq = serializer.serialize_seq(Some(var.len()))?;
                for e in var {
                    seq.serialize_element(e)?;
                }
                seq.end()
            }
            Action::ObjectItem(var) => ActionItem::serialize(var, serializer),
        }
    }
}

#[derive(Serialize, Deserialize)]
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

impl ActionItem {
    pub fn new() -> ActionItem {
        ActionItem {
            text: None,
            url: None,
            file: None,
            auto: None,
        }
    }

    pub fn text(mut self, text: &str) -> ActionItem {
        let mut texts = self.text.unwrap_or_default();
        texts.push(text.to_string());
        self.text = Some(texts);
        self
    }

    pub fn texts(mut self, texts: Vec<&str>) -> ActionItem {
        let mut a_texts = self.text.unwrap_or_default();
        texts.iter().for_each(|it| a_texts.push(it.to_string()));
        self.text = Some(a_texts);
        self
    }

    pub fn url(mut self, url: &str) -> ActionItem {
        self.url = Some(url.to_string());
        self
    }

    pub fn file(mut self, file: &str) -> ActionItem {
        self.file = Some(file.to_string());
        self
    }

    pub fn auto(mut self, auto: &str) -> ActionItem {
        self.auto = Some(auto.to_string());
        self
    }
}

#[derive(Serialize, Deserialize)]
pub struct ItemText {
    #[serde(skip_serializing_if = "Option::is_none")]
    copy: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "largetype")]
    large_type: Option<String>,
}

impl ItemText {
    pub fn new(copy_text: &str) -> ItemText {
        ItemText {
            copy: Some(copy_text.to_string()),
            large_type: None,
        }
    }

    pub fn large_text(mut self, text: &str) -> ItemText {
        self.large_type = Some(text.to_string());
        self
    }
}

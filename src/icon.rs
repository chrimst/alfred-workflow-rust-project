use crate::common::EnumIdent;
use serde::{Deserialize, Serialize};

pub enum IconType {
    IconFile,
    FileType,
    ImageSelf,
}

impl EnumIdent for IconType {
    fn name(&self) -> &'static str {
        match self {
            IconType::IconFile => "fileicon",
            IconType::FileType => "filetype",
            IconType::ImageSelf => "",
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct Icon {
    #[serde(rename = "type")]
    icon_type: String,
    path: String,
}

impl Icon {
    pub fn new(path: &str, icon_type: Option<IconType>) -> Icon {
        Icon {
            icon_type: if icon_type.is_none() {
                IconType::ImageSelf.name().to_string()
            } else {
                icon_type.unwrap().name().to_string()
            },
            path: path.to_string(),
        }
    }
}

pub enum BuiltinIcon {
    ACCOUNT,
    BURN,
    CLOCK,
    COLOR,
    COLOUR,
    EJECT,
    ERROR,
    FAVORITE,
    FAVOURITE,
    GROUP,
    HELP,
    HOME,
    INFO,
    NETWORK,
    NOTE,
    SETTINGS,
    SWIRL,
    SWITCH,
    SYNC,
    TRASH,
    USER,
    WARNING,
    WEB,
}

fn build_icon_path(icon_full_name: &str) -> String {
    let mut sys_path =
        "/System/Library/CoreServices/CoreTypes.bundle/Contents/Resources/".to_string();
    sys_path.push_str(icon_full_name);
    sys_path
}

impl BuiltinIcon {
    pub fn get_icon(self) -> Icon {
        match self {
            BuiltinIcon::ACCOUNT => Icon::new(
                build_icon_path(&"Accounts.icns").as_str(),
                None,
            ),
            BuiltinIcon::BURN => Icon::new(
                build_icon_path(&"BurningIcon.icns").as_str(),
                None,
            ),
            BuiltinIcon::CLOCK => Icon::new(
                build_icon_path(&"Clock.icns").as_str(),
                None,
            ),
            BuiltinIcon::COLOR => Icon::new(
                build_icon_path(&"ProfileBackgroundColor.icns").as_str(),
                None,
            ),
            BuiltinIcon::COLOUR => Icon::new(
                build_icon_path(&"ProfileBackgroundColor.icns").as_str(),
                None,
            ),
            BuiltinIcon::EJECT => Icon::new(
                build_icon_path(&"EjectMediaIcon.icns").as_str(),
                None,
            ),
            BuiltinIcon::ERROR => Icon::new(
                build_icon_path(&"AlertStopIcon.icns").as_str(),
                None,
            ),
            BuiltinIcon::FAVORITE => Icon::new(
                build_icon_path(&"ToolbarFavoritesIcon.icns").as_str(),
                None,
            ),
            BuiltinIcon::FAVOURITE => Icon::new(
                build_icon_path(&"ToolbarFavoritesIcon.icns").as_str(),
                None,
            ),
            BuiltinIcon::GROUP => Icon::new(
                build_icon_path(&"GroupIcon.icns").as_str(),
                None,
            ),
            BuiltinIcon::HELP => Icon::new(
                build_icon_path(&"HelpIcon.icns").as_str(),
                None,
            ),
            BuiltinIcon::HOME => Icon::new(
                build_icon_path(&"HomeFolderIcon.icns").as_str(),
                None,
            ),
            BuiltinIcon::INFO => Icon::new(
                build_icon_path(&"ToolbarInfo.icns").as_str(),
                None,
            ),
            BuiltinIcon::NETWORK => Icon::new(
                build_icon_path(&"GenericNetworkIcon.icns").as_str(),
                None,
            ),
            BuiltinIcon::NOTE => Icon::new(
                build_icon_path(&"AlertNoteIcon.icns").as_str(),
                None,
            ),
            BuiltinIcon::SETTINGS => Icon::new(
                build_icon_path(&"ToolbarAdvanced.icns").as_str(),
                None,
            ),
            BuiltinIcon::SWIRL => Icon::new(
                build_icon_path(&"ErasingIcon.icns").as_str(),
                None,
            ),
            BuiltinIcon::SWITCH => Icon::new(
                build_icon_path(&"General.icns").as_str(),
                None,
            ),
            BuiltinIcon::SYNC => Icon::new(
                build_icon_path(&"Sync.icns").as_str(),
                None,
            ),
            BuiltinIcon::TRASH => Icon::new(
                build_icon_path(&"TrashIcon.icns").as_str(),
                None,
            ),
            BuiltinIcon::USER => Icon::new(
                build_icon_path(&"UserIcon.icns").as_str(),
                None,
            ),
            BuiltinIcon::WARNING => Icon::new(
                build_icon_path(&"AlertCautionIcon.icns").as_str(),
                None,
            ),
            BuiltinIcon::WEB => Icon::new(
                build_icon_path(&"BookmarkIcon.icns").as_str(),
                None,
            ),
        }
    }
}

// System icons

mod alfred;
mod alfred_logger;
mod common;
mod icon;
mod version;
mod workflow;
mod workflow_cache;
mod workflow_config;
mod workflow_item;
mod workflow_keychain;
mod workflow_updater;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}

#[cfg(test)]
mod workflow_tests {
    use crate::workflow::AlfredWorkflow;

    #[test]
    fn test_workflow_init_ok() {
        let workflow = AlfredWorkflow::init();
        assert!(workflow.is_ok())
    }

    #[test]
    fn test_workflow_send_feedback_ok() {
        let workflow = AlfredWorkflow::init().unwrap();
        workflow.send_feedback();
        assert!(true)
    }
}

#[cfg(test)]
mod workflow_item_tests {
    use crate::icon::{BuiltinIcon, Icon};
    use crate::workflow_item::{
        Action, ActionItem, ItemText, ItemType, ModKey, Modifier, WorkflowItem,
    };

    #[test]
    fn test_modifier_init_ok() {
        let modifier = Modifier::new()
            .subtitle(&"hello")
            .valid(false)
            .icon(Icon::new(&"test_path", None))
            .args("kk")
            .args("vv")
            .vars("cmd", "just_ket")
            .vars("cmd", "just_key_2")
            .vars("cmd2", "hello_kitty");
        let modifier_str = serde_json::to_string(&modifier).unwrap();

        assert!(modifier_str.contains("hello"));
        assert!(modifier_str.contains("kk"));
        assert!(modifier_str.contains("vv"));
        assert!(modifier_str.contains("cmd"));
        assert!(modifier_str.contains("just_key_2"));
        assert!(!modifier_str.contains("just_ket"));
        assert!(modifier_str.contains("cmd2"));
        assert!(modifier_str.contains("hello_kitty"));
    }

    #[test]
    fn test_workflow_item_build() {
        let item = WorkflowItem::new("this_is_title")
            .actions(Action::SingleItem("single_action".to_string()))
            .uid("uuid")
            .item_type(ItemType::DEFAULT)
            .subtitle("this_is_subtitle")
            .icon(BuiltinIcon::ACCOUNT.get_icon())
            .valid(true)
            .matches("this_is_match")
            .mods(vec![ModKey::CMD, ModKey::CTRL], Modifier::new())
            .mods(vec![], Modifier::new().subtitle("invalid_subtitle"))
            .quick_look("quick_look_uri")
            .auto_complete("this_is_auto_complete_text")
            .vars("key_1", "value_1")
            .text(ItemText::new("copy_text"));

        let item_str = serde_json::to_string(&item).unwrap();
        assert!(item_str.contains("single_action"));
        assert!(item_str.contains("uuid"));
        assert!(item_str.contains("this_is_subtitle"));
        assert!(item_str.contains("this_is_match"));
        assert!(item_str.contains("quick_look_uri"));
        assert!(item_str.contains("cmd+ctrl"));
        assert!(item_str.contains("this_is_auto_complete_text"));
        assert!(item_str.contains("copy_text"));
        assert!(!item_str.contains("invalid_subtitle"));
    }

    #[test]
    fn test_workflow_item_build_with_multi_item_action() {
        let item = WorkflowItem::new("test")
            .actions(Action::MultiItem(vec!["cc".to_string(), "bb".to_string()]));

        let item_str = serde_json::to_string(&item).unwrap();
        assert!(item_str.contains("[\"cc\",\"bb\"]"));
    }

    #[test]
    fn test_workflow_item_build_with_object_item_action() {
        let item = WorkflowItem::new("test").actions(Action::ObjectItem(
            ActionItem::new()
                .text("cc")
                .texts(vec!["bb", "dd"])
                .url("action_url")
                .file("this_is_file")
                .auto("use_auto"),
        ));

        let item_str = serde_json::to_string(&item).unwrap();
        assert!(item_str.contains("[\"cc\",\"bb\",\"dd\"]"));
        assert!(item_str.contains("text"));
        assert!(item_str.contains("\"url\":"));
        assert!(item_str.contains("\"file\":"));
        assert!(item_str.contains("action_url"));
        assert!(item_str.contains("this_is_file"));
        assert!(item_str.contains("use_auto"));
    }
}

#[cfg(test)]
mod icon_tests {
    use crate::common::EnumIdent;
    use crate::icon::{BuiltinIcon, Icon, IconType};

    #[test]
    fn test_get_builtin_icon() {
        let icon = BuiltinIcon::BURN.get_icon();
        let icon_str = serde_json::to_string(&icon).unwrap();
        assert!(icon_str.contains("BurningIcon.icns"));
        assert!(icon_str.contains(IconType::IconFile.name()))
    }

    #[test]
    fn test_new_customize_icon() {
        let icon_1 = Icon::new(&"my_path", None);
        let icon_1_str = serde_json::to_string(&icon_1).unwrap();
        assert!(icon_1_str.contains("my_path"));
        assert!(icon_1_str.contains(IconType::ImageSelf.name()))
    }
}

#[cfg(test)]
mod env_tests {
    use crate::alfred::{Alfred, AlfredEnv};

    #[test]
    fn test_alfred_env_get_fine() {
        dotenv::dotenv().ok();
        let alfred = Alfred::init();
        assert_eq!(alfred.get_preference_path(), "alfred_preferences");
        assert_eq!(
            alfred.get_preference_hash_path(),
            "alfred_preferences_localhash"
        );
        assert_eq!(alfred.get_theme(), "alfred.theme.yosemite");
        assert_eq!(alfred.get_theme_background(), "rgba(255,255,255,0.98)");
        assert_eq!(alfred.get_theme_subtext(), "3");
        assert_eq!(alfred.get_version_build(), "277");
        assert_eq!(alfred.get_version(), "2.4");
        assert_eq!(
            alfred.get_workflow_bundle_id(),
            "com.alfredapp.david.googlesuggest"
        );
        assert_eq!(alfred.get_workflow_cache_path(), "cache_path");
        assert_eq!(alfred.get_workflow_data_path(), "cache_path");
        assert_eq!(alfred.get_workflow_name(), "GoogleSuggest");
        assert_eq!(
            alfred.get_workflow_uuid(),
            "user.workflow.B0AC54EC-601C-479A-9428-01F9FD732959"
        );
        assert!(alfred.is_debug_mode());
    }
}

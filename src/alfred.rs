use serde::ser::SerializeSeq;
use serde::{Deserialize, Serialize, Serializer};


#[derive(Serialize, Deserialize)]
pub struct Alfred {}

impl Alfred {
    pub fn init() -> Alfred {
        Alfred {}
    }


    pub fn search() {}
    pub fn action() {}
    pub fn browse() {}
    pub fn set_theme() {}
    pub fn reload() {}
    pub fn trigger() {}
    pub fn set_config() {}
    pub fn remove_config() {}
    pub fn action_with_types() {}
}

pub trait AlfredEnv {
    fn get_preference_path(&self) -> String;
    fn get_preference_hash_path(&self) -> String;
    fn get_theme(&self) -> String;
    fn get_version(&self) -> String;
    fn get_version_build(&self) -> String;
    fn get_workflow_bundle_id(&self) -> String;
    fn get_workflow_cache_path(&self) -> String;
    fn get_workflow_data_path(&self) -> String;
    fn get_workflow_name(&self) -> String;
    fn is_debug_mode(&self) -> bool;
    fn get_workflow_uuid(&self) -> String;
    fn get_workflow_version(&self) -> String;
    fn get_theme_background(&self) -> String;
    fn get_theme_selection_background(&self) -> String;
    fn get_theme_subtext(&self) -> String;
}

impl AlfredEnv for Alfred {
    fn get_theme_subtext(&self) -> String {
        return std::env::var("alfred_theme_subtext").unwrap_or_default();
    }
    fn get_preference_path(&self) -> String {
        return std::env::var("alfred_preferences").unwrap_or_default();
    }


    fn get_preference_hash_path(&self) -> String {
        return std::env::var("alfred_preferences_localhash").unwrap_or_default();
    }

    fn get_theme(&self) -> String {
        return std::env::var("alfred_theme").unwrap_or_default();
    }
    fn get_version(&self) -> String {
        return std::env::var("alfred_version").unwrap_or_default();
    }
    fn get_version_build(&self) -> String {
        return std::env::var("alfred_version_build").unwrap_or_default();
    }

    fn get_workflow_bundle_id(&self) -> String {
        return std::env::var("alfred_workflow_bundleid").unwrap_or_default();
    }
    fn get_workflow_cache_path(&self) -> String {
        return std::env::var("alfred_workflow_cache").unwrap_or_default();
    }
    fn get_workflow_data_path(&self) -> String {
        return std::env::var("alfred_workflow_data").unwrap_or_default();
    }
    fn get_workflow_name(&self) -> String {
        return std::env::var("alfred_workflow_name").unwrap_or_default();
    }
    fn is_debug_mode(&self) -> bool {
        return std::env::var("alfred_debug")
            .unwrap_or_default()
            .eq("1");
    }
    fn get_workflow_uuid(&self) -> String {
        return std::env::var("alfred_workflow_uid").unwrap_or_default();
    }

    fn get_workflow_version(&self) -> String {
        return std::env::var("alfred_workflow_version").unwrap_or_default();
    }

    fn get_theme_background(&self) -> String {
        return std::env::var("alfred_theme_background").unwrap_or_default();
    }
    fn get_theme_selection_background(&self) -> String {
        return std::env::var("alfred_theme_selection_background").unwrap_or_default();
    }
}
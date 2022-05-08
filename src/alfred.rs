use serde::{Deserialize, Serialize};
use crate::alfred_logger::Logger;

#[derive(Serialize, Deserialize)]
pub struct Alfred {}

impl Alfred {
    pub fn init() -> Alfred {
        Logger::init();
        Alfred {}
    }

    pub fn search(&self, query: &str) {
        let script = format!("Application(\"{}\").search(\"{}\");", self.get_app_name(), query);
        let mut _command = std::process::Command::new("/usr/bin/osascript")
            .args(["-l", "JavaScript", "-e", script.as_str()])
            .spawn();
    }
    pub fn action(&self, query: &str) {
        let script = format!("Application(\"{}\").action(\"{}\");", self.get_app_name(), query);
        let mut _command = std::process::Command::new("/usr/bin/osascript")
            .args(["-l", "JavaScript", "-e", script.as_str()])
            .spawn();
    }
    pub fn action_with_types(&self, query: &str, action_type: &str) {
        let script = format!("Application(\"{}\").action(\"{}\",\"{}\");", self.get_app_name(), query, action_type);
        let mut _command = std::process::Command::new("/usr/bin/osascript")
            .args(["-l", "JavaScript", "-e", script.as_str()])
            .spawn();
    }
    pub fn browse(&self, query: &str) {
        let script = format!("Application(\"{}\").browse(\"{}\");", self.get_app_name(), query);
        let mut _command = std::process::Command::new("/usr/bin/osascript")
            .args(["-l", "JavaScript", "-e", script.as_str()])
            .spawn();
    }
    pub fn set_theme(&self, theme: &str) {
        let script = format!("Application(\"{}\").setTheme(\"{}\");", self.get_app_name(), theme);
        let mut _command = std::process::Command::new("/usr/bin/osascript")
            .args(["-l", "JavaScript", "-e", script.as_str()])
            .spawn();
    }
    pub fn reload(&self, workflow: &str) {
        let script = format!("Application(\"{}\").reloadWorkflow(\"{}\");", self.get_app_name(), workflow);
        let mut _command = std::process::Command::new("/usr/bin/osascript")
            .args(["-l", "JavaScript", "-e", script.as_str()])
            .spawn();
    }
    pub fn trigger(&self, p1: &str, p2: &str) {
        let script = format!("Application(\"{}\").runTrigger(\"{}\",\"{}\");", self.get_app_name(), p1, p2);
        let mut _command = std::process::Command::new("/usr/bin/osascript")
            .args(["-l", "JavaScript", "-e", script.as_str()])
            .spawn();
    }
    pub fn set_config(&self, bundle: &str, query: &str, query2: &str) {
        let script = format!("\
            tell application id \"{}\"
                 set configuration \"{}\" to value \"{}\" in workflow \"{}\"
            end
        ", self.get_app_name(), query, query2, bundle);
        let mut _command = std::process::Command::new("/usr/bin/osascript")
            .args(["-e", script.as_str()])
            .spawn();
    }
    pub fn remove_config(&self, bundle: &str, query: &str) {
        let script = format!("\
            tell application id \"{}\"
                 remove configuration \"{}\" in workflow \"{}\"
            end
        ", self.get_app_name(), query, bundle);
        let mut _command = std::process::Command::new("/usr/bin/osascript")
            .args(["-e", script.as_str()])
            .spawn();
    }
    pub fn get_app_name(&self) -> &str {
        "com.runningwithcrayons.Alfred"
    }
}

#[test]
fn test_alfred_search() {
    Alfred::init().browse("c");
    Alfred::init().action("c");
}

#[test]
fn test_set_config() {
    Alfred::init().set_config("com.christ.alfred.rust.demo", "test_rust_keu", "ccc")
}

#[test]
fn test_remove_config() {
    Alfred::init().remove_config("com.christ.alfred.rust.demo", "test_rust_keu")
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
        return std::env::var("alfred_debug").unwrap_or_default().eq("1");
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

use std::thread::sleep;
use serde::{Serialize, Deserialize};
use crate::alfred::{Alfred, AlfredEnv};
use crate::workflow_item::WorkflowItem;

// Alfred workflow object
#[derive(Serialize, Deserialize)]
pub struct AlfredWorkflow {
    #[serde(skip_serializing)]
    alfred: Alfred,
    items: Vec<WorkflowItem>,
}


impl AlfredWorkflow {
    pub fn init() -> Result<AlfredWorkflow, &'static str> {
        Ok(AlfredWorkflow {
            alfred: Alfred::init(),
            items: Vec::new(),
        })
    }

    pub fn get_workflow_env(key: &str) -> String {
        std::env::var(key).unwrap_or_default()
    }

    pub fn send_feedback(&self) {
        print!("{}", serde_json::to_string(self).unwrap())
    }
}

impl AlfredEnv for AlfredWorkflow {
    fn get_preference_path(self) -> String {
        self.alfred.get_preference_path()
    }

    fn get_preference_hash_path(self) -> String {
        self.alfred.get_preference_hash_path()
    }

    fn get_theme(self) -> String {
        self.alfred.get_theme()
    }

    fn get_version(self) -> String {
        self.alfred.get_version()
    }

    fn get_version_build(self) -> String {
        self.alfred.get_version_build()
    }

    fn get_workflow_bundle_id(self) -> String {
        self.alfred.get_workflow_bundle_id()
    }

    fn get_workflow_cache_path(self) -> String {
        self.alfred.get_workflow_cache_path()
    }

    fn get_workflow_data_path(self) -> String {
        self.alfred.get_workflow_data_path()
    }

    fn get_workflow_name(self) -> String {
        self.alfred.get_workflow_name()
    }

    fn is_debug_mode(self) -> bool {
        self.alfred.is_debug_mode()
    }

    fn get_workflow_uuid(self) -> String {
        self.alfred.get_workflow_uuid()
    }

    fn get_workflow_version(self) -> String {
        self.alfred.get_workflow_version()
    }

    fn get_theme_background(self) -> String {
        self.alfred.get_theme_background()
    }

    fn get_theme_selection_background(self) -> String {
        self.alfred.get_theme_selection_background()
    }

    fn get_theme_subtext(self) -> String {
        self.alfred.get_theme_subtext()
    }
}
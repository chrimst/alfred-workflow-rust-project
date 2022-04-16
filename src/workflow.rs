use serde::{Serialize, Deserialize};
use crate::workflow_item::WorkflowItem;

// Alfred workflow object
#[derive(Serialize, Deserialize)]
pub struct AlfredWorkflow {
    items: Vec<WorkflowItem>,
}


impl AlfredWorkflow {
    pub fn init() -> Result<AlfredWorkflow, &'static str> {
        Ok(AlfredWorkflow {
            items: Vec::new()
        })
    }

    pub fn send_feedback(&self) {
        print!("{}", serde_json::to_string(self).unwrap())
    }
}
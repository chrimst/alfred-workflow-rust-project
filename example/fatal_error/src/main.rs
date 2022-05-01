use alfred_workflow_rust_project::workflow::AlfredWorkflow;
use alfred_workflow_rust_project::workflow_item::WorkflowItem;

fn main() {

    AlfredWorkflow::init()
        .fatal_error("errorCode","errorMsg")
        .add_item(WorkflowItem::new("newItem")
            .subtitle("cccc"))
        .send_feedback();
}

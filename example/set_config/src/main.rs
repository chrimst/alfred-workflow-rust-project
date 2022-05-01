use alfred_workflow_rust_project::alfred::{Alfred, AlfredEnv};
use alfred_workflow_rust_project::workflow::AlfredWorkflow;

fn main() {
    let config_key = "test_set_config";

    let alfred = Alfred::init();
    let workflow = AlfredWorkflow::init();

    let bundle_id = workflow.get_workflow_bundle_id();
    alfred.set_config(bundle_id.as_str(),config_key,"cc");
    print!("ccccc.........")
}

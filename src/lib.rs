mod workflow;
mod workflow_item;
mod icon;
mod common;

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
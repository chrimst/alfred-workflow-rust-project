use crate::alfred::AlfredEnv;
use crate::workflow::AlfredWorkflow;
use security_framework::passwords;

impl AlfredWorkflow {
    pub fn get_generic_password(&self, name: &str) -> String {
        let service = self.get_workflow_bundle_id();
        String::from_utf8(
            passwords::get_generic_password(service.as_str(), name).unwrap_or_default(),
        )
        .unwrap_or_default()
    }

    pub fn set_generic_password(&self, name: &str, password: &str) -> bool {
        let service = self.get_workflow_bundle_id();
        passwords::set_generic_password(service.as_str(), name, password.as_bytes()).is_ok()
    }
}

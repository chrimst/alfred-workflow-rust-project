use std::fs::OpenOptions;
use std::io::Write;
use std::process::Command;
use std::time::Duration;
use nix::libc::printf;
use crate::alfred::AlfredEnv;
use crate::workflow::AlfredWorkflow;
use nix::unistd::{fork, ForkResult};

impl AlfredWorkflow {
    pub fn run_background(&self, f: &dyn Fn(&AlfredWorkflow) -> ()) {
        let fork_rs = fork();
        if fork_rs.is_ok() {
            let fork = fork_rs.unwrap();
            if fork.is_child() {
                f(self)
            }
        }
    }
}

#[test]
fn test_run_backgroud() {
    let workflow = AlfredWorkflow::init();
    workflow.run_background(&cc);
    print!("hello")
}

fn cc(v: &AlfredWorkflow) {
    let mut file = OpenOptions::new()
        .create(true)
        .write(true)
        .open("./test.log").unwrap();
    file.write("cccachjashdckjas".as_bytes());
    print!("ccc => {}", v.get_workflow_cache_path())
}
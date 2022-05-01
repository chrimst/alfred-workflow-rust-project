use std::error::Error;
use std::fs::OpenOptions;
use std::io::{Read, Write};
use std::os::unix::raw::pthread_t;
use std::path::Path;
use std::time::{Duration, SystemTime};
use crate::alfred::AlfredEnv;
use crate::workflow::AlfredWorkflow;

impl AlfredWorkflow {
    pub fn cache(&mut self, name: &str, data: &str) {
        let pre_path = self.get_workflow_cache_path();
        let full_name = format!("{}/{}", pre_path, name);

        // try open cache file
        let file_rs = OpenOptions::new()
            .create(true)
            .write(true)
            .open(full_name);

        // if open failed
        // report fatal error
        if file_rs.is_err() {
            self.fatal_error("CacheError", file_rs.err().unwrap().description());
        } else {
            // write data to cache file
            let mut file = file_rs.unwrap();
            let write_rs = file.write(data.as_bytes());
            if write_rs.is_err() {
                self.fatal_error("CacheError", write_rs.err().unwrap().description())
            }
        }
    }

    pub fn load(&mut self, name: &str) -> String {
        let pre_path = self.get_workflow_cache_path();
        let full_name = format!("{}/{}", pre_path, name);

        let file_rs = OpenOptions::new()
            .read(true)
            .open(full_name);

        if file_rs.is_err() {
            self.fatal_error("CacheError", file_rs.err().unwrap().description());
            "".to_string()
        } else {
            // read data from cache file
            let mut file = file_rs.unwrap();
            let mut buf = String::new();
            let read_rs = file.read_to_string(&mut buf);
            if read_rs.is_err() {
                self.fatal_error("CacheError", read_rs.err().unwrap().description());
            }
            buf
        }
    }

    pub fn expired(&mut self, name: &str, max_age: u64) -> bool {
        let pre_path = self.get_workflow_cache_path();
        let full_name = format!("{}/{}", pre_path, name);

        let result = Path::new(full_name.as_str()).metadata();
        if result.is_err() {
            self.fatal_error("CacheError", result.err().unwrap().description());
            return false;
        } else {
            let modified = result.unwrap().modified().unwrap();
            let elapsed = modified.elapsed().unwrap().as_secs();
            return elapsed > max_age;
        }
    }
}

#[test]
fn test_is_file_exist() {
    // let exist = is_file_exist("~/CodeSpace/JIT");
    // assert_eq!(exist, false);


    // let exist = is_file_exist("~/Downloads/gradle-icon2.jpf");
    // assert_eq!(exist, true);

    let result = Path::new("/Users/christfm/Downloads/gradle-icon2.jpf").metadata();
    print!("{:?}", result);
    let result = Path::new("./test").metadata();
    print!("{:?}", result);

    let mut file = OpenOptions::new()
        .create(true)
        .write(true)
        .append(true)
        .open("./test")
        .unwrap();

    file.write("ccc".as_bytes());
    file.write("ddd".as_bytes());
    print!("cc")
}


#[test]
fn test_workflow_cache() {
    dotenv::dotenv().ok();

    let mut workflow = AlfredWorkflow::init();
    workflow.cache("test", "just_cache");

    let is_expired = workflow.expired("test", 0);
    assert_eq!(is_expired, false);

    let content = workflow.load("test");
    assert_eq!(content, "just_cache".to_string());

    std::thread::sleep(Duration::new(1, 0));

    let is_expired = workflow.expired("test", 0);
    assert_eq!(is_expired, true);
}

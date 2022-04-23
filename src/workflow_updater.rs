use crate::version::Version;

use serde::{Deserialize, Serialize};
use std::time::UNIX_EPOCH;

// const SYSTEM_ICON_PATH: &str = "/System/Library/CoreServices/CoreTypes.bundle/Contents/Resources/{}";
// this files define the icons provided by the macos system

// features: auto upgrade newer workflow
// the config should be set in workflow env
// url: where to download the newer version
// duration: how often to check whether a newer version
// is_update_check: true/false
// url_type: github
#[derive(Serialize, Deserialize)]
pub struct UpdateConfig {
    release_url: String,
    url_type: String,
    auto_update: bool,
    how_often: u8,
    last_check_time_stamp: u128,

    current_version: Version,
}
pub fn check_update(_current_version: &Version, config: &UpdateConfig) {
    // check the `auto_update` config is on or off
    // exit
    if !config.auto_update {
        return;
    }

    // get the checking history to test whether
    // now should to check the update
    // exit  when checking is done in checking duration
    let time_range: u128 = (config.how_often) as u128 * 24 * 60 * 60 * 1000;
    if config.last_check_time_stamp + time_range
        < std::time::SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_millis()
    {
        return;
    }

    // get the release info
}

pub fn download_update() {}

pub fn install_update() {}

use serde::ser::SerializeSeq;
use std::time::UNIX_EPOCH;
use serde::{Deserialize, Serialize, Serializer};


// version eg: v1.2.3-RC
#[derive(Serialize, Deserialize, PartialEq)]
pub struct Version {
    major: u8,
    minor: u8,
    patch: u8,
    build: Option<String>,
}

const SUFFIX_CANDIDATE: [&str; 2] = ["M", "RC"];

impl Version {
    pub fn new(version_str: &str) -> Result<Version, &'static str> {
        let vp: Vec<&str> = version_str.split(".").collect();
        if vp.len() != 3 {
            return Err("version should be like 1.0.0 or 1.2.3 or 1.2.3-RC");
        }

        let path_with_build = vp.get(2).unwrap();
        let vb: Vec<&str> = path_with_build.split("-").collect();
        if vb.len() > 2 {
            return Err("version should be like 1.0.0 or 1.2.3 or 1.2.3-RC");
        }

        let major_with_prefix = vp.get(0).unwrap();
        let mut major_str = major_with_prefix.to_string();
        if major_str.starts_with("v") {
            major_str = major_str.to_uppercase().replace("V", "");
        }

        let version = Version {
            major: major_str.parse().unwrap(),
            minor: vp.get(1).unwrap().parse().unwrap(),
            patch: vb.get(0).unwrap().parse().unwrap(),
            build: if vb.len() == 2 {
                Some(vb.get(1).unwrap().to_string())
            } else {
                None
            },
        };

        if version.build.is_some() {
            let suffix = version.build.as_ref().unwrap();
            let mut build_valid = false;
            for suf in SUFFIX_CANDIDATE {
                if suffix.starts_with(suf) {
                    build_valid = true;
                }
            }
            if !build_valid {
                return Err("Version suffix should be like RC1 or RC2");
            }
        }
        Ok(version)
    }

    fn compare(&self, comparison: &Version) -> i8 {
        let self_v_num = self.major * 100 + self.minor * 10 + self.patch;
        let comp_v_num = comparison.major * 100 + comparison.minor * 10 + comparison.patch;
        if self_v_num > comp_v_num {
            return 1;
        }

        if self_v_num < comp_v_num {
            return -1;
        }

        // must compare the build
        if self.build.is_some() && comparison.build.is_none() {
            return -1;
        }

        if self.build.is_none() && comparison.build.is_some() {
            return 1;
        }

        if self.build.is_none() && comparison.build.is_none() {
            return 0;
        }

        let self_build_v: u8 = self
            .build
            .as_ref()
            .unwrap()
            .replace("RC", "0")
            .parse()
            .unwrap();
        let comparison_build_v: u8 = comparison
            .build
            .as_ref()
            .unwrap()
            .replace("RC", "0")
            .parse()
            .unwrap();

        return if self_build_v == comparison_build_v {
            0
        } else if self_build_v > comparison_build_v {
            1
        } else {
            -1
        };
    }

    pub fn gt(&self, com: &Version) -> bool {
        if self.compare(com) == 1 {
            true
        } else {
            false
        }
    }

    pub fn eq(&self, com: &Version) -> bool {
        if self.compare(com) == 0 {
            true
        } else {
            false
        }
    }

    pub fn lt(&self, com: &Version) -> bool {
        !self.gt(com)
    }
}

#[cfg(test)]
mod version_unit_test {
    use crate::version::Version;

    #[test]
    fn test_parse_version() {
        let v = Version::new("1.2.3").unwrap();
        assert_eq!(v.major, 1);
        assert_eq!(v.minor, 2);
        assert_eq!(v.patch, 3);
    }

    #[test]
    fn test_parse_version_with_invalid() {
        let v = Version::new("r1.2.3");
        assert_eq!(v.is_err(), true);

        let v2 = Version::new("1.2.3.RC");
        assert_eq!(v2.is_err(), true);

        let v3 = Version::new("1.2.3-RCX");
        assert_eq!(v3.is_err(), true);

        let v4 = Version::new("1.2.3.4-RC");
        assert_eq!(v4.is_err(), true);
    }

    #[test]
    fn test_parse_version_with_build() {
        let v = Version::new("1.2.3-RC").unwrap();
        assert_eq!(v.major, 1);
        assert_eq!(v.minor, 2);
        assert_eq!(v.patch, 3);
        assert_eq!(v.build, Some("RC".to_string()));
    }

    #[test]
    fn test_parse_version_with_prefix() {
        let v = Version::new("v1.2.3").unwrap();
        assert_eq!(v.major, 1);
        assert_eq!(v.minor, 2);
        assert_eq!(v.patch, 3);
    }

    #[test]
    fn test_parse_version_with_prefix_build() {
        let v = Version::new("v1.2.3-RC").unwrap();
        assert_eq!(v.major, 1);
        assert_eq!(v.minor, 2);
        assert_eq!(v.patch, 3);
        assert_eq!(v.build, Some("RC".to_string()))
    }

    #[test]
    fn test_version_newer() {
        let v = Version::new("1.2.3").unwrap();
        let v2 = Version::new("2.2.3").unwrap();
        assert_eq!(v.compare(&v2), -1);

        let v3 = Version::new("1.3.3").unwrap();
        assert_eq!(v.compare(&v3), -1);

        let v4 = Version::new("1.2.4").unwrap();
        assert_eq!(v.compare(&v4), -1);
    }

    #[test]
    fn test_version_newer_with_prefix() {
        let v = Version::new("v1.2.3").unwrap();
        let v2 = Version::new("v2.2.3").unwrap();
        assert_eq!(v.compare(&v2), -1);

        let v3 = Version::new("v1.3.3").unwrap();
        assert_eq!(v.compare(&v3), -1);

        let v4 = Version::new("1.2.4").unwrap();
        assert_eq!(v.compare(&v4), -1);
    }

    #[test]
    fn test_version_newer_with_build() {
        let v = Version::new("v1.2.3").unwrap();
        let v2 = Version::new("v2.2.3").unwrap();
        assert_eq!(v.compare(&v2), -1);

        let v3 = Version::new("v1.3.3").unwrap();
        assert_eq!(v.compare(&v3), -1);

        let v4 = Version::new("1.2.4").unwrap();
        assert_eq!(v.compare(&v4), -1);

        let v5 = Version::new("1.2.3-RC").unwrap();
        assert_eq!(v.compare(&v5), 1);

        let v6 = Version::new("1.2.3-RC1").unwrap();
        assert_eq!(v6.compare(&v5), 1);

        let v7 = Version::new("1.2.3-RC2").unwrap();
        assert_eq!(v7.compare(&v6), 1);
    }

    #[test]
    fn test_version_equal() {
        let v = Version::new("1.2.3").unwrap();
        let v1 = Version::new("v1.2.3").unwrap();
    }
}

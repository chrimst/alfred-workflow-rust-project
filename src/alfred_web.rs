use crate::alfred::Alfred;
use reqwest::blocking::Response;
use reqwest::header::HeaderMap;
use serde::Serialize;
use pbkdf2::{
    password_hash::{PasswordHasher, Salt},
    Algorithm, Params, Pbkdf2,
};
use url::Url;

impl Alfred {
    pub fn get<T: Serialize + ?Sized>(url: &str, params: &T) -> Response {
        reqwest::blocking::Client::builder()
            .build()
            .unwrap()
            .get(url)
            .query(params)
            .send()
            .unwrap()
    }

    pub fn post<T: Serialize + ?Sized>(url: &str, headers: HeaderMap) -> Response {
        reqwest::blocking::Client::builder()
            .build()
            .unwrap()
            .post(url)
            .headers(headers)
            .send()
            .unwrap()
    }

    pub fn get_chrome_cookie(url:&str) {
        let chrome_encrypt_key = security_framework::passwords::get_generic_password("Chrome Safe Storage", "Chrome")
            .unwrap_or_default();
        let salt = Salt::new("c2FsdHlzYWx0").unwrap();
        let params = Params {
            rounds: 1003,
            output_length: 16,
        };
        let saltKey = Pbkdf2.hash_password_customized(
                &chrome_encrypt_key,
                Some(Algorithm::Pbkdf2Sha1.ident()),
                None,
                params,
                salt,
            )
            .unwrap()
            .hash
            .unwrap()
            .as_ref()
            .to_vec();
    }
}

fn get_chrome_cookie(url: &str, profile: &str) -> Vec<ChromeCookie> {
    // parse url
    let url = Url::parse(url).unwrap();
    let host = url.domain().unwrap();

    // build cookie sql
    let cookie_sql = build_cookie_query_sql(host);

    // open chrome cookie db
    // ${process.env.HOME}/Library/Application Support/Google/Chrome/${PROFILE}/Cookies
    let home_path = std::env::var("HOME").unwrap();
    let chrome_cookie_db_path =
        home_path + "/Library/Application Support/Google/Chrome/" + profile + "/Cookies";
    let conn = sqlite::open(chrome_cookie_db_path).unwrap();

    // read cookie
    let mut cookie_stmt = conn.prepare(cookie_sql).unwrap().into_cursor();
    let mut cookie_rs: Vec<ChromeCookie> = Vec::new();
    while let Some(state) = cookie_stmt.next().unwrap() {
        let cookie = ChromeCookie {
            creation_utc: state[0].as_integer().unwrap(),
            host_key: String::from(state[1].as_string().unwrap()),
            top_frame_site_key: String::from(state[2].as_string().unwrap()),
            name: String::from(state[3].as_string().unwrap()),
            value: String::from(state[4].as_string().unwrap()),
            encrypted_value: state[5].as_binary().unwrap().to_vec(),
            path: String::from(state[6].as_string().unwrap()),
            expires_utc: state[7].as_integer().unwrap(),
            is_secure: state[8].as_integer().unwrap(),
            is_httponly: state[9].as_integer().unwrap(),
            last_access_utc: state[10].as_integer().unwrap(),
            has_expires: state[11].as_integer().unwrap(),
            is_persistent: state[12].as_integer().unwrap(),
            priority: state[13].as_integer().unwrap(),
            samesite: state[14].as_integer().unwrap(),
            source_scheme: state[15].as_integer().unwrap(),
            source_port: state[16].as_integer().unwrap(),
            is_same_part: state[17].as_integer().unwrap(),
        };
        cookie_rs.push(cookie)
    }
    return cookie_rs;
}

#[derive(Debug, Eq, Hash, PartialEq, PartialOrd, Ord)]
pub struct ChromeCookie {
    creation_utc: i64,
    host_key: String,
    top_frame_site_key: String,
    name: String,
    value: String,
    encrypted_value: Vec<u8>,
    path: String,
    expires_utc: i64,
    is_secure: i64,
    is_httponly: i64,
    last_access_utc: i64,
    has_expires: i64,
    is_persistent: i64,
    priority: i64,
    samesite: i64,
    source_scheme: i64,
    source_port: i64,
    is_same_part: i64,
}

fn build_cookie_query_sql(host: &str) -> String {
    let mut part_1 = "select creation_utc,
            host_key,
            top_frame_site_key,
            name,
            value,
            encrypted_value,
            path,
            expires_utc,
            is_secure,
            is_httponly,
            last_access_utc,
            has_expires,
            is_persistent,
            priority,
            samesite,
            source_scheme,
            source_port,
            is_same_party
     FROM cookies
     where host_key like '%"
        .to_string();
    part_1.push_str(host);
    part_1.push_str("%'ORDER BY LENGTH(path) DESC, creation_utc ASC;");
    return part_1;
}

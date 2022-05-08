use crate::alfred::Alfred;
use reqwest::blocking::Response;
use reqwest::header::HeaderMap;
use serde::Serialize;

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
}
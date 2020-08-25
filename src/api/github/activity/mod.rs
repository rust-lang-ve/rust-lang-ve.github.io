use crate::api::common::RawFetchCallback;
use yew::format::Nothing;
use yew::services::fetch::{FetchService, FetchTask, Request};

pub mod serde;
pub mod types;

pub const GITHUB_EVENTS: &str = "https://api.github.com/users/rust-lang-ve/events";

pub fn get_activity(callback: RawFetchCallback<String>) -> FetchTask {
    let req = Request::get(GITHUB_EVENTS).body(Nothing).unwrap();

    FetchService::fetch(req, callback).unwrap()
}

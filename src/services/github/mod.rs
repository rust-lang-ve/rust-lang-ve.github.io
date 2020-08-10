use anyhow::{anyhow, Error};
use serde_derive::Deserialize;
use yew::callback::Callback;
use yew::format::{Json, Nothing};
use yew::services::fetch::{FetchService, FetchTask, Request, Response};

const GITHUB_REPOS_URL: &str = "https://api.github.com/orgs/rust-lang-ve/repos";

#[derive(Deserialize, Debug)]
pub struct GitHubRepository {
  pub name: String,
  pub description: String,
  pub language: String,
}

pub type Repositories = Vec<GitHubRepository>;

#[derive(Default)]
pub struct GitHubService;

impl GitHubService {
  pub fn get_repos(&mut self, callback: Callback<Result<Repositories, Error>>) -> FetchTask {
    let handler = move |response: Response<Json<Result<Repositories, Error>>>| {
      let (meta, Json(data)) = response.into_parts();

      if meta.status.is_success() {
        callback.emit(data);
      } else {
        callback.emit(Err(anyhow!(
          "{}: error getting repositories!",
          meta.status
        )))
      }
    };

    let request = Request::get(GITHUB_REPOS_URL).body(Nothing).unwrap();

    FetchService::fetch(request, handler.into()).unwrap()
  }
}

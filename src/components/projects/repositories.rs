use anyhow::Error;  
use serde::Deserialize;
use yew::prelude::*;
use yew::format::{Nothing, Json};
use yew::services::fetch::{FetchService, FetchTask, Request, Response};

/// URL to fetch repositories for **rust-lang-ve** organization from GitHub
// const GITHUB_REPOS_URL: &str = "https://api.github.com/orgs/rust-lang-ve/repos";
const GITHUB_REPOS_URL: &str = "http://127.0.0.1:8080/gh_org_repos.json";

pub struct Repositories {
  is_fetching: bool,
  fetch_failed: bool,
  fetch_task: Option<FetchTask>,
  has_repos: bool,
  link: ComponentLink<Self>,
  repositories: Option<Vec<Repository>>,
}

#[derive(Deserialize, Debug)]
pub struct Repository {
  pub title: String,
  pub description: String,
  pub lang: String,
}

pub enum Msg {
  FetchFailed,
  FetchSuccess(Vec<Repository>),
}

impl Component for Repositories {
  type Message = Msg;
  type Properties = ();

  fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
    Self {
      is_fetching: false,
      fetch_failed: false,
      fetch_task: None,
      has_repos: false,
      link,
      repositories: None,
    }
  }

  fn update(&mut self, msg: Self::Message) -> ShouldRender {
    match msg {
      Msg::FetchSuccess(repos) => {
        if repos.len() > 0 {
          self.has_repos = true;
        }

        self.is_fetching = false;
        self.repositories = Some(repos);

        return true;
      },
      Msg::FetchFailed => {
        self.is_fetching = false;
        self.fetch_failed = true;

        return true;
      }
    }
  }

  fn change(&mut self, _: Self::Properties) -> ShouldRender {
    true
  }

  fn rendered(&mut self, _first_render: bool) {
    if _first_render {
      self.is_fetching = true;

      let callback = self.link.callback(move |response: Response<Json<Result<Vec<Repository>, Error>>>| {
        if let (meta, Json(Ok(body))) = response.into_parts() {
          if meta.status.is_success() {
            Msg::FetchSuccess(body);
          }
        }

        Msg::FetchFailed
      });

      let request = Request::builder()
        .method("GET")
        .uri(GITHUB_REPOS_URL.to_string())
        .body(Nothing)
        .unwrap();

      let task = FetchService::fetch(request, callback).unwrap();
      self.fetch_task = Some(task);
    }
  }

  fn view(&self) -> Html {
    fn render_repos() -> Html {
      html!{
        <ul>
          <li>{"Hello"}</li>
        </ul>
      }
    }

    fn render_failed_to_fetch() -> Html {
      html! {
        <h3>{ "Failed to gather repos from GitHub!" }</h3>
      }
    }

    if !self.is_fetching && self.has_repos {
      render_repos();
    }

    if self.fetch_failed {
      render_failed_to_fetch();
    }

    html! {
      <h1>{"Here the repos!"}</h1>
    }
  }
}

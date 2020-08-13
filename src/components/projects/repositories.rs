use anyhow::Error;
use serde::Deserialize;
use serde_json;
use yew::format::Nothing;
use yew::prelude::*;
use yew::services::fetch::{FetchService, FetchTask, Request, Response};
use yew::services::ConsoleService;

/// URL to fetch repositories for **rust-lang-ve** organization from GitHub
// const GITHUB_REPOS_URL: &str = "https://api.github.com/orgs/rust-lang-ve/repos";
const GITHUB_REPOS_URL: &str = "http://127.0.0.1:8080";

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
  pub name: String,
  pub description: String,
  pub language: String,
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
      }
      Msg::FetchFailed => {
        self.is_fetching = false;
        self.fetch_failed = true;

        return true;
      }
    }
  }

  fn change(&mut self, _: Self::Properties) -> ShouldRender {
    false
  }

  fn rendered(&mut self, _first_render: bool) {
    if _first_render {
      self.is_fetching = true;

      let callback = self
        .link
        .callback(move |response: Response<Result<String, Error>>| {
          let (meta, body) = response.into_parts();

          if meta.status.is_success() {
            match body {
              Ok(body) => {
                ConsoleService::info("Fetched repositories");
                let body = body.as_str();

                match serde_json::from_str::<Vec<Repository>>(body) {
                  Ok(repos) => {
                    return Msg::FetchSuccess(repos);
                  }
                  Err(_) => ConsoleService::error("Failed to deserialize response!"),
                }
              }
              Err(err) => {
                ConsoleService::error(err.to_string().as_str());
              }
            }
          }

          Msg::FetchFailed
        });

      let request = Request::get(GITHUB_REPOS_URL).body(Nothing).unwrap();

      let task = FetchService::fetch(request, callback).unwrap();
      self.fetch_task = Some(task);
    }
  }

  fn view(&self) -> Html {
    fn render_failed_to_fetch() -> Html {
      html! {
        <h3>{ "Failed to gather repos from GitHub!" }</h3>
      }
    }

    if self.fetch_failed {
      render_failed_to_fetch()
    } else if !self.is_fetching && self.has_repos {
      match &self.repositories {
        Some(repos) => {
          fn render_repo(name: &str, description: &str, language: &str) -> Html {
            html! {
              <li class="repository">
                <main>
                  <strong>{ name }</strong>
                  <p>{ description }</p>
                </main>
                <footer>
                  <span>{ language }</span>
                </footer>
              </li>
            }
          }

          html! {
            <ul class="content" id="repositories">
              {
                for repos.iter().map(|repo| {
                  render_repo(&repo.name, &repo.description, &repo.language)
                })
              }
            </ul>
          }
        },
        _ => {
          Html::default()
        }
      }
    } else {
      html! {
        <h1>{"Nothing to see here!"}</h1>
      }
    }
  }
}

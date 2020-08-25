use serde_derive::{Deserialize, Serialize};
use std::default;

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
pub struct Actor {
    pub display_login: String,
    pub url: String,
    pub avatar_url: String,
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
pub enum Event {
    PushEvent(PushEventBody),
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
pub struct PushEventPayload {
    pub commits: Vec<Commit>,
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
pub struct PushEventBody {
    pub id: String,
    pub actor: Actor,
    pub repo: Repository,
    pub payload: PushEventPayload,
    pub created_at: String,
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
pub struct Activity(pub Vec<Event>);

impl default::Default for Activity {
    fn default() -> Self {
        Self(Vec::new())
    }
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
pub struct Repository {
    pub id: i32,
    pub name: String,
    pub url: String,
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
pub struct Commit {
    pub message: String,
}

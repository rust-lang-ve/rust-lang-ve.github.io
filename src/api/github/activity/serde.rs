use crate::api::github::activity::types::Event;
use serde_json::{from_str, from_value, Error as SerdeError, Value};

pub fn deserialize(data: String) -> Result<Vec<Event>, SerdeError> {
    let mut events: Vec<Event> = Vec::default();

    match from_str::<Vec<Value>>(data.as_str()) {
        Ok(deserialized) => {
            deserialized.into_iter().for_each(|val| {
                if val.is_object() {
                    if let Ok(push_event) = from_value(val) {
                        events.push(Event::PushEvent(push_event))
                    }
                }
            });
        }
        Err(err) => {
            return Err(err);
        }
    }

    Ok(events)
}

#[cfg(test)]
mod tests {
    use super::*;

    const DATA: &str = r#"[
      {
        "id": "13321814824",
        "type": "PushEvent",
        "actor": {
          "id": 34756077,
          "login": "EstebanBorai",
          "display_login": "EstebanBorai",
          "gravatar_id": "",
          "url": "https://api.github.com/users/EstebanBorai",
          "avatar_url": "https://avatars.githubusercontent.com/u/34756077?"
        },
        "repo": {
          "id": 283040238,
          "name": "rust-lang-ve/rust-lang-ve.github.io",
          "url": "https://api.github.com/repos/rust-lang-ve/rust-lang-ve.github.io"
        },
        "payload": {
          "push_id": 5591528093,
          "size": 1,
          "distinct_size": 1,
          "ref": "refs/heads/feat/github-activity",
          "head": "208e0d598cbbba475ac5a862851d2b0f6db1612b",
          "before": "a8e78336373968e1c47013add2f17ea6ecf4cdff",
          "commits": [
            {
              "sha": "208e0d598cbbba475ac5a862851d2b0f6db1612b",
              "author": {
                "email": "estebanborai@gmail.com",
                "name": "Esteban Borai"
              },
              "message": "feat: deserialize multiple types",
              "distinct": true,
              "url": "https://api.github.com/repos/rust-lang-ve/rust-lang-ve.github.io/commits/208e0d598cbbba475ac5a862851d2b0f6db1612b"
            }
          ]
        },
        "public": true,
        "created_at": "2020-08-27T02:20:13Z",
        "org": {
          "id": 68873317,
          "login": "rust-lang-ve",
          "gravatar_id": "",
          "url": "https://api.github.com/orgs/rust-lang-ve",
          "avatar_url": "https://avatars.githubusercontent.com/u/68873317?"
        }
      }
    ]
    "#;

    #[test]
    fn it_deserializes_push_event() {
        let events = deserialize(DATA.to_string()).unwrap();

        match events.get(0).unwrap().to_owned() {
            Event::PushEvent(value) => {
                assert_eq!(
                    value.repo.name,
                    String::from("rust-lang-ve/rust-lang-ve.github.io")
                );
            }
        }
    }
}

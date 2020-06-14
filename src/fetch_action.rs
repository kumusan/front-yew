use anyhow::{anyhow, Error};
use yew::callback::Callback;
use yew::format::{Json, Nothing};
use yew::services::fetch::{FetchService, FetchTask, Request, Response};
use serde_derive::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Todo {
    entry: Vec<Entry>,
}

#[derive(Deserialize, Debug)]
pub struct Entry {
    id: i32,
    title: String,
    body: String,
    done: bool,
}

#[derive(Default)]
pub struct RequestTodo {}

impl RequestTodo {
    pub fn new() -> Self {
        Self {}
    }

    pub fn all(&mut self, callback: Callback<Result<Todo, Error>>) -> FetchTask {
        let url = format!("http://localhost:8000/todos");
        let handler = move |response: Response<Json<Result<Todo, Error>>>| {
            let (meta, Json(data)) = response.into_parts();
            if meta.status.is_success() {
                callback.emit(data)
            } else {
                callback.emit(Err(anyhow!(
                    "{}: error get",
                    meta.status
                )))
            }
        };
        let request = Request::get(url.as_str()).body(Nothing).unwrap();
        FetchService::new().fetch(request, handler.into()).unwrap()
    }
}
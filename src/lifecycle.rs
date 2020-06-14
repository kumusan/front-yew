use yew::{html, Component, ComponentLink, Html, ShouldRender};
use yew::services::fetch::{FetchService, FetchTask, Request, Response};
use yew::format::{Json, Nothing};
use yew::callback::Callback;
use anyhow::Error;

use crate::fetch_action::{Todo, RequestTodo};

pub struct Model { 
    link: ComponentLink<Model>,
    fetch_todo: RequestTodo,
    fetching: bool,
    callback: Callback<Result<Todo, Error>>,
    todos: Option<Vec<Todo>>,
    select_value: Option<i32>,
    task: Option<FetchTask>
}


pub enum Action {
    All,
    FetchReady(Result<Todo, Error>),
}

impl Component for Model {

    type Message = Action;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        Model {
            link: link.clone(),
            fetch_todo: RequestTodo::new(),
            fetching: false,
            callback: link.callback(Action::FetchReady),
            todos: None,
            select_value: None,
            task: None,
        }
    }

    fn update(&mut self, act: Self::Message) -> ShouldRender {
        match act {
            Action::All => {
                self.fetching = true;
                let task = self.fetch_todo.all(self.callback.clone());
                self.task = Some(task);
            }

            Action::FetchReady(Ok(todos)) => {
                self.fetching = false;
                self.todos = Some(vec![todos]);
            }

            Action::FetchReady(Err(_)) => {}
        }
        true
    }

    fn change(&mut self, _: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        html! {
            <h1>{"tetetetete"}</h1>
        }
    }
}
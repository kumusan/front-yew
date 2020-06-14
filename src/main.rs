extern crate yew;

mod fetch_action;
mod lifecycle;
use lifecycle::Model;

fn main() {
    yew::start_app::<Model>();
}

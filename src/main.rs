#![allow(non_snake_case)]

use dioxus::prelude::*;
use dioxus_logger::tracing::{info, Level};

#[derive(Clone, Routable, Debug, PartialEq)]
enum Route {
    #[route("/")]
    Home {},
    #[route("/blog/:id")]
    Blog { id: i32 },
}

fn main() {
    if cfg!(debug_assertions) {
        // Init logger
        dioxus_logger::init(Level::INFO).expect("failed to init logger");
        info!("starting app");
    }
    launch(App);
}

fn App() -> Element {
    rsx! {
        Router::<Route> {}
    }
}

#[component]
fn Blog(id: i32) -> Element {
    rsx! {
        Link {
            class: "inline-block rounded p-1 m-2 border border-yellow-400 bg-yellow-200",
            to: Route::Home {},
            "Go to counter"
        }
        h1 {
            class: "text-xl m-2",
            "Blog post ",
            span {
                class: "text-red-600 font-bold",
                "{id}"
            }
        }
    }
}

#[component]
fn Home() -> Element {
    let mut count = use_signal(|| 0);

    rsx! {
        Link {
            class: "inline-block rounded p-1 m-2 border border-indigo-400 bg-indigo-200",
            to: Route::Blog {
                id: count()
            },
            "Go to blog"
        }
        div {
            h1 {
                class: "text-xl m-2",
                "counter: ",
                span {
                    class: "text-red-600 font-bold",
                    "{count}"
                }
            }
            button {
                class: "inline-block rounded p-1 m-2 border border-pink-400 bg-pink-200",
                onclick: move |_| count += 1, 
                "Up high!"
            }
            button {
                class: "inline-block rounded p-1 m-2 border border-green-400 bg-green-200",
                onclick: move |_| count -= 1,
                "Down low!"
            }
        }
    }
}

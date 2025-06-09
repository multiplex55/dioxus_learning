use dioxus::prelude::*;
use dioxus_router::prelude::*;
use dioxus_signals::*;

fn main() {
    dioxus::launch(App);
}

#[derive(Clone, Routable, PartialEq)]
enum Route {
    #[route("/")]
    Home {},
    #[route("/about")]
    About {},
}

#[derive(Clone)]
struct SharedState {
    count: Signal<i32>,
}

#[component]
fn App() -> Element {
    use_context_provider(|| SharedState { count: Signal::new(0) });

    rsx! {
        Router::<Route> {}
    }
}

#[component]
fn Home() -> Element {
    let mut state = use_context::<SharedState>().count;

    rsx! {
        div {
            h1 { "Welcome to Dioxus!" }
            button { onclick: move |_| state += 1, "Clicked {state} times" }
            Link { to: Route::About {}, "About" }
        }
    }
}

#[component]
fn About() -> Element {
    let state = use_context::<SharedState>().count;

    rsx! {
        div {
            h1 { "About" }
            p { "The counter is at {state}" }
            Link { to: Route::Home {}, "Home" }
        }
    }
}

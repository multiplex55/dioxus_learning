use dioxus::prelude::*;

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    let mut count = use_signal(|| 0);

    rsx! {
        div {
            h1 { "Welcome to Dioxus!" }
            button { onclick: move |_| count += 1, "Clicked {count} times" }
        }
    }
}

mod console;
mod prelude;
mod help;

use crate::{
    prelude::*,
    help::Menu,
};

#[derive(Debug, Clone, Routable, PartialEq)]
#[rustfmt::skip]
pub enum Route {
    #[layout(crate::console::ConsoleLine)]
    #[route("/")]
    Home {},
    #[route("/menu")]
    Menu {},
}

const MAIN_CSS: Asset = asset!("/assets/main.css");

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    rsx! {
        document::Link { rel: "stylesheet", href: MAIN_CSS }
        Router::<Route> {}
    }
}

#[component]
pub fn Home() -> Element {
    focus_console!();
    rsx! {
        div {
            id: "hero",
            div {
                h1 { "Sebastian Hyland" }
                h2 { "Biomedical engineer with interests in bioinformatics, embedded systems, distributed compute, and optimization" }
                p { "This site is currently under construction."}
            }
        }
    }
}


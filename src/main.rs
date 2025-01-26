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
const FONT_API: &str = "https://fonts.googleapis.com/css2?family=IBM+Plex+Mono:ital,wght@0,100;0,200;0,300;0,400;0,500;0,600;0,700;1,100;1,200;1,300;1,400;1,500;1,600;1,700&display=swap";

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    rsx! {
        document::Stylesheet { href: MAIN_CSS }
        document::Style {
            r#"
                body {{
                    background-color: #0f1116;
                    color: #ffffff;
                }}
            "#
        }
        document::Stylesheet { href: FONT_API }
        Router::<Route> {}
    }
}

#[component]
pub fn Home() -> Element {
    focus_console!();
    let detected_platform = web_sys::window()
        .map(|w| w.navigator())
        .and_then(|n| n.platform().ok())
        .unwrap_or("".to_string());
    rsx! {
        div {
            id: "hero",
            div {
                h1 { "Sebastian Hyland" }
                h2 { "UBC School of Biomedical Engineering" }
                p { "Interests:" }
                ul {
                    li { "Bioinformatics" }
                    li { "Embedded systems/firmware" }
                    li { "Distributed compute" }
                    li { "Optimization" }
                }
                p { "This site is currently under construction." }
                p { "Detected platform: " b { "{detected_platform}" } }
            }
        }
    }
}


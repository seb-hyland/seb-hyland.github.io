use crate::prelude::*;


#[component]
pub fn Menu() -> Element {
    focus_console!();
    rsx! {
        div {
            id: "help",
            p { "Navigate by entering page names into the console" }
            br {}

            p { "." }
            p { "├─── " Link { to: Route::Home {}, "home" }}
            p { "└─── " Link { to: Route::Menu {}, "menu" }}
            br {}

            p { "ext" }
            p { "└─── " Link { to: "https://github.com/seb-hyland/", "github" }}
        }
    }
}


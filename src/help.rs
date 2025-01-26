use crate::prelude::*;


#[component]
pub fn Menu() -> Element {
    focus_console!();
    rsx! {
        div {
            id: "help",
            if !is_mobile() {
                p { "Navigate by entering page names into the console." }
                p { span { class: "keypress", "TAB" } " can be used for completion"}
                br {}
            }
            div {
                id: "help-grid",
                div {
                    p { style: "margin-bottom: 5px;", "pages" }
                    p { "│ "}
                    p { "├─── " Link { to: Route::Home {}, "home" } }
                    p { "│ "}
                    p { "└─── " Link { to: Route::Menu {}, "menu" } }
                    br {}
                }
                
                div {
                    p { style: "margin-bottom: 5px;", "external" }
                    p { "│ "}
                    p { "├─── " Link { to: "https://github.com/seb-hyland/", "github" } }
                    p { "│ "}
                    p { "└─── " Link { to: "https://github.com/seb-hyland/seb-hyland.github.io", "source" } }
                }
            }
        }
    }
}


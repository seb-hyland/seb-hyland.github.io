use crate::prelude::*;
use std::{
    sync::LazyLock,
    collections::HashMap,
};

const CONSOLE: GlobalSignal<String> = Global::new(|| "".to_string());
const INFO_STR: (&str, &str, &str) = ("", "To see all available pages, enter ", "menu");
const INTERN_PAGES: LazyLock<HashMap<&'static str, Route>> = LazyLock::new(|| {
    let mut map = HashMap::new();
    map.insert("home", Route::Home {});
    map.insert("menu", Route::Menu {});
    map
});
const EXTERN_PAGES: LazyLock<HashMap<&'static str, &'static str>> = LazyLock::new(|| {
    let mut map = HashMap::new();
    map.insert("github", "https://github.com/seb-hyland/");
    map.insert("source", "https://github.com/seb-hyland/seb-hyland.github.io");
    map
});

#[component]
pub fn ConsoleLine() -> Element {
    let mut instructions = use_signal(|| INFO_STR);
    instructions.set(update_instructions());
    rsx! {
        div {
            id: "console",
            tabindex: "0",
            onkeydown: move |event| { 
                event.prevent_default();
                parse_keypress(&event.key()); 
            },
            p {
                style: "margin-left: 14px",
                "> {CONSOLE}█" 
            }
            p {
                id: "instructions",
                p {
                    if !instructions().0.is_empty() {
                        span {
                            class: "keypress",
                            "{instructions().0}" 
                        }
                    }
                    "{instructions().1}" 
                    b { "{instructions().2}" } 
                }
            }
        }
        Outlet::<Route> {}
    }
}


fn parse_keypress(key: &Key) {
    let _ = document::eval(
        r#"
            const console = document.getElementById("console");
            console.style.borderColor = "var(--text-color)";
        "#,
    );
    match key {
        Key::Enter => parse_command(),
        Key::Backspace => console_backspace(),
        Key::Tab => complete(),
        Key::Character(c) => *CONSOLE.write() += c,
        _ => {},
    }
}

fn console_backspace() {
    CONSOLE.write().pop();
}

fn parse_command() {
    let cmd: &str = &CONSOLE();
    if let Some(r) = INTERN_PAGES.get(cmd) {
        push_and_flush(r);
    }
    else if let Some(l) = EXTERN_PAGES.get(cmd) {
        new_tab(l);
    }
    else {
        let _ = document::eval(
            r#"
                const console = document.getElementById("console");
                console.style.borderColor = "var(--accent-red)";
            "#,
        );
    }
}

fn push_and_flush(target: &Route) {
    let nav = navigator();
    nav.push(target.clone());
    *CONSOLE.write() = "".to_string();
}


fn new_tab(target: &str) {
    document::eval(&format!(r#"window.open("{}", "_blank");"#, target));
    *CONSOLE.write() = "".to_string();
}


fn update_instructions() -> (&'static str, &'static str, &'static str) {
    let input: &str = &CONSOLE();
    let current_route = use_route::<Route>();
    if input.is_empty() && current_route == (Route::Home {}) {
        INFO_STR
    }
    else if let Some(_) = INTERN_PAGES.get(input) {
        ("↵", " to open page", "") 
    }
    else if let Some(_) = EXTERN_PAGES.get(input) {
        ("↵", " to open external page", "") 
    }
    else {
        ("", "", "")
    }
}


fn complete() {
    let input = CONSOLE();
    let mut completion = None;

    if let Some(internal) = completion_searcher(&INTERN_PAGES, &input) {
        completion = Some(internal)
    } else if let Some(external) = completion_searcher(&EXTERN_PAGES, &input) {
        completion = Some(external)
    }

    if let Some(c) = completion {
        *CONSOLE.write() = c.to_string();
    } else {
        let _ = document::eval(
            r#"
                const console = document.getElementById("console");
                console.style.borderColor = "var(--accent-red)";
            "#,
        );
    }
}


fn completion_searcher<T>(map: &LazyLock<HashMap<&'static str, T>>, input: &str) -> Option<&'static str> {
    map
        .keys()
        .filter(|cand| cand.starts_with(input))
        .min_by_key(|cand| cand.len())
        .copied() 
}



#[macro_export]
macro_rules! focus_console {
    () => {
        use_effect(move || {
            let _ = document::eval(
                r#"
                    const console = document.getElementById("console");
                    console.focus();
                    console.addEventListener("blur", () => {
                        console.style.color = "var(--accent-lg)";
                        console.style.borderColor = "var(--accent-lg)";
                    });
                    console.addEventListener("focus", () => {
                        console.style.color = "var(--text-color)";
                        console.style.borderColor = "var(--text-color)";
                    });
                "#,
            );
        });
    };
}


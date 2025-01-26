use crate::prelude::*;
use std::{
    sync::LazyLock,
    collections::HashMap,
};
use web_sys::{
    HtmlElement,
    window,
    wasm_bindgen::JsCast,
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
            onfocus: |_| {
                style_console(Some(Color::White), Some(Color::White));
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
    style_console(Some(Color::White), None);
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
        style_console(Some(Color::Red), None);
    }
}


fn push_and_flush(target: &Route) {
    let nav = navigator();
    nav.push(target.clone());
    *CONSOLE.write() = "".to_string();
}


fn new_tab(target: &str) {
    if let Some(win) = window() {
        let _ = win.open_with_url_and_target(target, "_blank");
    }
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
        style_console(Some(Color::Red), None);
    }
}


fn completion_searcher<T>(map: &LazyLock<HashMap<&'static str, T>>, input: &str) -> Option<&'static str> {
    if input.is_empty() {
        Some("home")
    } else {
        map
            .keys()
            .filter(|cand| cand.starts_with(input))
            .min_by_key(|cand| cand.len())
            .copied() 
    }
}


pub enum Color {
    White,
    Grey,
    Red
}

pub fn style_console(border_color: Option<Color>, text_color: Option<Color>) {
    let style = window()
        .and_then(|win| win.document())
        .and_then(|doc| doc.get_element_by_id("console"))
        .and_then(|element| element.dyn_into::<HtmlElement>().ok())
        .map(|console| {
            console.style()
        });
    let css_var = |c: Color| -> &'static str {
        match c {
            Color::White => "var(--text-color)",
            Color::Grey => "var(--accent-lg)",
            Color::Red => "var(--accent-red)",
        }
    };
    
    if let Some(s) = style {
        if let Some(c) = border_color {
            let color_var = css_var(c);
            let _ = s.set_property("border-color", color_var);
        }
        if let Some(c) = text_color {
            let color_var = css_var(c);
            let _ = s.set_property("color", color_var);
        }
    }
}


#[macro_export]
macro_rules! focus_console {
    () => {
        use web_sys::{
            HtmlElement,
            window,
            wasm_bindgen::{
                JsCast,
                closure::Closure,
            },
        };
        use crate::console::{
            Color,
            style_console,
        };
        
        let set_inactive = Closure::once(|| style_console(Some(Color::Grey), Some(Color::Grey)));
        use_effect(move || {
            let elem = window()
                .and_then(|win| win.document())
                .and_then(|doc| doc.get_element_by_id("console"))
                .and_then(|element| element.dyn_into::<HtmlElement>().ok());
            if let Some(e) = elem {
                let _ = e.focus();
                let _ = e.add_event_listener_with_callback("blur", set_inactive.as_ref().unchecked_ref());
            }}
        );
    };
}


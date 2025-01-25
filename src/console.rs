use crate::prelude::*;

const CONSOLE: GlobalSignal<String> = Global::new(|| "".to_string());
const INFO_STR: (&str, &str, &str) = ("", "To see all available pages, enter ", "menu");

#[component]
pub fn ConsoleLine() -> Element {
    let mut instructions = use_signal(|| INFO_STR);
    instructions.set(update_instructions());
    rsx! {
        div {
            id: "console",
            tabindex: "0",
            onkeydown: move |event| parse_keypress(&event.key()),
            p {
                style: "margin-left: 14px",
                "> {CONSOLE}█" 
            }
            p {
                id: "instructions",
                p {
                    if !instructions().0.is_empty() {
                        span { 
                            style: "
                                color: white; 
                                background-color: #333;
                                border-radius: 5px;
                                padding: 4px 8px;
                            ", 
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
            console.style.borderColor = "white";
        "#,
    );
    match key {
        Key::Enter => parse_command(),
        Key::Backspace => console_backspace(),
        Key::Character(c) => *CONSOLE.write() += c,
        _ => {},
    }
}

fn console_backspace() {
    CONSOLE.write().pop();
}

fn parse_command() {
    let cmd: &str = &CONSOLE();
    if let Some(r) = is_page(cmd) {
        push_and_flush(r);
    }
    else if let Some(l) = is_extern(cmd) {
        new_tab(l);
    }
    else {
        let _ = document::eval(
            r#"
                const console = document.getElementById("console");
                console.style.borderColor = "red";
            "#,
        );
    }
}


fn is_page(input: &str) -> Option<Route> {
    match input {
        "home" => Some(Route::Home {}),
        "menu" => Some(Route::Menu {}),
        _ => None,
    }
}


fn is_extern(input: &str) -> Option<&'static str> {
    match input {
        "github" => Some("https://github.com/seb-hyland/"),
        _ => None,
    }
}


fn push_and_flush(target: Route) {
    let nav = navigator();
    nav.push(target);
    *CONSOLE.write() = "".to_string();
}


fn new_tab(target: &str) {
    document::eval(&format!(r#"window.open("{}", "_blank");"#, target));
    *CONSOLE.write() = "".to_string();
}


fn update_instructions() -> (&'static str, &'static str, &'static str) {
    let input = CONSOLE();
    let current_route = use_route::<Route>();
    if input.is_empty() && current_route == (Route::Home {}) {
        INFO_STR
    }
    else if let Some(_) = is_page(&input) {
        ("↵", " to open page", "") 
    }
    else if let Some(_) = is_extern(&input) {
        ("↵", " to open external page", "") 
    }
    else {
        ("", "", "")
    }
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
                        console.style.color = "gray";
                        console.style.borderColor = "gray";
                    });
                    console.addEventListener("focus", () => {
                        console.style.color = "white";
                        console.style.borderColor = "white";
                    });
                "#,
            );
        });
    };
}


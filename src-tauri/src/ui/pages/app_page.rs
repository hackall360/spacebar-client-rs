use dioxus::prelude::*;

pub fn AppPage(cx: Scope) -> Element {
    cx.render(rsx! {
        div { "App Page" }
    })
}

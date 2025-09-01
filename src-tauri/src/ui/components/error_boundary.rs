use dioxus::prelude::*;

pub fn ErrorBoundary(cx: Scope) -> Element {
    cx.render(rsx! {
        div { "Error boundary" }
    })
}

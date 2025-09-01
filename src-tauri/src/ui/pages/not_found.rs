use dioxus::prelude::*;

pub fn NotFoundPage(cx: Scope) -> Element {
    cx.render(rsx! { div { "Page Not Found" } })
}

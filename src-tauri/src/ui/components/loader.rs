use dioxus::prelude::*;

pub fn Loader(cx: Scope) -> Element {
    cx.render(rsx! {
        div { "Loading..." }
    })
}

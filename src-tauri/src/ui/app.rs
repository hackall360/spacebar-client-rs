use dioxus::prelude::*;

use super::components::{ErrorBoundary, Loader};
use super::pages::AppPage;

pub fn App(cx: Scope) -> Element {
    cx.render(rsx! {
        ErrorBoundary {}
        Loader {}
        AppPage {}
    })
}

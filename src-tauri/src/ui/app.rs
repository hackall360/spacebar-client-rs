use dioxus::prelude::*;

use super::components::{ErrorBoundary, Loader};
use super::pages::AppPage;
use spacebar_core::controllers::{BannerRenderer, ModalRenderer};

pub fn App(cx: Scope) -> Element {
    cx.render(rsx! {
        ErrorBoundary {}
        BannerRenderer {}
        ModalRenderer {}
        Loader {}
        AppPage {}
    })
}

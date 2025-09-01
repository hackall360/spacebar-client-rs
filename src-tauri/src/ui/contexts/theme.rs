use dioxus::prelude::*;
use crate::ui::hooks::app_store::use_app_store;

/// Apply theme variables based on the global [`AppStore`].
pub fn Theme<'a>(cx: Scope<'a>) -> Element<'a> {
    let _app_store = use_app_store(cx);
    // In a full implementation this component would inject CSS variables.
    cx.render(rsx!({}))
}

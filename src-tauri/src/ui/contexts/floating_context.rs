use dioxus::prelude::*;
use crate::ui::hooks::floating::{use_floating, FloatingState};

#[derive(Props)]
pub struct FloatingProviderProps<'a> {
    #[props(default)]
    pub initial_open: bool,
    children: Element<'a>,
}

pub fn FloatingProvider<'a>(cx: Scope<'a, FloatingProviderProps<'a>>) -> Element<'a> {
    let state = use_floating(cx, cx.props.initial_open);
    use_context_provider(cx, || state.clone());
    cx.render(rsx!( {cx.props.children.clone()} ))
}

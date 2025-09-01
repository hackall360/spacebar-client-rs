use std::rc::Rc;
use dioxus::prelude::*;

/// Minimal representation of the `useFloating` hook.
#[derive(Clone)]
pub struct FloatingState {
    pub open: UseState<bool>,
    pub set_open: Rc<dyn Fn(bool)>,
}

pub fn use_floating(cx: &ScopeState, initial_open: bool) -> FloatingState {
    let open = use_state(cx, move || initial_open);
    let open_clone = open.clone();
    let set_open = Rc::new(move |v: bool| open_clone.set(v));

    FloatingState { open, set_open }
}

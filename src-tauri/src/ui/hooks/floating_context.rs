use dioxus::prelude::*;
use super::floating::FloatingState;

pub fn use_floating_context(cx: &ScopeState) -> FloatingState {
    use_context::<FloatingState>(cx)
        .expect("Floating components must be wrapped in a floating provider")
        .clone()
}

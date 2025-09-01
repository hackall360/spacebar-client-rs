use dioxus::prelude::*;

/// Invoke `callback` when the window is resized.
///
/// Currently this function is a placeholder that wires the callback
/// into a [`use_effect`] so it can be extended with a real listener.
pub fn use_window_resize(cx: &ScopeState, _callback: impl Fn() + 'static, _interval: u64) {
    use_effect(cx, (), |_| async move {});
}

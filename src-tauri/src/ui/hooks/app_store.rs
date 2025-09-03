use dioxus::prelude::*;
use spacebar_core::stores::app_store::AppStore;

/// Access the global [`AppStore`].
pub fn use_app_store(cx: &ScopeState) -> &UseSharedState<AppStore> {
    use_shared_state::<AppStore>(cx).expect("AppStore not provided")
}

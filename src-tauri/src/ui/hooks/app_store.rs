use dioxus::prelude::*;
use crate::stores::app_store::AppStore;

/// Access the global [`AppStore`].
pub fn use_app_store(cx: &ScopeState) -> &UseSharedState<AppStore> {
    cx.use_shared_state::<AppStore>().expect("AppStore not provided")
}

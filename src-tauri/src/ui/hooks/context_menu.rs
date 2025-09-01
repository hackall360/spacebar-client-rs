use std::rc::Rc;
use dioxus::prelude::*;
use crate::ui::contexts::context_menu_context::ContextMenuProps;

#[derive(Clone)]
pub struct ContextMenuController {
    pub is_open: UseState<bool>,
    pub props: UseState<Option<ContextMenuProps>>,
    pub open: Rc<dyn Fn(ContextMenuProps)>,
    pub close: Rc<dyn Fn()>,
}

pub fn use_context_menu(cx: &ScopeState) -> ContextMenuController {
    let is_open = use_state(cx, || false);
    let props = use_state(cx, || None::<ContextMenuProps>);

    let open_state = is_open.clone();
    let props_state = props.clone();
    let open = Rc::new(move |p: ContextMenuProps| {
        props_state.set(Some(p));
        open_state.set(true);
    });

    let close_state = is_open.clone();
    let close = Rc::new(move || {
        close_state.set(false);
    });

    ContextMenuController {
        is_open,
        props,
        open,
        close,
    }
}

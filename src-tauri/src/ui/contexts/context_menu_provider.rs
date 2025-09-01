use std::rc::Rc;
use dioxus::prelude::*;
use super::context_menu_context::ContextMenuContext;
use crate::ui::hooks::context_menu::use_context_menu;

#[derive(Props)]
pub struct ContextMenuProviderProps<'a> {
    children: Element<'a>,
}

pub fn ContextMenuProvider<'a>(cx: Scope<'a, ContextMenuProviderProps<'a>>) -> Element<'a> {
    let controller = use_context_menu(cx);

    let open = controller.open.clone();
    let close = controller.close.clone();
    let set_reference_element = Rc::new(|| {});
    let on_context_menu = Rc::new(|_p: super::context_menu_context::ContextMenuProps| {});

    use_context_provider(cx, || ContextMenuContext {
        open,
        close,
        set_reference_element,
        on_context_menu,
    });

    cx.render(rsx!( {cx.props.children.clone()} ))
}

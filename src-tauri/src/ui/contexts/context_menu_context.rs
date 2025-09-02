use std::rc::Rc;
use dioxus::prelude::*;
use crate::stores::objects::{
    channel::Channel,
    guild::Guild,
    guild_member::GuildMember,
    message::Message,
    user::User,
};

#[derive(Clone)]
pub enum ContextMenuProps {
    User { user: User, member: Option<GuildMember> },
    Message { message: Message },
    Channel { channel: Channel },
    ChannelMention { channel: Channel },
    Guild { guild: Guild },
}

#[derive(Clone)]
pub struct ContextMenuContext {
    pub open: Rc<dyn Fn(ContextMenuProps)>,
    pub close: Rc<dyn Fn()>,
    pub set_reference_element: Rc<dyn Fn()>,
    pub on_context_menu: Rc<dyn Fn(ContextMenuProps)>,
}

pub fn use_context_menu_context(cx: &ScopeState) -> ContextMenuContext {
    use_context::<ContextMenuContext>(cx)
        .expect("ContextMenuContext not provided")
        .clone()
}

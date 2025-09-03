use super::store::Store;
use serde::{Deserialize, Serialize};
use std::default::Default;

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct MessageQueueState {
    // TODO: replicate fields from TypeScript MessageQueue.ts
}

#[derive(Debug, Clone)]
pub enum MessageQueueEvent {
    Updated,
}

pub type MessageQueue = Store<MessageQueueState, MessageQueueEvent>;

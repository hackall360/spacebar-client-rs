pub mod store;

// Individual store modules generated from the TypeScript MobX stores.
// Each store exposes a `State` struct, an `Event` enum and a type alias
// `<Name>Store` which wraps the generic [`Store`] type.

pub mod account_store;
pub mod app_store;
pub mod channel_store;
pub mod emoji_store;
pub mod experiments_store;
pub mod gateway_connection_store;
pub mod guild_member_list_store;
pub mod guild_member_store;
pub mod guild_store;
pub mod message_queue;
pub mod message_store;
pub mod presence_store;
pub mod private_channel_store;
pub mod read_state_store;
pub mod role_store;
pub mod theme_store;
pub mod updater_store;
pub mod user_store;

pub mod objects;

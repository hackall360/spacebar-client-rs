use std::sync::Arc;
use tokio::sync::{broadcast, Mutex};

/// Generic observable store backed by an [`Arc<Mutex<_>>`] state and a broadcast channel.
///
/// Each store holds its state inside a [`Mutex`] so it can be shared across
/// asynchronous tasks.  When the state is mutated an event can be sent through
/// the broadcast channel allowing other parts of the application to react to
/// the update.
pub struct Store<S, E> {
    pub state: Arc<Mutex<S>>,           // shared mutable state
    pub tx: broadcast::Sender<E>,       // event broadcaster
}

impl<S: Default, E: Clone> Store<S, E> {
    /// Creates a new store and returns it along with a receiver for events.
    pub fn new() -> (Self, broadcast::Receiver<E>) {
        let (tx, rx) = broadcast::channel(32);
        (
            Self {
                state: Arc::new(Mutex::new(S::default())),
                tx,
            },
            rx,
        )
    }
}

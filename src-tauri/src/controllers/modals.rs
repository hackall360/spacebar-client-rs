use std::sync::{atomic::{AtomicUsize, Ordering}, Mutex, OnceLock};
use tokio::sync::watch;
use dioxus::prelude::*;

#[derive(Clone)]
pub struct Modal {
    pub key: usize,
    pub modal_type: String,
}

pub struct ModalController {
    stack: Mutex<Vec<Modal>>,
    tx: watch::Sender<Vec<Modal>>,
    counter: AtomicUsize,
}

impl ModalController {
    fn new() -> Self {
        let (tx, _rx) = watch::channel(Vec::new());
        Self { stack: Mutex::new(Vec::new()), tx, counter: AtomicUsize::new(0) }
    }

    pub fn subscribe(&self) -> watch::Receiver<Vec<Modal>> {
        self.tx.subscribe()
    }

    fn notify(&self) {
        let stack = self.stack.lock().unwrap().clone();
        let _ = self.tx.send(stack);
    }

    pub fn push(&self, modal_type: impl Into<String>) {
        let key = self.counter.fetch_add(1, Ordering::SeqCst) + 1;
        let mut stack = self.stack.lock().unwrap();
        stack.push(Modal { key, modal_type: modal_type.into() });
        drop(stack);
        self.notify();
    }

    pub fn pop(&self) {
        let mut stack = self.stack.lock().unwrap();
        stack.pop();
        drop(stack);
        self.notify();
    }

    pub fn remove(&self, key: usize) {
        let mut stack = self.stack.lock().unwrap();
        stack.retain(|m| m.key != key);
        drop(stack);
        self.notify();
    }

    pub fn clear(&self) {
        self.stack.lock().unwrap().clear();
        self.notify();
    }
}

static MODAL_CONTROLLER: OnceLock<ModalController> = OnceLock::new();

pub fn modal_controller() -> &'static ModalController {
    MODAL_CONTROLLER.get_or_init(|| ModalController::new())
}

pub fn ModalRenderer(cx: Scope) -> Element {
    let modals = use_state(cx, || Vec::<Modal>::new());
    use_future(cx, (), |_| {
        to_owned![modals];
        async move {
            let mut rx = modal_controller().subscribe();
            loop {
                if rx.changed().await.is_ok() {
                    modals.set((*rx.borrow()).clone());
                } else {
                    break;
                }
            }
        }
    });

    cx.render(rsx! {
        div {
            class: "modal-container",
            for modal in modals.get().iter() {
                div { key: "{modal.key}", class: "modal", {modal.modal_type.clone()} }
            }
        }
    })
}

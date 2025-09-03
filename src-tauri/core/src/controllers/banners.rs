use dioxus::prelude::*;
use std::sync::{
    atomic::{AtomicUsize, Ordering},
    Mutex, OnceLock,
};
use tokio::sync::watch;

#[derive(Clone)]
pub enum BannerType {
    Offline,
}

#[derive(Clone)]
pub struct Banner {
    pub key: usize,
    pub banner_type: BannerType,
}

pub struct BannerController {
    stack: Mutex<Vec<Banner>>,
    tx: watch::Sender<Vec<Banner>>,
    counter: AtomicUsize,
}

impl BannerController {
    fn new() -> Self {
        let (tx, _rx) = watch::channel(Vec::new());
        Self {
            stack: Mutex::new(Vec::new()),
            tx,
            counter: AtomicUsize::new(0),
        }
    }

    pub fn subscribe(&self) -> watch::Receiver<Vec<Banner>> {
        self.tx.subscribe()
    }

    fn notify(&self) {
        let stack = self.stack.lock().unwrap().clone();
        let _ = self.tx.send(stack);
    }

    pub fn push(&self, banner_type: BannerType) {
        let key = self.counter.fetch_add(1, Ordering::SeqCst) + 1;
        let mut stack = self.stack.lock().unwrap();
        stack.push(Banner { key, banner_type });
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
        stack.retain(|b| b.key != key);
        drop(stack);
        self.notify();
    }

    pub fn clear(&self) {
        self.stack.lock().unwrap().clear();
        self.notify();
    }
}

static BANNER_CONTROLLER: OnceLock<BannerController> = OnceLock::new();

pub fn banner_controller() -> &'static BannerController {
    BANNER_CONTROLLER.get_or_init(|| BannerController::new())
}

pub fn BannerRenderer(cx: Scope) -> Element {
    let banners = use_state(cx, || Vec::<Banner>::new());
    use_future(cx, (), |_| {
        to_owned![banners];
        async move {
            let mut rx = banner_controller().subscribe();
            loop {
                if rx.changed().await.is_ok() {
                    banners.set((*rx.borrow()).clone());
                } else {
                    break;
                }
            }
        }
    });

    cx.render(rsx! {
        div {
            class: "banner-container",
            for banner in banners.get().iter() {
                match banner.banner_type {
                    BannerType::Offline => rsx!(div { key: "{banner.key}", "Offline" }),
                }
            }
        }
    })
}

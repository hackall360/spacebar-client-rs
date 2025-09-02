use std::rc::Rc;
use dioxus::prelude::*;
use super::logger::use_logger;

pub struct InstanceValidation {
    pub handle_instance_change: Rc<dyn Fn(String)>,
    pub is_checking_instance: UseState<bool>,
}

pub fn use_instance_validation(cx: &ScopeState) -> InstanceValidation {
    let logger = use_logger("InstanceValidation");
    let is_checking_instance = use_state(cx, || false);
    let checking = is_checking_instance.clone();

    let handle_instance_change = Rc::new(move |value: String| {
        checking.set(true);
        logger.debug(&format!("checking instance: {}", value));
        // placeholder for network lookup
        checking.set(false);
    });

    InstanceValidation {
        handle_instance_change,
        is_checking_instance: is_checking_instance.clone(),
    }
}

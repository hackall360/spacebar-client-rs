use log::{debug, error, info, trace, warn};

/// Simple logger wrapper replicating the TypeScript `useLogger` hook.
pub struct Logger {
    name: String,
}

impl Logger {
    pub fn debug(&self, message: &str) {
        debug!(target: &self.name, "{}", message);
    }
    pub fn info(&self, message: &str) {
        info!(target: &self.name, "{}", message);
    }
    pub fn warn(&self, message: &str) {
        warn!(target: &self.name, "{}", message);
    }
    pub fn error(&self, message: &str) {
        error!(target: &self.name, "{}", message);
    }
    pub fn trace(&self, message: &str) {
        trace!(target: &self.name, "{}", message);
    }
}

pub fn use_logger(name: &str) -> Logger {
    Logger { name: name.to_string() }
}

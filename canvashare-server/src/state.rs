use std::sync::{atomic::AtomicUsize, Arc};

#[derive(Clone)]
pub struct AppState {
    pub visitor_count: Arc<AtomicUsize>,
}
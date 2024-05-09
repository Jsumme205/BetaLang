use std::{collections::VecDeque, sync::{atomic::AtomicUsize, Arc, RwLock}};

struct RawShared<T, U> {
    queue_one: Arc<RwLock<VecDeque<T>>>,
    queue_two: Arc<RwLock<VecDeque<U>>>
}

enum 
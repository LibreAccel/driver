pub mod fast_mutex;
pub mod push_lock;

pub use self::{fast_mutex::FastMutex as Mutex, push_lock::PushLock as RwLock};

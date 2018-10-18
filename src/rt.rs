//! Primitives for spawning asynchronous tasks

#[doc(no_inline)]
pub use tokio::executor::{spawn, DefaultExecutor, Executor, Spawn, SpawnError};

#[doc(no_inline)]
pub use tokio::runtime::run;

#[doc(no_inline)]
pub use tokio_threadpool::{blocking, BlockingError};

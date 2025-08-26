#[cfg(feature = "server")]
pub mod postgresql;

#[cfg(feature = "server")]
pub use postgresql::get_pg_pool;

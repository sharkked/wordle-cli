pub mod cache;
pub mod share;
pub use cache::Cache;

pub fn home_dir() -> String {
    std::env::var_os("HOME").unwrap().into_string().unwrap()
}

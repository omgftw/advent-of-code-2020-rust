use eyre::{eyre, Result};
use std::fmt::Display;

pub fn is_debug() -> bool {
    std::env::var("RUST_LOG").unwrap_or_default() == "debug"
}

pub fn print_debug<T: Display>(s: T) {
    if is_debug() {
        println!("{}", s);
    }
}

pub(crate) trait OptionExt<T> {
    fn to_result(self) -> Result<T>;
}

impl<T> OptionExt<T> for Option<T> {
    fn to_result(self) -> Result<T> {
        self.ok_or_else(|| eyre!("Option was None"))
    }
}

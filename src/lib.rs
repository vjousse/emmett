pub mod codeblock;
pub mod config;
pub mod content;
pub mod errors;
pub mod startup;

// So we can use bail! in all other crates
#[macro_export]
macro_rules! bail {
    ($e:expr) => {
        return Err($e.into());
    };
    ($fmt:expr, $($arg:tt)+) => {
        return Err(format!($fmt, $($arg)+).into())
    };
}

use log::{debug, error};

trait Loggable<T> {
    fn make_log<F>(self, ctx: &str, on_error: F) -> T
    where
        F: FnOnce() -> T;
}

impl<T: std::fmt::Debug, E: std::fmt::Display> Loggable<T> for Result<T, E> {
    fn make_log<F>(self, ctx: &str, on_error: F) -> T
    where
        F: FnOnce() -> T,
    {
        match self {
            Ok(val) => {
                debug!("[{}] Ok: {:?}", ctx, val);
                val
            }
            Err(e) => {
                error!(" [{}] Error: {}", ctx, e);
                on_error()
            }
        }
    }
}

impl<T: std::fmt::Debug> Loggable<T> for Option<T> {
    fn make_log<F>(self, ctx: &str, on_error: F) -> T
    where
        F: FnOnce() -> T,
    {
        match self {
            Some(val) => {
                debug!(" [{}] Some: {:?}", ctx, val);
                val
            }
            None => {
                debug!("[{}] Got None", ctx);
                on_error()
            }
        }
    }
}

#[macro_export]
macro_rules! make_log {
    ($expr:expr, $ctx:expr) => {{
        let __temp = $expr;
        Loggable::make_log(__temp, $ctx, || {
            return;
        })
    }};
}

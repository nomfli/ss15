use log::{debug, error};

pub trait Loggable<T> {
    fn make_log(self, ctx: &str) -> Option<T>;
}

impl<T: std::fmt::Debug, E: std::fmt::Display> Loggable<T> for Result<T, E> {
    fn make_log(self, ctx: &str) -> Option<T> {
        match self {
            Ok(val) => {
                debug!("[{}] Ok: {:?}", ctx, val);
                Some(val)
            }
            Err(e) => {
                error!(" [{}] Error: {}", ctx, e);
                None
            }
        }
    }
}

impl<T: std::fmt::Debug> Loggable<T> for Option<T> {
    fn make_log(self, ctx: &str) -> Option<T> {
        match self {
            Some(val) => {
                debug!(" [{}] Some: {:?}", ctx, val);
                Some(val)
            }
            None => {
                debug!("[{}] Got None", ctx);
                None
            }
        }
    }
}

#[macro_export]
macro_rules! make_log {
    ($expr:expr, $ctx:expr) => {{
        let __temp = $expr;
        Loggable::make_log(__temp, $ctx)
    }};
}

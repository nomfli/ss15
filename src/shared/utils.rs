use bevy::log::{debug, error};

pub trait Loggable<T> {
    fn make_log(self, ctx: &str) -> Option<T>;
}

impl<T: std::fmt::Debug, E: std::fmt::Debug> Loggable<T> for Result<T, E> {
    fn make_log(self, ctx: &str) -> Option<T> {
        match self {
            Ok(val) => {
                debug!(
                    "[{:?}] Ok: {:?} in line: {:?}, in file {:?}",
                    ctx,
                    val,
                    line!(),
                    file!(),
                );
                Some(val)
            }
            Err(e) => {
                error!(
                    " [{:?}] Error: {:?}, in line: {:?}, in file {:?} ",
                    ctx,
                    e,
                    line!(),
                    file!()
                );
                None
            }
        }
    }
}

impl<T: std::fmt::Debug> Loggable<T> for Option<T> {
    fn make_log(self, ctx: &str) -> Option<T> {
        match self {
            Some(val) => {
                debug!(
                    " [{:?}] Some: {:?}, in line: {:?}, in file {:?}",
                    ctx,
                    val,
                    line!(),
                    file!()
                );
                Some(val)
            }
            None => {
                debug!(
                    "[{:?}] Got None in line: {:?}, in file {:?}",
                    ctx,
                    line!(),
                    file!()
                );
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

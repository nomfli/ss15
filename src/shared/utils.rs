#[macro_export]
macro_rules! make_log {
    ($expr:expr, $ctx:expr) => {
        match $expr {
            Ok(val) => {
                debug!("[{}] Value: {:?} (at {}:{})", $ctx, val, file!(), line!());
                val
            }
            Err(e) => {
                error!("[{}] Error: {} (at {}:{})", $ctx, e, file!(), line!());
                return;
            }
        }
    };
    ($expr:expr, $ctx:expr, opt) => {
        match $expr {
            Some(val) => {
                debug!("[{}] Some: {:?} (at {}:{})", $ctx, val, file!(), line!());
                val
            }
            None => {
                debug!("[{}] Got None (at {}:{})", $ctx, file!(), line!());
                return;
            }
        }
    };
}

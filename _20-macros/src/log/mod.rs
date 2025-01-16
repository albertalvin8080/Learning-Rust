#[macro_export]
macro_rules! log {
    ($ty:expr, $msg:expr) => {
        match $ty {
            "info" => println!("[INFO] {}", $msg),
            "warn" => println!("[WARN] {}", $msg),
            "err" => eprintln!("[ERR] {}", $msg),
            _ => println!("[UNKNOWN]: {}", $msg),
        }
    };
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_log() {
        log!("info", "Testing log INFO");
        log!("warn", "Testing log WARN");
        log!("err", "Testing log ERR");
        log!("___", "Testing log UNKNOWN");
    }
}
use std::sync::Mutex;
use chrono;

static INIT_TIME: Mutex<String> = Mutex::new(String::new());

pub fn init() {
    *INIT_TIME.lock().unwrap() = chrono::Utc::now().to_rfc3339();
}

pub fn get_data() -> &'static Mutex<String> {
    &INIT_TIME
}


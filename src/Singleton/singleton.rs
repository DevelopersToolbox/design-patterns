use std::sync::{Arc, Mutex};
use std::sync::Once;

struct Singleton {
    // Add fields here
}

impl Singleton {
    fn new() -> Self {
        Singleton {
            // Initialize fields here
        }
    }
}

static mut SINGLETON: Option<Arc<Mutex<Singleton>>> = None;
static ONCE: Once = Once::new();

fn singleton_instance() -> Arc<Mutex<Singleton>> {
    unsafe {
        ONCE.call_once(|| {
            let singleton = Singleton::new();
            SINGLETON = Some(Arc::new(Mutex::new(singleton)));
        });
        SINGLETON.clone().unwrap()
    }
}

fn main() {
    let singleton1 = singleton_instance();
    let singleton2 = singleton_instance();

    println!("{}", Arc::ptr_eq(&singleton1, &singleton2));  // Output: true
}

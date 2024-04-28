pub mod libs;
use ::std::time::Duration;
use std::thread::sleep;

fn main() {
    let mut limiter = libs::limiter::Limiter::new(3, 1);

    for i in 0..100 {
        if limiter.acquire(1) {
            println!("Request {}: Allowed", i + 1);
        } else {
            println!("Request {}: Denied", i + 1);
        }
        sleep(Duration::from_millis(200));
    }
}

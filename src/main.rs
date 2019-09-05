use std::time::Duration;
use std::thread;

fn main() {
    loop {
        println!("Hello, world!");
        thread::sleep(Duration::from_millis(1000))
    }
}

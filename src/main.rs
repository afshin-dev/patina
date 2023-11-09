use std::{thread, time};

fn main() {
    // closure example of `wait_a_sec`
    let wait_a_second = |n: u64| -> () {
        for i in 0..n {
            thread::sleep(time::Duration::from_secs(1));
            println!("second {i}")
        }
    };
    wait_a_sec(3);
    wait_a_second(3);
}

fn wait_a_sec(second: u64) -> () {
    for i in 0..second {
        thread::sleep(time::Duration::from_secs(1));
        println!("sec {i}");
    }
}

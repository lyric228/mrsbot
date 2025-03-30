pub fn deadlock_detection_thread() -> () {
    use parking_lot::deadlock;
    use sysx::time::sleep;
    use std::thread;

    thread::spawn(move || {
        loop {
            sleep("10s");
            let deadlocks = deadlock::check_deadlock();

            if deadlocks.is_empty() {
                continue;
            }

            println!("{} deadlocks detected", deadlocks.len());
            for (i, threads) in deadlocks.iter().enumerate() {
                println!("Deadlock #{}", i);
                for t in threads {
                    println!("Thread Id {:#?}", t.thread_id());
                    println!("{:#?}", t.backtrace());
                }
            }
        }
    });
}

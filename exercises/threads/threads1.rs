// threads1.rs
// Make this compile! Execute `rustlings hint threads1` for hints :)
// The idea is the thread spawned on line 22 is completing jobs while the main thread is
// monitoring progress until 10 jobs are completed. Because of the difference between the
// spawned threads' sleep time, and the waiting threads sleep time, when you see 6 lines
// of "waiting..." and the program ends without timing out when running,
// you've got it :)

use std::sync::Arc;
use std::thread;
use std::time::Duration;
use std::sync::Mutex;

struct JobStatus {
    jobs_completed: Mutex<u32>,
}

fn main() {
    let status = Arc::new(JobStatus { jobs_completed: Mutex::new(0) });
    let status_shared = status.clone();
    thread::spawn(move || {
        for _ in 0..10 {
            thread::sleep(Duration::from_millis(250));
            {
                let mut job_mutex = status_shared.jobs_completed.lock().unwrap();
                *job_mutex += 1;
            }
        }
    });

    while {
        let mut job_mutex = status.jobs_completed.lock().unwrap();
        *job_mutex < 10 
    }
    {
        println!("waiting... ");
        thread::sleep(Duration::from_millis(500));
    }
}

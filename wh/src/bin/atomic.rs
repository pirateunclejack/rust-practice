use std::{sync::atomic::AtomicUsize, thread, time::Instant};

static COUNTER: AtomicUsize = AtomicUsize::new(0);

fn main() {
    let start = Instant::now();

    let mut handles = Vec::new();

    for _ in 0..1000 {
        let h = thread::spawn(|| {
            for _ in 0..1000 {
                COUNTER.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
            }
        });
        handles.push(h);
    }

    handles.into_iter().for_each(|h| h.join().unwrap());

    println!(
        "Total: {}",
        COUNTER.load(std::sync::atomic::Ordering::Relaxed)
    );

    let elapsed = start.elapsed();
    println!("Elapsed time: {}", elapsed.as_micros());
}

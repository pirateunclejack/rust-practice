use std::sync::atomic::AtomicUsize;
use std::thread;
use std::time::Instant;

use rand::Rng;

fn main() {
    let start = Instant::now();

    // products, 1, 2, 3, 4, 5
    let section_count = rand::rng().random_range(10..=20);

    let mut sections = Vec::new();

    let mut actual = [0; 5];

    for _ in 0..section_count {
        let mut section = Section([0; 5]);

        for (i, p) in section.0.iter_mut().enumerate() {
            *p = rand::rng().random_range(0..=1_000_000);
            actual[i] += *p
        }
        sections.push(section);
    }

    println!("Actual: {actual:#?}");

    let counted: [AtomicUsize; 5] = Default::default();

    thread::scope(|s| {
        for sec in sections.iter() {
            s.spawn(|| {
                for (i, c) in sec.0.iter().enumerate() {
                    for _ in 0..*c {
                        counted[i].fetch_add(1, std::sync::atomic::Ordering::Relaxed);
                    }
                }
            });
        }
    });

    println!("Counted: {counted:#?}");

    for i in 0..5 {
        assert_eq!(
            actual[i],
            counted[i].load(std::sync::atomic::Ordering::Relaxed)
        )
    }

    let elapsed = start.elapsed();
    println!("Elapsed time: {}", elapsed.as_micros());
}

struct Section([usize; 5]);

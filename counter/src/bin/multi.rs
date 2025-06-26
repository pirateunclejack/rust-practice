use std::{collections::HashMap, error::Error, fs, thread, time::Instant};

fn main() -> Result<(), Box<dyn Error>> {
    let start = Instant::now();

    let path = "./files";
    let mut map: HashMap<String, usize> = HashMap::new();
    let files: Vec<_> = fs::read_dir(path)?
        .filter_map(|entry| {
            let entry = entry.ok()?;
            let path = entry.path();
            // let is_txt = path.extension().map_or(false, |s| s == "txt");
            let is_txt = path.extension().and_then(|s| s.to_str()) == Some("txt");
            if path.is_file() && is_txt {
                Some(path)
            } else {
                None
            }
        })
        .collect();
    // .for_each(|p| {
    //     if let Ok(text) = fs::read_to_string(p) {
    //         text.split_whitespace().for_each(|w| {
    //             let word = w
    //                 .trim_matches(|c: char| c.is_ascii_punctuation())
    //                 .to_lowercase();
    //             if !word.is_empty() {
    //                 *map.entry(word).or_insert(0) += 1;
    //             }
    //         });
    //     }
    // });

    const CHUNK_SIZE: usize = 5;
    let chunks = files.chunks(CHUNK_SIZE);

    // Scoped threads 带作用域的线程
    thread::scope(|s| {
        let mut handles = vec![];

        for chunk in chunks {
            let mut local_map: HashMap<String, usize> = HashMap::new();
            // let chunk = chunk.to_vec();
            let handle = s.spawn(move || {
                chunk
                    .iter()
                    .filter_map(|p| fs::read_to_string(p).ok())
                    .for_each(|text| {
                        text.split_whitespace().for_each(|w| {
                            let word = w
                                .trim_matches(|c: char| c.is_ascii_punctuation())
                                .to_lowercase();
                            if !word.is_empty() {
                                *local_map.entry(word).or_insert(0) += 1;
                            }
                        });
                    });
                local_map
            });
            handles.push(handle);
        }

        for h in handles {
            let local_map = h.join().unwrap();
            for (k, v) in local_map {
                *map.entry(k).or_insert(0) += v;
            }
        }
    });

    println!("Map count: {}", map.len());

    let mut vec: Vec<_> = map.iter().collect();

    vec.sort_by(|a, b| b.1.cmp(a.1));

    for i in 0..10 {
        println!("{}: {}", vec.get(i).unwrap().0, vec.get(i).unwrap().1);
    }

    let elapsed = start.elapsed();
    println!("Time elapsed: {}", elapsed.as_millis());

    Ok(())
}

/*
- https://stackoverflow.com/questions/50282619/is-it-possible-to-share-a-hashmap-between-threads-without-locking-the-entire-has
- https://nickymeuleman.netlify.app/blog/multithreading-rust
- https://www.jb51.net/program/317685aja.htm

*/

use std::{
    collections::HashMap,
    sync::{Arc, RwLock},
    thread,
    time::Duration,
};

fn main() {
    let data: Arc<RwLock<HashMap<u8, i32>>> =
        Arc::new(RwLock::new(vec![(4, 9), (5, 8)].into_iter().collect()));

    let threads: Vec<_> = (0..10)
        .map(|i| {
            let data = Arc::clone(&data);
            thread::spawn(move || worker_thread(i, data))
        })
        .collect();

    for t in threads {
        t.join().expect("Thread panicked");
    }

    println!("{:?}", data);
}

fn worker_thread(id: u8, data: Arc<RwLock<HashMap<u8, i32>>>) {
    loop {
        {
            let mut map = data.write().expect("RwLock poisoned");
            if let Some(_) = map.get(&id) {
                map.entry(id).and_modify(|v| *v += 100);
                thread::sleep(Duration::from_millis(50));
                return;
            }
        }

        // drop(map);

        {
            let mut map = data.write().expect("RwLock poisoned");
            thread::sleep(Duration::from_millis(50));
            map.entry(id).or_insert(0);
        }
    }
}

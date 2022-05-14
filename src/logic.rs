use rayon::prelude::*;
use rustc_hash::FxHashMap;
use std::{
    collections::HashMap,
    sync::{mpsc, Arc},
    thread,
};

pub fn count_letters_hash_map_fold(text: &str) -> HashMap<char, u64> {
    text.chars().fold(HashMap::new(), |mut acc, curr| {
        let count = acc.entry(curr).or_insert(0);
        *count += 1;
        acc
    })
}

pub fn count_letters_hash_map_forloop(text: &str) -> HashMap<char, u64> {
    let mut map = HashMap::new();

    for c in text.chars() {
        let count = map.entry(c).or_insert(0);
        *count += 1;
    }

    map
}

pub fn count_letters_fx_hash_map_fold(text: &str) -> FxHashMap<char, u64> {
    text.chars().fold(FxHashMap::default(), |mut acc, curr| {
        let count = acc.entry(curr).or_insert(0);
        *count += 1;
        acc
    })
}

pub fn count_letters_fx_hash_map_forloop(text: &str) -> FxHashMap<char, u64> {
    let mut map = FxHashMap::default();

    for c in text.chars() {
        let count = map.entry(c).or_insert(0);
        *count += 1;
    }

    map
}

pub fn count_letters_parallel(text: &'static str, thread_count: u8) -> FxHashMap<char, u64> {
    let thread_count = usize::from(thread_count);
    let text = Arc::new(text);
    let mut join_handles = Vec::with_capacity(thread_count);

    let (tx, rx) = mpsc::channel();

    let slice_size = text.len() / thread_count;

    for i in 0..thread_count {
        let start = i * slice_size;
        let end = start + slice_size;

        let c_text = Arc::clone(&text);
        let thread_tx = tx.clone();

        join_handles.push(thread::spawn(move || {
            let thread_map = count_letters_fx_hash_map_forloop(&c_text[start..end]);
            thread_tx.send(thread_map).unwrap();
        }));
    }

    drop(tx);

    for jh in join_handles {
        let _ = jh.join();
    }

    let mut uber_map = FxHashMap::<char, u64>::default();

    for thread_map in rx {
        for key in thread_map.keys() {
            let uber_value = uber_map.entry(*key).or_insert(0);
            *uber_value += thread_map.get(key).unwrap();
        }
    }

    uber_map
}

pub fn count_letters_with_rayon(text: &'static str) -> FxHashMap<char, u64> {
    text.par_chars()
        .fold(FxHashMap::default, |mut acc, curr| {
            let count = acc.entry(curr).or_insert(0);
            *count += 1;
            acc
        })
        .reduce(FxHashMap::default, |mut m1, m2| {
            for k in m2.keys() {
                let value = m1.entry(*k).or_insert(0);
                *value += 1;
            }
            m1
        })
}

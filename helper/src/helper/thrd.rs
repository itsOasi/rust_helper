use std::{thread::{self, JoinHandle}, sync::{Arc, Mutex}};

const POOL: Vec<JoinHandle<Vec<usize>>> = vec![];
pub fn new_usize_vec(func: fn()->Vec<usize>)->Vec<usize>{
    let handle = thread::spawn(move || {
            let result = func();
            result
        });
    handle.join().unwrap()
}
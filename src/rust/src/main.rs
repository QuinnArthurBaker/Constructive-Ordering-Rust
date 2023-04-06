use num::BigUint;
use std::sync::mpsc;
use std::thread;
use cof::{factorial, validate_perm, kth_permutation};

struct ThreadResult {
    id: usize,
    sum: BigUint,
}

pub fn main() {
    
    //program constants
    let n: usize = 12;
    let num_threads: usize = num_cpus::get();

    let partition_size: BigUint = BigUint::from(n-1);
    let partition_size = (factorial(partition_size))/num_threads; 
    // Create shared mutable state for the results of each thread
    //let thread_result1 = Arc::new(Mutex::new(ThreadResult { sum: 0 }));
    //let thread_result2 = Arc::new(Mutex::new(ThreadResult { sum: 0 }));
    let mut threads = Vec::new();
    let (tx,rx) = mpsc::channel();
    for threadid in 0..num_threads{
        let this_tx = tx.clone();
        let mut index: BigUint = &partition_size*threadid;
        let end_index = &partition_size*(threadid+1);
        let thandle = thread::spawn(move || {
            coz::thread_init();
            let mut ord_count: BigUint = BigUint::from(0 as usize);
            while index < end_index{
                let p = kth_permutation(n, index.clone());
                ord_count += validate_perm(p);
                index = index+(1 as usize);
            }
            let thread_result = ThreadResult{id: threadid, sum: ord_count};
            this_tx.send(thread_result).unwrap();
        });
        threads.push(thandle);
    }

    for handle in threads{
        handle.join().unwrap();
    }
    
    let mut recv = rx.try_recv();
    let mut result_sum = BigUint::from(0 as usize);
    while recv.is_ok() {
        let tr = recv.unwrap();
        println!("Got {} constructive orderings from thread {}", tr.sum, tr.id);
        result_sum = result_sum + tr.sum;
        recv = rx.try_recv();
    }

    println!("Total constructive orderings: {}", result_sum);
}

/* 
fn get_starting_perms(n: usize, num_perms: usize) -> Vec<Vec<usize>>{
    let mut perms: Vec<Vec<usize>> = Vec::new();
    let offset: u128 = ((n-1).factorial()/num_perms) as u128;

    for i in 0..num_perms{
        perms.push(kth_permutation(n, (offset*i as u128)));
    }
    perms

}
*/




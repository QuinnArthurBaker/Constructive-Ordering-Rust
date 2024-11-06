//use cof::tree_search;
use num::{BigUint, FromPrimitive};
use std::collections::VecDeque;
use std::thread;
use std::sync::mpsc;
use cof_tree::N;


pub fn one() -> BigUint {
    BigUint::from_i32(1).unwrap()
}
//recursive tree-search function to find valid constructive orderings. Once a partial ordering has been determined to be non-constructive, the recursion unwinds and all "descendant" orderings of that partial ordering are never considered
//rem: double-ended queue of remaining values that the partial ordering could have. mutable to remove values for future recursive calls. Uses a queue over an array for some slight list-modification optimizations
//daggers: the collections of partial sums of the known values of this ordering. mutable to add a new partial sum per recursive iteration. Represented as an ordered list of boolean flags.
//current_sum: the previous partial sum, since g_i dagger = g_{i-1} dagger + g_i. Avoids unnecessary recomputation of old partial sums. mutable to be updated with the current "newest" permutation value's partial sum
//total: the running total of valid orderings. Incremented when rem is empty.
//This function returns no values as the "total" parameter is used as a reference parameter

pub fn tree_search(rem: &mut VecDeque<usize>, daggers: &mut Vec<bool>, current_sum: &mut usize, total: &mut BigUint){
    //Base Case 1: If the permutation reaches one of the known failstates with unique partial sums, abort and return to unwind the recursion. 
    if *current_sum == 0 || (rem.len()>1 && *current_sum == N/2) {
        //ordering failed
        return;
    }
    //Base Case 2: if the queue of possible next permutation values is empty, we've constructed the full ordering without encountering an error state. This is a constructive ordering so increment the total by 1 and return to unwind the recursion
    if rem.is_empty() {
        *total += one();
        return;
        //ordering passed, add one to the total number of orderings. 
    }

    //recursive case: (could this for loop be threaded? would that create too many threads?)
    for _ in 0..rem.len() {//iterate through each value in rem.
        //remove the first item from the remaning values queue and save in a variable
        let popped = rem.pop_front().unwrap();
        //popped is then used as the next value in the partial ordering that has been defined thus far in previous recursive calls. This partial ordering now is (0, x, y, z, ..., popped). 
        //If this partial ordering is still possibly valid (hasn't entered an error state), recurse down to the next potential ordering value. 
        //If this partial ordering becomes invalid with popped, the state of the partial ordering and other data is restored to before popped was removed from rem, essentially undoing this iteration of the for loop

        let save_sum: usize = *current_sum;
        *current_sum = (*current_sum + popped)%N; 
        //check the other fail state: if the updated current_sum has been seen before, skip over this recursive call, avoiding checking any descendents of this partial ordering

        if !daggers[*current_sum]{
            //flag this index in the daggers array to true
            daggers[*current_sum] = true;
            // start a recursive call using the updated rem/daggers/current_sum values after popping off "popped" and processing it above
            tree_search(rem, daggers, current_sum, total);
            // unflag this current sum as part of the restoration process to revert the state of the system to before "popped" was used as the next value in this partial ordering.
			daggers[*current_sum] = false;
        }     
        //restore current_sum 
        *current_sum = save_sum;
        //add popped to the back of rem. This way it won't be popped again in this loop, since popped is set by always getting the frontmost value. 
        rem.push_back(popped); 
    }
}


fn main() {
    println!("Hello, world!");
    //create the initial states for the state variables used in the recursive algorithm
    let mut rem: VecDeque<usize> = VecDeque::new();
    let mut daggers: Vec<bool> = vec![false; N];
	daggers[0] = true;
    for i in 1..N{
        rem.push_back(i);
    }

    //create the threads object
    let mut threads = Vec::new();
    let (tx,rx) = mpsc::channel();

    for i in 1..N/2{//for the first half of values 1..N 
        //more threading setup
		let this_tx = tx.clone();
        //copy the initial state variables for each thread 
		let mut this_daggers = daggers.clone();
		let mut this_rem = rem.clone();
        //spawn a thread
		let thandle = thread::spawn(move || {
            //initialize the state variables for this thread.
			this_daggers[i] = true; 
			this_rem.retain(|&x| x != i);
			let mut s = i;
			let mut total = BigUint::from_usize(0).unwrap();
			tree_search(&mut this_rem, &mut this_daggers, &mut s, &mut total);
            //send the calculated total for this thread on the mpsc channel
			this_tx.send(total).unwrap();
		});
        //add all threads to the threads vector 
		threads.push(thandle);
    }
    //wait for all threads to finish
	for handle in threads {
		handle.join().unwrap();
	}	

	let mut total = BigUint::from_usize(0).unwrap();
    //receive all values from the threads and add the results together
	let mut recv = rx.try_recv();
	while recv.is_ok() {
		let result = recv.unwrap();
		total += result;
		recv = rx.try_recv();	
	}
    //the threads calculate the number of constructive orderings for permutations from (0 1 ...) to (0 N/2-1 ...). 
    //All these constructive orderings have a 1-1 mapping to all constructive orderings in the range (0 N/2+1 ...) to (0 N-1 ...), 
    //so we multiply by 2 to get the total number of constructive orderings 
	total *= 2_usize;

    println!("Total constructive orderings for N={} - {}", N, total);

}


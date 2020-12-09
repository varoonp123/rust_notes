use rand::{thread_rng, Rng};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::thread;
use std::time;
fn run_philosopher(philosopher_indx: usize, adj_forks: [Arc<AtomicBool>; 2]) {
    //Strategy: Odd indexed philosophers will pick up their left fork first when
    //it becomes available, then pick up the right fork when it becomes
    //available. Then eat for a while then put the left fork then the right fork
    //back on the table. Even indexed philosophers will do the same but starting
    //with their right fork. This prevents deadlocks.  The assertions sleeps,
    //and prints, are purely sanity checks. Does this actually need sequential
    //consistency?
    let starting_indx = philosopher_indx % 2;
    let first_fork = &adj_forks[starting_indx];
    let second_fork = &adj_forks[(starting_indx + 1) % 2];
    loop {
        while !first_fork.compare_and_swap(true, false, Ordering::SeqCst) {}
        assert_eq!(first_fork.load(Ordering::SeqCst), false);
        while !second_fork.compare_and_swap(true, false, Ordering::SeqCst) {}
        assert_eq!(first_fork.load(Ordering::SeqCst), false);
        assert_eq!(second_fork.load(Ordering::SeqCst), false);
        println!("Philosopher {} is eating!", philosopher_indx);
        thread::sleep(time::Duration::from_secs(1));
        println!("Philosopher {} Stopped eating!!", philosopher_indx);
        assert_eq!(first_fork.load(Ordering::SeqCst), false);
        assert_eq!(second_fork.load(Ordering::SeqCst), false);
        first_fork.compare_and_swap(false, true, Ordering::SeqCst);
        thread::sleep(time::Duration::from_secs(1));
        assert_eq!(second_fork.load(Ordering::SeqCst), false);
        second_fork.compare_and_swap(false, true, Ordering::SeqCst);
        thread::sleep(time::Duration::from_secs(1));
    }
}
fn main() {
    // ``true`` means the fork is on the table/is available to be taken by a
    // diner These must be Arcs because the borrow checker cannot rule out the
    // possibility that pure AtomicBools will only live through the main
    // function.  There are different threads so any references may live longer
    // than this function.
    let forks: Vec<Arc<AtomicBool>> = vec![
        Arc::new(AtomicBool::new(true)),
        Arc::new(AtomicBool::new(true)),
        Arc::new(AtomicBool::new(true)),
        Arc::new(AtomicBool::new(true)),
        Arc::new(AtomicBool::new(true)),
    ];
    let mut rng = thread_rng();
    // One could do something interesting with these threads such as adding a
    // new philosopher if a thread dies.
    let mut threads = vec![];
    for indx in 0..5 {
        let fuzz: u64 = rng.gen_range(0, 1500);
        thread::sleep(time::Duration::from_millis(fuzz));
        // Calling `.clone()` increases the ref count of each atomic bool, while
        // pointing to the same underlying heap structure.
        let fks: [Arc<AtomicBool>; 2] = [
            forks[indx % 5].clone(),
            forks[(indx + 1) % 5].clone(),
        ];
        // Note that if ``move`` is removed, this will not compile. By default,
        // the closure will try to borrow these values, but the identities
        // that hold these references in the other thread may live longer than
        // the underlying usize (this iteration of the loop) or the forks (the
        // scope of this function). This is why ``run_philosopher`` cannot take
        // these variables by reference; the threads must own the `usize` (which
        // is trivial since that is a [`std::mem::Copy`]able type), and the
        // array of arcs.  
        threads.push(thread::spawn(move || run_philosopher(indx.clone(), fks)));
    }
    // Without joining a thread, the threads will be created then the program
    // will end before anything interesting happens.
    threads.pop().unwrap().join().unwrap();
}

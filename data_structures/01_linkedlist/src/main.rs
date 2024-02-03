use std::thread;
use linkedlist::LinkedList;



fn main () {
    // Spawn a new thread with an increased stack size
    let handle = thread::Builder::new()
        // .stack_size(1 * 1024 * 1024) // Set stack size to 1 MB (adjust as needed)
        .stack_size(20 * 1024 * 1024) // Set stack size to 1 MB (adjust as needed)
        .spawn(|| {
            let mut list = LinkedList::default();
            for i in 0..10_000_000 {
                list.push_front(i);
            }
            // list.print_list();
            println!("{}", list);
        })
        .expect("Failed to spawn thread");

    // Wait for the spawned thread to finish
    handle.join().expect("Failed to join thread");

    // let mut list = LinkedList::default();
    // for i in 0 .. 10_000_000 {
    //    list.push_front(i);
    // }
    // list.print_list();
    // println("{}", list);
}

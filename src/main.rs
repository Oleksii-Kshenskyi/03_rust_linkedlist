// Basic requirements for this list:
// + This is a singly linked list, meaning the only pointer is to the head of the list
// + The list should implement Rust's Display trait for println! macro to be able to display it
// + It should have foreach (like map, apply a function to every item in the list)
// + It should have push to head
// + It should have pop from head

mod llist;
use llist::List;

fn main() {
    let mut trylist: List<u64> = List::new();
    trylist.push(22);
    trylist.push(22644);
    trylist.push(161615451313);
    trylist.push(1414);

    println!("{}", trylist);

    trylist.pop();
    trylist.pop();
    trylist.pop();
    println!("{}", trylist);
    trylist.pop();
    let noneval = trylist.pop();
    assert_eq!(noneval, None);
    println!("{}", trylist);

    trylist.push(13);
    trylist.push(44);
    trylist.push(123);
    let val = trylist.pop().unwrap();
    assert_eq!(val, 123);

    trylist.for_each(|x| *x += 1);
    println!("{}", trylist);
}

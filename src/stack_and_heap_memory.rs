use std::mem;

struct Point {
    x: f64,
    y: f64
}

fn origin() -> Point {
    Point{x: 0.0, y: 0.0}
}

pub fn stack_and_heap_memory() {
    let x = 5; // Allocated in stack
    let y = Box::new(6); // value is stored in heap and address variable is created in stack

    // Points
    let p1 = origin(); // point in Stack
    let p2 = Box::new(origin()); // Point in Heap

    println!("p1 takes {} bytes", mem::size_of_val(&p1)); // will print 16 bytes
    println!("p2 takes {} bytes", mem::size_of_val(&p2)); // will print 8 bytes

    let p3 = *p2; // Value from p2 is copied and new point in Stack is created, REFERENCE IS NOT SHARED HERE
}
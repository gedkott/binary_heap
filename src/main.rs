use binary_heap::{BinaryHeap, SmartHeapMutatingPointer};

fn main() {
    let mut bh = BinaryHeap::new();
    for i in 0..10 {
        bh.push(i);
    }

    bh.peek_mut().map(|mut s: SmartHeapMutatingPointer<'_, _>| {
        *s += 10000;
    });

    println!("Binary Heap: {:?}", bh.heap().collect::<Vec<_>>());
}

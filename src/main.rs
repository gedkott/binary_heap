use binary_heap::{BinaryHeap};

#[derive(PartialOrd, Debug)]
struct Job {
    priority: i32,
    id: u32,
}

impl PartialEq for Job {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl From<u32> for Job {
    fn from(t: u32) -> Job {
        Job { priority: 0, id: t }
    }
}

fn main() {
    let mut bh = BinaryHeap::new();
    for i in 0..10 {
        bh.push(Job {
            priority: i * 10 + 1,
            id: i as u32,
        });
    }

    bh.delete(5.into());

    if let Some(s) = bh.peek_mut() {
        println!("Just looking {:?}", *s);
    };

    if let Some(mut s) = bh.peek_mut() {
        println!("Going to mutate {:?}", *s);
        (*s).priority -= 10000; // same as s.priority (auto deref)
    };

    println!("Binary Heap: {:?}", bh.heap().collect::<Vec<_>>());
}

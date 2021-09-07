fn main() {
    println!("Hello, world!");
}

struct BinaryHeap<T>
where
    T: PartialOrd,
{
    heap: Vec<T>,
}

impl<T> BinaryHeap<T>
where
    T: PartialOrd,
{
    pub fn new() -> Self {
        BinaryHeap { heap: vec![] }
    }

    pub fn heap(self) -> Vec<T> {
        self.heap
    }

    pub fn push(&mut self, value: T) {
        self.heap.push(value);
        self.swim();
    }

    pub fn peek(&self) -> Option<&T> {
        None
    }

    pub fn pop(&mut self) -> Option<T> {
        None
    }

    fn swim(&mut self) {
        // greatest value swims to top
        // let mut index_of_swimmer = self.heap.len() - 1;
        // let mut index_of_parent = (f64::floor((index_of_swimmer as f64 - 1.0)/2.0)) as usize;
        // loop {
        //     let swimmer_value = &self.heap[index_of_swimmer];
        //     let parent_value = &self.heap[index_of_parent];
        //     if swimmer_value > parent_value {
        //         // swap swimmer with parent and then do it again
        //         self.heap.swap(index_of_parent, index_of_swimmer);
        //         index_of_swimmer = index_of_parent;
        //         index_of_parent = (f64::floor((index_of_swimmer as f64 - 1.0)/2.0)) as usize;
        //     } else {
        //         break;
        //     }
        // }

        let mut index_of_swimmer = self.heap.len() - 1;
        let mut index_of_parent = (f64::floor((index_of_swimmer as f64 - 1.0) / 2.0)) as usize;
        let mut swimmer_value = &self.heap[index_of_swimmer];
        let mut parent_value = &self.heap[index_of_parent];
        while swimmer_value > parent_value {
            // swap swimmer with parent and then do it again
            self.heap.swap(index_of_parent, index_of_swimmer);
            index_of_swimmer = index_of_parent;
            index_of_parent = (f64::floor((index_of_swimmer as f64 - 1.0) / 2.0)) as usize;
            swimmer_value = &self.heap[index_of_swimmer];
            parent_value = &self.heap[index_of_parent];
        }
    }
}

#[test]
fn test_creation_and_pushing() {
    let mut bh = BinaryHeap::new();
    bh.push(1);
    bh.push(2);
    bh.push(5);
    bh.push(4);
    bh.push(3);
    assert_eq!(bh.heap().as_slice(), [5, 4, 2, 1, 3]);
}

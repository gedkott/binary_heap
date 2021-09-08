use std::ops::{Deref, DerefMut};

// I think the reason the std lib uses this smart pointer for peek mut
// is to avoid sifting if the value was derefed but not mutated, but that
// seems like a lot of overhead for avoiding a single sift down since the root
// should not need to sift down at all if the value was not mutated
pub struct SmartHeapMutatingPointer<'a, T: PartialOrd> {
    heap: &'a mut BinaryHeap<T>,
    needs_sifting: bool,
}

impl<T: PartialOrd> Deref for SmartHeapMutatingPointer<'_, T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        self.heap.peek().unwrap()
    }
}

impl<T: PartialOrd> DerefMut for SmartHeapMutatingPointer<'_, T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        // now that we are providing mutable access, we assume we
        // need to sift down the max value just in case its changed
        // to a value that lowers its value
        self.needs_sifting = true;
        self.heap.items.get_mut(0).unwrap()
    }
}

// when a SmartHeapMutatingPointer goes out of scope and is dropped,
// we will sift everything back to perfection. This should only happen
// if DerefMut.deref_mut was called on the smart pointer.
impl<T: PartialOrd> Drop for SmartHeapMutatingPointer<'_, T> {
    fn drop(&mut self) {
        if self.needs_sifting {
            self.heap.sift_down(0);
        }
    }
}

pub struct BinaryHeap<T>
where
    T: PartialOrd,
{
    items: Vec<T>,
}

impl<T> BinaryHeap<T>
where
    T: PartialOrd,
{
    pub fn new() -> Self {
        BinaryHeap { items: vec![] }
    }

    pub fn heap(&self) -> impl std::iter::Iterator<Item = &T> {
        self.items.iter()
    }

    pub fn push(&mut self, value: T) {
        self.items.push(value);
        self.sift_up();
    }

    pub fn peek(&self) -> Option<&T> {
        self.items.get(0)
    }

    pub fn peek_mut(&mut self) -> Option<SmartHeapMutatingPointer<T>> {
        if self.is_empty() {
            None
        } else {
            Some(SmartHeapMutatingPointer {
                heap: self,
                needs_sifting: false,
            })
        }
    }

    pub fn pop(&mut self) -> Option<T> {
        let len = self.items.len();
        self.items.swap(0, len - 1);
        let largest = self.items.pop();

        // now sift the topmost element down until its in the right place
        self.sift_down(0);

        largest
    }

    pub fn delete(&mut self, item: T) -> Option<T> {
        self.items
            .iter()
            .position(|t| item == *t)
            .and_then(|index| {
                let len = self.items.len();
                self.items.swap(index, len - 1);
                let deleted = self.items.pop();
                self.sift_down(index);
                deleted
            })
    }

    pub fn clear(&mut self) {
        self.items.clear()
    }

    pub fn len(&self) -> usize {
        self.items.len()
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    fn sift_up(&mut self) {
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

        let mut index_of_swimmer = self.items.len() - 1;
        let mut index_of_parent = (f64::floor((index_of_swimmer as f64 - 1.0) / 2.0)) as usize;
        let mut swimmer_value = &self.items[index_of_swimmer];
        let mut parent_value = &self.items[index_of_parent];
        while swimmer_value > parent_value {
            // swap swimmer with parent and then do it again
            self.items.swap(index_of_parent, index_of_swimmer);
            index_of_swimmer = index_of_parent;
            index_of_parent = (f64::floor((index_of_swimmer as f64 - 1.0) / 2.0)) as usize;
            swimmer_value = &self.items[index_of_swimmer];
            parent_value = &self.items[index_of_parent];
        }
    }

    fn sift_down(&mut self, start_index: usize) {
        let left_child_index = 2 * start_index + 1;
        let right_child_index = 2 * start_index + 2;

        let left_child_value = self.items.get(left_child_index);
        let right_child_value = self.items.get(right_child_index);

        let mut largest_value = self.items.get(start_index);
        let mut largest_index = start_index;

        if left_child_value.is_some() && left_child_value > largest_value {
            largest_index = left_child_index;
            largest_value = left_child_value;
        }

        if right_child_value.is_some() && right_child_value > largest_value {
            largest_index = right_child_index;
        }

        if largest_index != start_index {
            self.items.swap(start_index, largest_index);
            self.sift_down(largest_index)
        }
    }
}

#[test]
fn test_pushing_and_peeking() {
    let mut bh = BinaryHeap::new();
    bh.push(1);
    bh.push(2);
    bh.push(5);
    bh.push(4);
    bh.push(3);
    assert_eq!(bh.heap().collect::<Vec<&i32>>(), [&5, &4, &2, &1, &3]);

    /*
        5
       4 2
      1 3
    */
    assert_eq!(bh.peek(), Some(&5));
}

#[test]
fn test_popping() {
    let mut bh = BinaryHeap::new();
    bh.push(1);
    bh.push(2);
    bh.push(5);
    bh.push(4);
    bh.push(3);

    /*
        5
       1 2
    */

    /*
        5
       4 2
      1 3
    */

    let mut largest = bh.pop();
    assert_eq!(largest, Some(5));
    assert_eq!(bh.heap().collect::<Vec<&i32>>(), [&4, &3, &2, &1]);

    /*
        4
       3 2
      1
    */

    largest = bh.pop();
    assert_eq!(largest, Some(4));
    assert_eq!(bh.heap().collect::<Vec<&i32>>(), [&3, &1, &2]);

    /*
        3
       1 2
    */

    largest = bh.pop();
    assert_eq!(largest, Some(3));

    /*
        2
       1
    */

    largest = bh.pop();
    assert_eq!(largest, Some(2));
}

#[test]
fn test_popping_uses_largest_child() {
    /*
        5
       3 4
      1 2
    */
    let mut bh = BinaryHeap::new();

    bh.push(5);
    bh.push(1);
    bh.push(4);
    bh.push(2);
    bh.push(3);

    let mut largest = bh.pop();
    assert_eq!(largest, Some(5));
    assert_eq!(bh.heap().collect::<Vec<&i32>>(), [&4, &3, &2, &1]);

    /*
        4
       3 2
      1
    */

    largest = bh.pop();
    assert_eq!(largest, Some(4));
    assert_eq!(bh.heap().collect::<Vec<&i32>>(), [&3, &1, &2]);

    /*
        3
       1 2
    */

    largest = bh.pop();
    assert_eq!(largest, Some(3));

    /*
        2
       1
    */

    largest = bh.pop();
    assert_eq!(largest, Some(2));
}

#[test]
fn test_deleting() {
    /*
        5
       3 4
      2 1
    */
    let mut bh = BinaryHeap::new();

    bh.push(5);
    bh.push(3);
    bh.push(4);
    bh.push(2);
    bh.push(1);

    let deleted = bh.delete(3);
    assert_eq!(deleted, Some(3));

    /*
        5
       1 4
      2

      ---->

        5
       2 4
      1
    */

    assert_eq!(bh.heap().collect::<Vec<&i32>>(), [&5, &2, &4, &1]);
}

#[test]
fn test_peek_mut() {
    let mut heap = BinaryHeap::new();
    heap.push(1);
    heap.push(5);
    heap.push(2);
    {
        let mut val = heap.peek_mut().unwrap();
        *val = 0;
    }
    assert_eq!(heap.peek(), Some(&2));
}

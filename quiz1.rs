use std::fmt;

struct MinHeap {
    heap: Vec<i32>,
}

impl MinHeap {
    fn new(mut values: Vec<i32>) -> Self {
        let size = values.len();
        let mut heap = MinHeap { heap: values };
        println!("Initial array: {:?}", heap.heap);
        
        for i in (0..size / 2).rev() {
            heap.min_heapify(i);
        }
        
        println!("Final Min Heap: {:?}", heap.heap);
        heap
    }

    fn min_heapify(&mut self, i: usize) {
        let size = self.heap.len();
        let mut smallest = i;
        let left = 2 * i + 1;
        let right = 2 * i + 2;

        if left < size && self.heap[left] < self.heap[smallest] {
            smallest = left;
        }
        if right < size && self.heap[right] < self.heap[smallest] {
            smallest = right;
        }
        
        if smallest != i {
            self.heap.swap(i, smallest);
            println!("Heapify step: {:?}", self.heap);
            self.min_heapify(smallest);
        }
    }
}

fn main() {
    let values = vec![100, 5, 9, 6, 8, 20, 10, 12, 18, 9];
    let _heap = MinHeap::new(values);
}
